use bevy::{math::Vec3Swizzles, prelude::*};
use input::{InputPlugin, UserInputs};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputPlugin);
        app.add_system(selection);
    }
}

#[derive(Component)]
pub struct Selectable {
    pub size: f32,
    pub is_selected: bool,
    pub is_hover: bool,
}

impl Selectable {
    pub fn new(size: f32, is_selected: bool, is_hover: bool) -> Self {
        Self {
            size,
            is_selected,
            is_hover,
        }
    }
}

fn selection(
    user_inputs: Res<UserInputs>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut q_selectable: Query<(Entity, &Transform, &mut Selectable)>,
) {
    if let Some(world_pos) = &user_inputs.world_pos {
        let mut closest = None;
        for mut s in q_selectable.iter_mut() {
            let mut distance = s.1.translation.xy().distance(*world_pos);
            if let Some((_, closest_distance)) = closest {
                if closest_distance < distance {
                    continue;
                }
            }
            if distance < s.2.size {
                closest = Some((s.0, distance));
            }
        }
        if let Some(closest) = closest {
            for mut s in q_selectable.iter_mut() {
                if s.2.is_selected {
                    s.2.is_selected = false;
                    s.2.is_hover = false;
                }
            }
            let mut to_select = q_selectable.get_mut(closest.0).unwrap();
            to_select.2.is_hover = !to_select.2.is_hover;
            if mouse_button_input.just_pressed(MouseButton::Left) {
                to_select.2.is_selected = !to_select.2.is_selected;
            }
        }
    }
}
