use bevy::{
    ecs::component::TableStorage, math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle,
};
use map::RoomId;
use map_bevy::{Map, RoomEntity};
use shapes::ShapeMeshes;

use crate::RandomDeterministic;

const PLAYER_SPEED: f32 = 60f32;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_players);
        app.add_system(update_units_position);
        app.add_event::<EventPlayersSpawn>();
    }
}

pub struct EventPlayersSpawn;

pub struct Player;
impl Component for Player {
    type Storage = TableStorage;
}

pub struct Unit {
    pub room_id: RoomId,
    pub is_moving: bool,
}
impl Component for Unit {
    type Storage = TableStorage;
}

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

struct SpawnDef {
    players: Vec<RoomId>,
    points: Vec<RoomId>,
}

fn spawn_players(
    mut commands: Commands,
    shapes: Res<ShapeMeshes>,
    mut random: ResMut<RandomDeterministic>,
    mut maps: Query<&Map>,
    rooms: Query<(Entity, &RoomEntity)>,
    mut events: EventReader<EventPlayersSpawn>,
) {
    for my_event in events.iter() {
        let mut map = maps.single_mut();

        let spawn_def = SpawnDef {
            players: vec![
                *map.0.iter().next().unwrap().0,
                *map.0.iter().last().unwrap().0,
            ],
            points: vec![
                *map.0.iter().nth(2).unwrap().0,
                *map.0.iter().nth(3).unwrap().0,
                *map.0.iter().nth(4).unwrap().0,
                *map.0.iter().nth(5).unwrap().0,
                *map.0.iter().nth(6).unwrap().0,
            ],
        };

        let first = spawn_def.players[0];
        spawn_unit(
            &rooms,
            &mut commands,
            first,
            create_player_bundle(&shapes, map.0.rooms[&first].position),
            true,
        );
        if let Some(last) = map.0.iter().last() {
            spawn_unit(
                &rooms,
                &mut commands,
                *spawn_def.players.iter().last().unwrap(),
                create_ai_bundle(&shapes, last.1.position),
                false,
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

fn update_units_position(
    time: Res<Time>,
    mut maps: Query<&Map>,
    mut units: Query<(&mut Transform, &mut Unit)>,
) {
    let mut map = maps.single_mut();
    for (mut t, mut u) in units.iter_mut() {
        if u.is_moving {
            let target = &map.0.rooms[&u.room_id];
            let target: Vec2 = target.position.into();
            let to_target: Vec2 = target - t.translation.xy();
            let actual_move = to_target
                .clamp_length_max(PLAYER_SPEED * time.delta_seconds())
                .extend(0f32);
            t.translation += actual_move;

            if t.translation.xy() == target {
                u.is_moving = false;
            }
        }
    }
}
