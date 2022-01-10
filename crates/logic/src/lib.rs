use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use camera_pan::{CameraPan, CameraPanPlugin};
use input::InputCamera;
use map_bevy::{DisplayMap, Map, MapPlugin};
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use selection::SelectionPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run() {
    App::build().add_plugin(LogicPlugin).run();
}

struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());

        app.add_plugin(MapPlugin);
        app.add_plugin(CameraPanPlugin);
        app.add_plugin(SelectionPlugin);

        app.insert_resource(RandomDeterministic::default());

        app.add_startup_system(setup_camera.system());
        map_builder::setup(app);
    }
}

// Resource
pub struct RandomDeterministic {
    pub random: ChaCha20Rng,
    pub seed: u64,
}

impl Default for RandomDeterministic {
    fn default() -> Self {
        let seed = thread_rng().gen::<u64>();
        Self {
            random: ChaCha20Rng::seed_from_u64(seed),
            seed,
        }
    }
}

struct MainCamera;

fn setup_camera(mut commands: Commands, mut camera_pan: ResMut<CameraPan>) {
    // TODO: camera logic into its own plugin
    let mut cameraBundle = OrthographicCameraBundle::new_2d();
    cameraBundle.orthographic_projection.scale = 0.3;
    let entity = commands
        .spawn_bundle(cameraBundle)
        .insert(InputCamera)
        .insert(MainCamera)
        .id();
    camera_pan.camera = Some(entity);
}

mod map_builder {
    use std::{collections::HashMap, time::Duration};

    use bevy::prelude::*;
    use map::{Room, RoomId};
    use map_bevy::{DisplayMap, Map, RoomEntity};
    use rand::Rng;
    use selection::Selectable;

    use crate::RandomDeterministic;

    #[derive(Default)]
    struct RoomClutter {
        pub nb_gen_tries: u8,
    }
    #[derive(Default)]
    struct MapBuilder {
        pub clutters: HashMap<RoomId, RoomClutter>,
    }

    pub fn setup(app: &mut AppBuilder) {
        app.add_startup_system(setup_map.system());
        //app.add_system(update_map.system());
        app.add_system(make_rooms_selectable.system());
        app.add_system(expand_selected_rooms.system());
    }

    fn setup_map(mut commands: Commands, mut random: ResMut<RandomDeterministic>) {
        let mut map = Map::default();
        let mut room_id = map.0.create_raw(0, (0f32, 0f32), vec![]);

        commands
            .spawn()
            .insert(DisplayMap::default())
            .insert(MapBuilder::default())
            .insert(map);
    }

    fn update_map(
        mut commands: Commands,
        mut timer: Local<Timer>,
        time: Res<Time>,
        mut random: ResMut<RandomDeterministic>,
        mut maps: Query<(&mut Map, &mut MapBuilder)>,
    ) {
        if timer.duration() == Duration::default() {
            timer.set_duration(Duration::from_millis(50));
            timer.reset();
        }
        timer.tick(time.delta());
        if !timer.just_finished() {
            return;
        }
        timer.reset();
        for (mut map, mut builder) in maps.iter_mut() {
            if map.0.len() >= 20 {
                continue;
            }
            for _ in 0..15 {
                let mut filtered_rooms: Vec<(&RoomId, &Room<i32>)> = map
                    .0
                    .iter()
                    .filter(|r| match builder.clutters.get(r.0) {
                        Some(clutter) => clutter.nb_gen_tries <= 1,
                        None => true,
                    })
                    .collect();
                if filtered_rooms.is_empty() {
                    builder.clutters.clear();
                    filtered_rooms = map.0.iter().collect();
                    dbg!("no safe room left");
                }
                if filtered_rooms.is_empty() {
                    dbg!("no rooms left at all");
                    map.0.create_raw(
                        0,
                        (
                            random.random.gen_range(-1f32..=1f32) * 30f32,
                            random.random.gen_range(-1f32..=1f32) * 30f32,
                        ),
                        vec![],
                    );
                    break;
                }
                let random_index = random.random.gen_range(0..filtered_rooms.len());

                let (from_room, _) = filtered_rooms[random_index];
                let from_room = *from_room;
                if map.0.add(from_room, 1, &mut random.random, 15).is_err() {
                    builder
                        .clutters
                        .entry(from_room)
                        .or_insert_with(RoomClutter::default)
                        .nb_gen_tries += 1;
                } else {
                    break;
                }
            }
        }
    }

    fn make_rooms_selectable(
        mut commands: Commands,
        q_new_rooms: Query<Entity, Added<RoomEntity>>,
    ) {
        for e in q_new_rooms.iter() {
            let room_size = 30f32;
            let margin = 100f32;
            commands
                .entity(e)
                .insert(Selectable::new(room_size + margin, false, false));
        }
    }

    fn destroy_selected_rooms(
        mut commands: Commands,
        q_selected_rooms: Query<(Entity, &RoomEntity, &Selectable), With<RoomEntity>>,
        mut maps: Query<&mut Map>,
    ) {
        for (e, id, s) in q_selected_rooms.iter() {
            if s.is_hover {
                for mut m in maps.iter_mut() {
                    if m.0.len() <= 1 {
                        return;
                    }
                    m.0.remove(id.room_id);
                }
                commands.entity(e).despawn();
            }
        }
    }
    fn expand_selected_rooms(
        mut commands: Commands,
        mut random: ResMut<RandomDeterministic>,
        mut q_selected_rooms: Query<(Entity, &RoomEntity, &mut Selectable), With<RoomEntity>>,
        mut maps: Query<(&mut Map, &mut MapBuilder)>,
    ) {
        for (e, id, s) in q_selected_rooms.iter_mut() {
            if s.is_hover {
                let from_room = id.room_id;
                for (mut map, mut builder) in maps.iter_mut() {
                    for _ in 0..2 {
                        if map.0.add(from_room, 1, &mut random.random, 15).is_ok() {
                            return;
                        }
                    }
                }
            }
        }
    }
}
