use bevy::{math::Vec3Swizzles, prelude::*};
use input::{InputPlugin, UserInputs};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InputPlugin);
        app.add_system(selection.system());
    }
}

pub struct Selectable {
    pub size: f32,
    pub is_selected: bool,
}

impl Selectable {
    pub fn new(size: f32, is_selected: bool) -> Self {
        Self { size, is_selected }
    }
}

fn selection(
    user_inputs: Res<UserInputs>,
    mut q_selectable: Query<(Entity, &Transform, &mut Selectable)>,
) {
    if let Some(input::UserInput::Click(click)) = &user_inputs.click {
        let mut closest = None;
        for mut s in q_selectable.iter_mut() {
            let mut distance = s.1.translation.xy().distance(*click);
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
                }
            }
            let mut to_select = q_selectable.get_mut(closest.0).unwrap();
            to_select.2.is_selected = !to_select.2.is_selected;
            dbg!(closest.0);
        }
    }
}
