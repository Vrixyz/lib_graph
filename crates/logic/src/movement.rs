use bevy::{ecs::component::TableStorage, prelude::*, sprite::MaterialMesh2dBundle};
use map::RoomId;
use map_bevy::{Map, RoomEntity};
use shapes::ShapeMeshes;

use crate::{map_builder::MapBuilder, RandomDeterministic};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage_after(
            StartupStage::PostStartup,
            "player_setup",
            SystemStage::single_threaded(),
        );
        app.add_startup_system_to_stage("player_setup", spawn_players);
    }
}

struct PlayerUnit {
    pub room_id: RoomId,
}
impl Component for PlayerUnit {
    type Storage = TableStorage;
}

struct PlayerGraphics {
    pub mesh_bundle: MaterialMesh2dBundle<shapes::ColorMaterial>,
}
fn create_room_bundle(shapes: &Res<ShapeMeshes>, pos: (f32, f32)) -> PlayerGraphics {
    let mut transform = Transform::from_xyz(pos.0, pos.1, 15.0);
    transform.scale = Vec3::ONE * 7.5;
    let mesh = MaterialMesh2dBundle {
        mesh: shapes.quad2x2.clone().into(),
        material: shapes.mat_orange.clone(),
        transform,
        ..Default::default()
    };
    PlayerGraphics { mesh_bundle: mesh }
}
fn spawn_players(
    mut commands: Commands,
    shapes: Res<ShapeMeshes>,
    mut random: ResMut<RandomDeterministic>,
    mut maps: Query<(&Map, &MapBuilder)>,
    rooms: Query<(Entity, &RoomEntity)>,
) {
    dbg!("setup");
    let (mut map, mut builder) = maps.single_mut();
    if let Some(first) = map.0.iter().next() {
        dbg!("first room: ", first);
        if let Some(room) = rooms.iter().find(|(e, r)| dbg!(r.room_id) == *first.0) {
            dbg!("new player");
            commands
                .spawn()
                .insert(PlayerUnit { room_id: *first.0 })
                .insert_bundle(create_room_bundle(&shapes, first.1.position).mesh_bundle);
        }
    }
}
