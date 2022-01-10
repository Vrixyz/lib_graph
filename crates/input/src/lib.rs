use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(base_input.system());
        app.insert_resource(UserInputs::default());
    }
}

pub struct InputCamera;

#[derive(Default)]
pub struct UserInputs {
    pub world_pos: Option<Vec2>,
}

fn base_input(
    window: Res<Windows>,
    mut user_inputs: ResMut<UserInputs>,
    q_camera: Query<(&Transform, &OrthographicProjection), With<InputCamera>>,
) {
    let win = window.get_primary().expect("no primary window");
    if let Some(pos) = win.cursor_position() {
        let (camera_transform, projection) = q_camera.iter().next().unwrap();
        let size = Vec2::new(win.width() as f32, win.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = (pos - size / 2.0) * projection.scale;

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        user_inputs.world_pos = Some(Vec2::new(pos_wld.x, pos_wld.y));
    } else if user_inputs.world_pos.is_some() {
        user_inputs.world_pos = None;
    }
}
