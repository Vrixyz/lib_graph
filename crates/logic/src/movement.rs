use bevy::{
    ecs::component::TableStorage, math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle,
};
use map::RoomId;
use map_bevy::{Map, RoomEntity};
use shapes::ShapeMeshes;

use crate::in_game::RandomDeterministic;

const PLAYER_SPEED: f32 = 60f32;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_units_position);
        app.add_event::<EventPlayersSpawn>();
        app.add_event::<UnitFinishedMove>();
    }
}

pub struct EventPlayersSpawn;
pub struct UnitFinishedMove {
    pub entity: Entity,
    pub arrived_at: RoomId,
}

#[derive(Component)]
pub struct Unit {
    pub room_id: RoomId,
    pub moving_to: Option<RoomId>,
}

fn update_units_position(
    time: Res<Time>,
    mut maps: Query<&Map>,
    mut units: Query<(Entity, &mut Transform, &mut Unit)>,
    mut event_unit_finished_move: EventWriter<UnitFinishedMove>,
) {
    let mut map = maps.get_single_mut();
    if map.is_err() {
        return;
    }
    let mut map = map.unwrap();
    for (e, mut t, mut u) in units.iter_mut() {
        if let Some(moving_to) = u.moving_to {
            let target = &map.0.rooms[&moving_to];
            let target: Vec2 = target.position.into();
            let to_target: Vec2 = target - t.translation.xy();
            let actual_move = to_target
                .clamp_length_max(PLAYER_SPEED * time.delta_seconds())
                .extend(0f32);
            t.translation += actual_move;

            if t.translation.xy() == target {
                u.room_id = moving_to;
                u.moving_to = None;
                event_unit_finished_move.send(UnitFinishedMove {
                    entity: e,
                    arrived_at: u.room_id,
                });
            }
        }
    }
}
