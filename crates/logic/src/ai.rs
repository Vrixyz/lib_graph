use bevy::prelude::*;
use map_bevy::Map;
use rand::Rng;

use crate::{in_game, movement::Unit};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(random_ai_move);
    }
}

#[derive(Component)]
pub struct Ai;

fn random_ai_move(
    mut maps: Query<&mut Map>,
    mut random: ResMut<in_game::RandomDeterministic>,
    mut ais: Query<&mut Unit, With<Ai>>,
) {
    for mut u in ais.iter_mut() {
        if u.moving_to.is_some() {
            return;
        }
        if let Ok(map) = maps.get_single() {
            let current_room = &map.0.rooms[&u.room_id];
            if current_room.connections.is_empty() {
                continue;
            }
            u.moving_to = Some(
                current_room.connections
                    [random.random.gen_range((0..current_room.connections.len()))],
            );
        }
    }
}
