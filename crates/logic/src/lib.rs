mod map_builder;
mod movement;
mod pickups;
mod spawn_elements;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::component::TableStorage,
    prelude::*,
};
use camera_pan::CameraPanPlugin;
use end_game::check_no_pickups;
use input::InputCamera;
use map_bevy::{DisplayMap, Map, MapPlugin};
use map_builder::MapBuilder;
use movement::MovementPlugin;
use pickups::unit_pickup_on_move_finished;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use selection::SelectionPlugin;
use spawn_elements::spawn_elements;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    App::new().add_plugin(LogicPlugin).run();
}

struct LogicPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    NotPlaying,
    LoadingBasic,
    LoadingMapSetup,
    LoadingMapRooms,
    LoadingSpawns,
    Playing,
}

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());

        app.add_plugin(MapPlugin);
        app.add_plugin(CameraPanPlugin);
        app.add_plugin(SelectionPlugin);
        app.add_plugin(MovementPlugin);
        app.add_state(GameState::LoadingBasic);

        app.insert_resource(in_game::RandomDeterministic::default());

        app.add_system(in_game::setup_camera);
        //app.add_system(expand_selected_rooms);
        app.add_system(in_game::cleanup_exit_playing);

        app.add_system(in_game::move_to_selected_rooms);
        app.add_system_to_stage(CoreStage::PostUpdate, spawn_elements);
        //app.add_system(spawn_elements);
        app.add_system(unit_pickup_on_move_finished);
        app.add_system_to_stage(CoreStage::PostUpdate, check_no_pickups);

        map_builder::setup(app);
    }
}

pub mod in_game {
    use crate::GameState;

    use super::map_builder::MapBuilder;
    use super::movement::EventPlayersSpawn;
    use super::movement::Unit;
    use super::spawn_elements::Player;
    use bevy::prelude::*;
    use camera_pan::CameraPan;
    use input::InputCamera;
    use map_bevy::Map;
    use map_bevy::RoomEntity;
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use selection::Selectable;

    // Resource
    pub struct RandomDeterministic {
        pub(crate) random: ChaCha20Rng,
        pub(crate) seed: u64,
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

    #[derive(Component)]
    struct MainCamera;

    pub(crate) fn setup_camera(
        mut commands: Commands,
        mut game_state: ResMut<State<GameState>>,
        mut camera_pan: ResMut<CameraPan>,
    ) {
        if game_state.current() != &GameState::LoadingBasic {
            return;
        }
        let mut cameraBundle = OrthographicCameraBundle::new_2d();
        cameraBundle.orthographic_projection.scale = 0.3;
        let entity = commands
            .spawn_bundle(cameraBundle)
            .insert(InputCamera)
            .insert(MainCamera)
            .id();
        camera_pan.camera = Some(entity);
        game_state.set(dbg!(GameState::LoadingMapSetup));
    }

    pub(crate) fn destroy_selected_rooms(
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

    pub fn cleanup_exit_playing(
        mut commands: Commands,
        mut game_state: ResMut<State<GameState>>,
        mut query: Query<Entity>,
    ) {
        if !game_state.is_changed() || game_state.current() != &GameState::NotPlaying {
            return;
        }
        dbg!("cleanup");
        game_state.set(GameState::LoadingBasic);
        for e in query.iter() {
            commands.entity(e).despawn();
        }
    }

    pub(crate) fn expand_selected_rooms(
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

    pub(crate) fn move_to_selected_rooms(
        mut commands: Commands,
        q_selected_rooms: Query<(Entity, &RoomEntity, &Selectable), With<RoomEntity>>,
        mut maps: Query<&mut Map>,
        mut players: Query<&mut Unit, With<Player>>,
    ) {
        for (e, id, s) in q_selected_rooms.iter() {
            if s.is_selected {
                let mut player = players.single_mut();
                if player.is_moving {
                    break;
                }
                if id.room_id == player.room_id {
                    break;
                }
                if let Ok(map) = maps.get_single() {
                    let current_room = &map.0.rooms[&player.room_id];
                    if !current_room.connections.contains(&id.room_id) {
                        break;
                    }
                    player.room_id = id.room_id;
                    player.is_moving = true;
                }
            }
        }
    }
}

mod end_game {
    use bevy::prelude::*;
    use map_bevy::Map;

    use crate::{movement::UnitFinishedMove, pickups::Pickup, GameState};

    pub fn check_no_pickups(
        mut commands: Commands,
        mut game_state: ResMut<State<GameState>>,
        mut pickups: Query<(Entity, &Pickup)>,
        mut map: Query<(&Map)>,
        mut event_unit_finished_move: EventReader<UnitFinishedMove>,
    ) {
        if event_unit_finished_move.iter().count() > 0 {
            if pickups.iter().count() == 0 {
                dbg!("Should restart Game.");
                game_state.set(dbg!(GameState::LoadingBasic));
            }
        }
    }
}
