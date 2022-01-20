use bevy::prelude::*;
use map::RoomId;
use map_bevy::Map;

use crate::{
    movement::{Unit, UnitFinishedMove},
    GameState,
};

#[derive(Component)]
pub struct Pickup {
    pub room_id: RoomId,
}

pub fn unit_pickup_on_move_finished(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut pickups: Query<(Entity, &Pickup)>,
    mut map: Query<(&Map)>,
    mut event_unit_finished_move: EventReader<UnitFinishedMove>,
) {
    if game_state.current() != &GameState::Playing {
        return;
    }
    for event in event_unit_finished_move.iter() {
        let map = map.single();
        let room = &map.0.rooms[&event.arrived_at];
        let mut pickup_count_left = 0;
        for (entity, pickup) in pickups.iter() {
            if pickup.room_id == event.arrived_at {
                commands.entity(entity).despawn();
            } else {
                pickup_count_left += 1;
            }
        }
        if pickup_count_left == 0 {
            dbg!("no pikcups");
            game_state.set(GameState::NotPlaying);
            // TODO: Send event no pickups left -> respawn pickups or restart game
            break;
        }
    }
}
