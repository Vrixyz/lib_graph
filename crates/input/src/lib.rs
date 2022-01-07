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

pub struct DragInput {
    pub new_click: Vec2,
    pub previous_click: Vec2,
}

impl DragInput {
    fn new(new_click: Vec2, previous_click: Vec2) -> Self {
        Self {
            new_click,
            previous_click,
        }
    }
}

pub enum UserInput {
    Click(Vec2),
    Drag(DragInput),
}

#[derive(Default)]
pub struct UserInputs {
    pub click: Option<UserInput>,
}

fn base_input(
    window: Res<Windows>,
    mut user_inputs: ResMut<UserInputs>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_camera: Query<(&Transform, &OrthographicProjection), With<InputCamera>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        let win = window.get_primary().expect("no primary window");
        if let Some(pos) = win.cursor_position() {
            let (camera_transform, projection) = q_camera.iter().next().unwrap();
            let size = Vec2::new(win.width() as f32, win.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = (pos - size / 2.0) * projection.scale;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            if mouse_button_input.just_pressed(MouseButton::Left) {
                user_inputs.click = Some(UserInput::Click(Vec2::new(pos_wld.x, pos_wld.y)));
            } else {
                let last_click = match &user_inputs.click {
                    Some(UserInput::Click(previous_click)) => previous_click,
                    Some(UserInput::Drag(drag)) => &drag.previous_click,
                    None => return,
                };
                user_inputs.click = Some(UserInput::Drag(DragInput::new(
                    Vec2::new(pos_wld.x, pos_wld.y),
                    *last_click,
                )));
            }
        }
    } else {
        if user_inputs.click.is_some() {
            user_inputs.click = None;
        }
    }
}
