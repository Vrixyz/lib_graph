use std::collections::HashMap;

use poisson::Poisson;
use rand::Rng;
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, Default, Clone, Copy, Debug)]
pub struct RoomId(usize);

#[derive(Debug)]
pub struct Room<T: Sized> {
    pub connections: Vec<RoomId>,
    pub position: (f32, f32),
    pub data: T,
}

#[derive(Debug, Default)]
pub struct Map<T: Sized> {
    pub rooms: HashMap<RoomId, Room<T>>,
    room_id_provider: RoomId,
}

#[derive(Error, Debug)]
pub enum ErrorAdd {
    #[error("Did not find `from` RoomId {0:?}")]
    InexistantFromRoomId(RoomId),
    #[error("Did not find enough place around `from` RoomId {0:?}")]
    NoPlaceFound(RoomId),
}

impl<T: Sized> Map<T> {
    pub fn add(
        &mut self,
        from: RoomId,
        data: T,
        rng: &mut impl Rng,
        nb_tries: u32,
    ) -> Result<RoomId, ErrorAdd> {
        let positions = self.get_positions();
        let ref_point = self.rooms[&from].position;
        let poisson = Poisson::new();

        let pos = get_position_around(nb_tries, poisson, positions, vec![ref_point], rng);
        if let Some(new_position) = pos {
            let room_id = self.create_raw(data, new_position, vec![from]);

            self.connect(from, room_id)?;
            return Ok(room_id);
        }
        Err(ErrorAdd::NoPlaceFound(from))
    }
    pub fn remove(&mut self, id: RoomId) {
        self.rooms.remove(&id);
    }

    fn get_positions(&mut self) -> Vec<(f32, f32)> {
        self.rooms
            .values()
            .into_iter()
            .map(|r| r.position)
            .collect()
    }

    fn connect(&mut self, from: RoomId, to: RoomId) -> Result<(), ErrorAdd> {
        match self.rooms.entry(from) {
            std::collections::hash_map::Entry::Occupied(mut room) => {
                room.get_mut().connections.push(to);
                Ok(())
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                Err(ErrorAdd::InexistantFromRoomId(from))
            }
        }
    }

    pub fn create_raw(
        &mut self,
        data: T,
        position: (f32, f32),
        connections: Vec<RoomId>,
    ) -> RoomId {
        let room_id_to_create = self.room_id_provider;
        let new_room = Room {
            connections,
            position,
            data,
        };
        self.rooms.insert(room_id_to_create, new_room);
        self.room_id_provider.0 += 1;
        room_id_to_create
    }

    pub fn len(&self) -> usize {
        self.rooms.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<RoomId, Room<T>> {
        self.rooms.iter_mut()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<RoomId, Room<T>> {
        self.rooms.iter()
    }
}

pub fn get_position_around(
    nb_tries: u32,
    poisson: Poisson,
    positions: Vec<(f32, f32)>,
    ref_points: Vec<(f32, f32)>,
    rng: &mut impl Rng,
) -> Option<(f32, f32)> {
    for ref_point in ref_points.iter() {
        if let Some(new_position) =
            poisson.compute_new_position(&positions, ref_point, 40f32, nb_tries, rng)
        {
            return Some(new_position);
        }
    }
    None
}
