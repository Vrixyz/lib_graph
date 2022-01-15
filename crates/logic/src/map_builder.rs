use std::{collections::HashMap, time::Duration};

use bevy::{ecs::component::TableStorage, prelude::*};
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
pub struct MapBuilder {
    clutters: HashMap<RoomId, RoomClutter>,
}
impl Component for MapBuilder {
    type Storage = TableStorage;
}

pub fn setup(app: &mut App) {
    app.add_startup_system(setup_map);
    app.add_startup_system_to_stage(StartupStage::PostStartup, create_level);
    //app.add_system(update_map);
    app.add_system(make_rooms_selectable);
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

fn create_level(
    mut commands: Commands,
    mut random: ResMut<RandomDeterministic>,
    mut maps: Query<(&mut Map, &mut MapBuilder)>,
) {
    for (mut map, mut builder) in maps.iter_mut() {
        for _ in 0..25 {
            create_room(map.as_mut(), builder.as_mut(), &mut random);
        }
    }
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
        create_room(map.as_mut(), builder.as_mut(), &mut random);
    }
}

fn create_room(
    mut map: &mut Map,
    mut builder: &mut MapBuilder,
    random: &mut ResMut<RandomDeterministic>,
) {
    if map.0.len() >= 20 {
        //continue;
    }
    for _ in 0..5 {
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
        match map.0.add(from_room, 1, &mut random.random, 10) {
            Ok(room_id) => {
                // TODO: add connections to that room (excluding from_room because add() did it already)
            }
            Err(_) => {
                builder
                    .clutters
                    .entry(from_room)
                    .or_insert_with(RoomClutter::default)
                    .nb_gen_tries += 1;
            }
        }
    }
}

fn make_rooms_selectable(mut commands: Commands, q_new_rooms: Query<Entity, Added<RoomEntity>>) {
    for e in q_new_rooms.iter() {
        let room_size = 30f32;
        let margin = 100f32;
        commands
            .entity(e)
            .insert(Selectable::new(room_size + margin, false, false));
    }
}
