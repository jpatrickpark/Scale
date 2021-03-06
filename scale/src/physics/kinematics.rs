use crate::gui::InspectVec2;
use cgmath::num_traits::zero;
use cgmath::Vector2;
use imgui_inspect_derive::*;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Component, Debug, Inspect, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Kinematics {
    #[inspect(proxy_type = "InspectVec2")]
    pub velocity: Vector2<f32>,
    #[inspect(proxy_type = "InspectVec2", skip = true)]
    pub acceleration: Vector2<f32>,
    pub mass: f32,
}

impl Kinematics {
    pub fn from_mass(mass: f32) -> Self {
        Kinematics {
            velocity: zero(),
            acceleration: zero(),
            mass,
        }
    }
}
