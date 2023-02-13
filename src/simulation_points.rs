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
        const TIME_SCALE: f32 = 100.0;
        self.x += (system.dx)(self.x,self.y,self.z)/TIME_SCALE;
        self.y += (system.dy)(self.x,self.y,self.z)/TIME_SCALE;
        self.z += (system.dz)(self.x,self.y,self.z)/TIME_SCALE;

        self.scene_node.set_local_translation(Translation3::new(self.x, self.y,self.z));
    }
}