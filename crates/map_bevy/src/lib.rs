use bevy::{ecs::component::TableStorage, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::{
    plugin::ShapePlugin,
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
};
use map::{Room, RoomId};
use shapes::*;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugin(ShapesPlugin);

        app.add_plugin(ShapePlugin);
        /*
        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        app.add_plugin(EguiPlugin)
            .add_plugin(MapGraphPlugin)
            .add_state(AppState::Menu)
            .add_system(ui_menu.system())
            .add_system(game_menu.system());*/

        app.add_system_to_stage(CoreStage::PreUpdate, update_map_display);
        app.add_system_to_stage(CoreStage::PreUpdate, connections::update_map_connections);
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
            if let Some(entity) = display.get_entity(*id) {
                match displays.get(entity) {
                    Ok(e) => {
                        // TODO: check if needs to update ?
                    }
                    Err(_) => {
                        to_remove.push(*id);
                        continue;
                    }
                }
            } else {
                // Create new room entity
                let ent = commands.spawn().insert(RoomEntity { room_id: *id }).id();
                display.add(*id, ent);
                let graphic_update = create_room_bundle(&shapes, room.position);
                commands
                    .entity(ent)
                    .insert_bundle(graphic_update.mesh_bundle);

                // Create new connection entities
                for c in room.connections.iter() {
                    // TODO: only add visible connections if id is from > to (to avoid duplicate connections)
                    // TODO: later, handle one way connections

                    let target = map.0.rooms[c].position;
                    let connection_def = (c, id);

                    let start = Vec2::new(room.position.0, room.position.1);
                    let end = Vec2::new(target.0, target.1);

                    let line = bevy_prototype_lyon::shapes::Line(start, end);

                    let mut builder = GeometryBuilder::new().add(&line);

                    commands.spawn_bundle(builder.build(
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::ORANGE_RED),
                            outline_mode: StrokeMode::new(Color::ORANGE_RED, 10.0),
                        },
                        Transform::default(),
                    ));
                    // TODO: add connections handle to remove them later
                    //connections.add(connection_def, connection_entity);
                }
            }
        }

        for r in to_remove {
            display.remove(r);
            // TODO: remove all connections containing that
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

mod connections {
    use bevy::prelude::*;
    use map::RoomId;

    #[derive(Default, Component)]
    pub struct DisplayConnections {
        pub entities: Vec<Entity>,
        pub ids: Vec<(RoomId, RoomId)>,
    }

    impl DisplayConnections {
        pub fn get_entity(&self, mut id: (RoomId, RoomId)) -> Option<Entity> {
            if id.0 < id.1 {
                id = (id.1, id.0);
            }
            for i in 0..self.ids.len() {
                if self.ids[i] == id {
                    return Some(self.entities[i]);
                }
            }
            None
        }
        pub fn get_entities(&self, mut id: RoomId) -> Vec<Entity> {
            let mut res = vec![];
            for i in 0..self.ids.len() {
                if self.ids[i].0 == id || self.ids[i].1 == id {
                    res.push(self.entities[i]);
                }
            }
            res
        }

        fn add(&mut self, id: (RoomId, RoomId), ent: Entity) {
            self.ids.push(id);
            self.entities.push(ent);
        }
        fn remove(&mut self, mut ids: (RoomId, RoomId)) {
            if ids.0 < ids.1 {
                ids = (ids.1, ids.0);
            }
            if let Some(index) = self.ids.iter().position(|id| *id == ids) {
                self.ids.remove(index);
                self.entities.remove(index);
            }
        }
    }

    pub fn update_map_connections() {}
}
