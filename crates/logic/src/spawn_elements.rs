use bevy::{ecs::component::TableStorage, prelude::*, sprite::MaterialMesh2dBundle};
use map::RoomId;
use map_bevy::{Map, RoomEntity};
use shapes::ShapeMeshes;

use crate::GameState;
use crate::{in_game::RandomDeterministic, movement::Unit, pickups::Pickup};

use crate::movement::EventPlayersSpawn;

#[derive(Component)]
pub struct Player;

struct UnitGraphics {
    pub mesh_bundle: MaterialMesh2dBundle<shapes::ColorMaterial>,
}
fn create_player_bundle(shapes: &Res<ShapeMeshes>, pos: (f32, f32)) -> UnitGraphics {
    let mut transform = Transform::from_xyz(pos.0, pos.1, 15.0);
    transform.scale = Vec3::ONE * 7.5;
    let mesh = MaterialMesh2dBundle {
        mesh: shapes.quad2x2.clone().into(),
        material: shapes.mat_white.clone(),
        transform,
        ..Default::default()
    };
    UnitGraphics { mesh_bundle: mesh }
}
fn create_ai_bundle(shapes: &Res<ShapeMeshes>, pos: (f32, f32)) -> UnitGraphics {
    let mut transform = Transform::from_xyz(pos.0, pos.1, 15.0);
    transform.scale = Vec3::ONE * 7.5;
    let mesh = MaterialMesh2dBundle {
        mesh: shapes.quad2x2.clone().into(),
        material: shapes.mat_gray.clone(),
        transform,
        ..Default::default()
    };
    UnitGraphics { mesh_bundle: mesh }
}
fn create_point_bundle(shapes: &Res<ShapeMeshes>, pos: (f32, f32)) -> UnitGraphics {
    let mut transform = Transform::from_xyz(pos.0, pos.1, 14.0);
    transform.scale = Vec3::ONE * 5.5;
    let mesh = MaterialMesh2dBundle {
        mesh: shapes.quad2x2.clone().into(),
        material: shapes.mat_fuchsia.clone(),
        transform,
        ..Default::default()
    };
    UnitGraphics { mesh_bundle: mesh }
}

struct SpawnDef {
    players: Vec<RoomId>,
    points: Vec<RoomId>,
}
#[derive(Default)]
pub struct SpawnCounter {
    counter: i32,
}

pub fn spawn_elements(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    shapes: Res<ShapeMeshes>,
    mut random: ResMut<RandomDeterministic>,
    mut maps: Query<&Map>,
    rooms: Query<(Entity, &RoomEntity)>,
    mut events: EventReader<EventPlayersSpawn>,
    mut counter: Local<SpawnCounter>,
) {
    if counter.counter == 0 && events.iter().next().is_some() {
        // Delay 1 frame
        counter.counter += 2;
        return;
    }
    if counter.counter == 0 {
        return;
    }
    counter.counter = 0;
    let mut map = maps.get_single_mut();

    if map.is_err() {
        return;
    }
    let mut map = map.unwrap();
    if map.0.is_empty() {
        return;
    }
    game_state.set(GameState::Playing);
    dbg!("spawning");
    let spawn_def = SpawnDef {
        players: vec![
            *map.0.iter().next().unwrap().0,
            *map.0.iter().nth(2).unwrap().0,
            *map.0.iter().last().unwrap().0,
        ],
        points: vec![
            *map.0.iter().nth(3).unwrap().0,
            *map.0.iter().nth(4).unwrap().0,
            *map.0.iter().nth(5).unwrap().0,
            *map.0.iter().nth(6).unwrap().0,
        ],
    };

    spawn_def.players.iter().enumerate().for_each(|(i, r)| {
        if let Some(room) = map.0.rooms.get(&r) {
            let is_player = i == 0;

            spawn_unit(
                &rooms,
                &mut commands,
                *r,
                if is_player {
                    create_player_bundle(&shapes, room.position)
                } else {
                    create_ai_bundle(&shapes, room.position)
                },
                is_player,
            );
        }
    });
    for p in spawn_def.points {
        if let Some(room) = map.0.rooms.get(&p) {
            spawn_pickup(
                &rooms,
                &mut commands,
                p,
                create_point_bundle(&shapes, room.position),
            );
        }
    }
}

fn spawn_unit(
    rooms: &Query<(Entity, &RoomEntity)>,
    commands: &mut Commands,
    room_id: RoomId,
    graphics: UnitGraphics,
    is_player: bool,
) {
    if let Some(room) = rooms.iter().find(|(e, r)| r.room_id == room_id) {
        let mut u = commands.spawn();

        u.insert(Unit {
            room_id,
            is_moving: false,
        })
        .insert_bundle(graphics.mesh_bundle);
        if is_player {
            u.insert(Player);
        }
    }
}
fn spawn_pickup(
    rooms: &Query<(Entity, &RoomEntity)>,
    commands: &mut Commands,
    room_id: RoomId,
    graphics: UnitGraphics,
) {
    if let Some(room) = rooms.iter().find(|(e, r)| r.room_id == room_id) {
        let mut u = commands.spawn();
        u.insert(Pickup { room_id });
        u.insert_bundle(graphics.mesh_bundle);
    }
}
