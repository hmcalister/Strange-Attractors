use kiss3d::{scene::SceneNode, nalgebra::Translation3};

use crate::system_of_equations::SystemOfEquations;

pub struct SimulationPoint {
    pub scene_node: SceneNode,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl SimulationPoint {
    pub fn update(self: &mut SimulationPoint, system: &SystemOfEquations) {
        let x_prime = (system.dx)(self.x,self.y,self.z)/system.time_scale;
        let y_prime = (system.dy)(self.x,self.y,self.z)/system.time_scale;
        let z_prime = (system.dz)(self.x,self.y,self.z)/system.time_scale;

        self.x += x_prime;
        self.y += y_prime;
        self.z += z_prime;

        self.scene_node.set_local_translation(Translation3::new(self.x, self.y,self.z));
    }
}