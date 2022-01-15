mod map_builder;
mod movement;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::component::TableStorage,
    prelude::*,
};
use camera_pan::{CameraPan, CameraPanPlugin};
use input::InputCamera;
use map_bevy::{DisplayMap, Map, MapPlugin, RoomEntity};
use map_builder::MapBuilder;
use movement::MovementPlugin;
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use selection::{Selectable, SelectionPlugin};

pub fn run() {
    App::new().add_plugin(LogicPlugin).run();
}

struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());

        app.add_plugin(MapPlugin);
        app.add_plugin(CameraPanPlugin);
        app.add_plugin(SelectionPlugin);
        app.add_plugin(MovementPlugin);

        app.insert_resource(RandomDeterministic::default());

        app.add_startup_system(setup_camera);
        //app.add_system(expand_selected_rooms);
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

impl Component for MainCamera {
    type Storage = TableStorage;
}

fn setup_camera(mut commands: Commands, mut camera_pan: ResMut<CameraPan>) {
    /*let cameraBundle = PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };*/
    let mut cameraBundle = OrthographicCameraBundle::new_2d();
    cameraBundle.orthographic_projection.scale = 0.3;
    let entity = commands
        .spawn_bundle(cameraBundle)
        .insert(InputCamera)
        .insert(MainCamera)
        .id();
    camera_pan.camera = Some(entity);
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
