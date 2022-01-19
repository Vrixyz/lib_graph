use bevy::prelude::*;

pub struct CameraPanPlugin;

impl Plugin for CameraPanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraPan {
            camera: None,
            ..Default::default()
        });
        app.add_system(systems::camera_pan)
            .add_system(systems::camera_prepan)
            .add_system(systems::camera_zoom);
    }
}

#[derive(PartialEq)]
pub enum PanState {
    NotPanning,
    RecordPrepan,
    Panning,
}

impl Default for PanState {
    fn default() -> Self {
        Self::NotPanning
    }
}

#[derive(Default, Component)]
pub struct CameraPan {
    pub camera: Option<Entity>,
    pub state: PanState,
    pub prepan_offset: Vec3,
}

mod systems {
    use bevy::{input::mouse::MouseMotion, input::mouse::MouseWheel, math::Vec3, prelude::*};

    use crate::PanState;

    use super::CameraPan;

    pub fn camera_prepan(
        mut camera_pan: ResMut<CameraPan>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut mouse_motion: EventReader<MouseMotion>,
        mut query: Query<&mut Transform>,
    ) {
        if camera_pan.state == PanState::Panning {
            return;
        }
        if mouse_button_input.just_pressed(MouseButton::Left) {
            camera_pan.prepan_offset = Vec3::default();
            camera_pan.state = PanState::RecordPrepan;
        }
        if (camera_pan.state != PanState::RecordPrepan) {
            return;
        }
        if mouse_button_input.pressed(MouseButton::Left) {
            if let Some(offset) = mouse_motion.iter().last() {
                if camera_pan.camera.is_some() {
                    let mut camera = query
                        .get_component_mut::<Transform>(camera_pan.camera.unwrap())
                        .unwrap();
                    camera_pan.prepan_offset -= Vec3::new(offset.delta.x, -offset.delta.y, 0.0);
                    if camera_pan.prepan_offset.length() > 10f32 {
                        camera_pan.state = PanState::Panning;
                    }
                }
            }
        } else {
            camera_pan.state = PanState::NotPanning;
        }
    }

    pub fn camera_pan(
        mut camera_pan: ResMut<CameraPan>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut mouse_motion: EventReader<MouseMotion>,
        mut query: Query<(&mut Transform, &OrthographicProjection)>,
    ) {
        if camera_pan.state != PanState::Panning {
            return;
        }
        if mouse_button_input.pressed(MouseButton::Left) {
            for offset in mouse_motion.iter() {
                if camera_pan.camera.is_some() {
                    let (mut camera, orthographic_projection) =
                        query.get_mut(camera_pan.camera.unwrap()).unwrap();
                    let offset = Vec3::new(
                        offset.delta.x * orthographic_projection.scale,
                        -offset.delta.y * orthographic_projection.scale,
                        0.0,
                    );
                    camera.translation -= offset;
                }
            }
        } else {
            camera_pan.state = PanState::NotPanning;
        }
    }
    pub fn camera_zoom(
        camera_pan: Res<CameraPan>,
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<&mut OrthographicProjection>,
    ) {
        if camera_pan.camera.is_none() {
            return;
        }
        use bevy::input::mouse::MouseScrollUnit;
        let mut offset = None;
        if let Some(ev) = scroll_evr.iter().last() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    offset = Some(ev.y * -0.1f32);
                }
                MouseScrollUnit::Pixel => {
                    offset = Some(ev.y * -0.1f32);
                }
            }
        }
        if let Some(offset) = offset {
            // FIXME: Scale is very ugly: zooming out messes up with cursor movement, and we can't zoom in.
            let mut orthographic_projection = query.get_mut(camera_pan.camera.unwrap()).unwrap();
            orthographic_projection.scale += offset;
            orthographic_projection.scale = f32::max(orthographic_projection.scale, 0.1f32);
        }
    }
}
