use bevy::{ecs::component::TableStorage, prelude::*, sprite::MaterialMesh2dBundle};
use map::{Room, RoomId};
use shapes::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugin(ShapesPlugin);
        /*
        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        app.add_plugin(EguiPlugin)
            .add_plugin(MapGraphPlugin)
            .add_state(AppState::Menu)
            .add_system(ui_menu.system())
            .add_system(game_menu.system());*/

        app.add_system_to_stage(CoreStage::PreUpdate, update_map_display);
    }
}

#[derive(Default)]
pub struct Map(pub map::Map<i32>);

impl Component for Map {
    type Storage = TableStorage;
}

#[derive(Default)]
pub struct DisplayMap {
    pub entities: Vec<Entity>,
    pub ids: Vec<RoomId>,
}
impl Component for DisplayMap {
    type Storage = TableStorage;
}

pub struct RoomEntity {
    pub room_id: map::RoomId,
}
impl Component for RoomEntity {
    type Storage = TableStorage;
}

pub struct RoomGraphUpdate {
    pub mesh_bundle: MaterialMesh2dBundle<shapes::ColorMaterial>,
}
fn create_room_bundle(shapes: &Res<ShapeMeshes>, pos: (f32, f32)) -> RoomGraphUpdate {
    let mut transform = Transform::from_xyz(pos.0, pos.1, 10.0);
    transform.scale = Vec3::ONE * 15.0;
    let mesh = MaterialMesh2dBundle {
        mesh: shapes.quad2x2.clone().into(),
        material: shapes.mat_green.clone(),
        transform,
        ..Default::default()
    };
    RoomGraphUpdate { mesh_bundle: mesh }
}

fn update_map_display(
    mut commands: Commands,
    shapes: Res<ShapeMeshes>,
    displays: Query<Entity, With<RoomEntity>>,
    mut maps: Query<(&mut Map, &mut DisplayMap), Changed<Map>>,
) {
    for (mut map, mut display) in maps.iter_mut() {
        let mut to_remove = vec![];
        for (id, room) in map.0.iter() {
            let entity = if let Some(entity) = display.get_entity(*id) {
                match displays.get(entity) {
                    Ok(e) => e,
                    Err(_) => {
                        to_remove.push(*id);
                        continue;
                    }
                }
            } else {
                // Create new room entities
                let ent = commands.spawn().insert(RoomEntity { room_id: *id }).id();
                display.add(*id, ent);
                ent
            };
            let graphic_update = create_room_bundle(&shapes, room.position);
            commands
                .entity(entity)
                .insert_bundle(graphic_update.mesh_bundle);
        }

        for r in to_remove {
            display.remove(r);
        }
    }
}
impl DisplayMap {
    pub fn get_entity(&self, id: RoomId) -> Option<Entity> {
        for i in 0..self.ids.len() {
            if self.ids[i] == id {
                return Some(self.entities[i]);
            }
        }
        None
    }

    fn add(&mut self, id: RoomId, ent: Entity) {
        self.ids.push(id);
        self.entities.push(ent);
    }
    fn remove(&mut self, room_id: RoomId) {
        if let Some(index) = self.ids.iter().position(|id| *id == room_id) {
            self.ids.remove(index);
            self.entities.remove(index);
        }
    }
}
