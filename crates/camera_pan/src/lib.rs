use bevy::prelude::*;
use input::InputPlugin;

pub struct CameraPanPlugin;

impl Plugin for CameraPanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InputPlugin);
        app.insert_resource(CameraPan {
            camera: None,
            ..Default::default()
        });
        app.add_system(systems::camera_pan.system())
            .add_system(systems::camera_zoom.system());
    }
}

#[derive(Default)]
pub struct CameraPan {
    pub camera: Option<Entity>,
}

mod systems {
    use bevy::{
        app::Events, input::mouse::MouseWheel, input::Input, math::Vec3, prelude::*,
        render::camera::CameraProjection, render::camera::OrthographicProjection,
    };
    use input::UserInputs;

    use super::CameraPan;

    pub fn camera_pan(
        mut camera_pan: ResMut<CameraPan>,
        user_inputs: Res<UserInputs>,
        mut query: Query<&mut Transform>,
    ) {
        if let Some(input::UserInput::Drag(click)) = &user_inputs.click {
            if camera_pan.camera.is_some() {
                let mut camera = query
                    .get_component_mut::<Transform>(camera_pan.camera.unwrap())
                    .unwrap();
                let offset = click.new_click - click.previous_click;
                camera.translation -= Vec3::new(offset.x, offset.y, 0.0);
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
