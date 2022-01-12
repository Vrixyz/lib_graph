use bevy::prelude::*;

pub struct CameraPanPlugin;

impl Plugin for CameraPanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraPan {
            camera: None,
            ..Default::default()
        });
        app.add_system(systems::camera_pan)
            .add_system(systems::camera_zoom);
    }
}

#[derive(Default, Component)]
pub struct CameraPan {
    pub camera: Option<Entity>,
}

mod systems {
    use bevy::{input::mouse::MouseMotion, input::mouse::MouseWheel, math::Vec3, prelude::*};

    use super::CameraPan;

    pub fn camera_pan(
        camera_pan: Res<CameraPan>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut mouse_motion: EventReader<MouseMotion>,
        mut query: Query<&mut Transform>,
    ) {
        if mouse_button_input.pressed(MouseButton::Left) {
            if let Some(offset) = mouse_motion.iter().last() {
                if camera_pan.camera.is_some() {
                    let mut camera = query
                        .get_component_mut::<Transform>(camera_pan.camera.unwrap())
                        .unwrap();
                    camera.translation -= Vec3::new(offset.delta.x, -offset.delta.y, 0.0);
                }
            }
        }
    }
    pub fn camera_zoom(
        camera_pan: Res<CameraPan>,
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<&mut Transform>,
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
            let mut transform = query
                .get_component_mut::<Transform>(camera_pan.camera.unwrap())
                .unwrap();
            transform.scale += Vec3::ONE * offset;
        }
    }
}
