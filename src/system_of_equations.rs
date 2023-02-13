#![allow(unused_variables)]

extern crate kiss3d;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct SystemOfEquations {
    pub dx: fn(f32, f32, f32) -> f32,
    pub dy: fn(f32, f32, f32) -> f32,
    pub dz: fn(f32, f32, f32) -> f32,
    pub init_camera_position: CameraPosition,
    pub init_camera_look_at: CameraPosition,
}

#[derive(Clone, Copy)]
pub struct CameraPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn map_user_selection_to_system(selection: i32) -> SystemOfEquations {
    let mapper: HashMap<i32, SystemOfEquations> = HashMap::from([(0, LORENTZ_SYSTEM)]);

    mapper[&selection]
}

const LORENTZ_RHO: f32 = 28.0;
const LORENTZ_SIGMA: f32 = 10.0;
const LORENTZ_BETA: f32 = 8.0 / 3.0;
const LORENTZ_SYSTEM: SystemOfEquations = SystemOfEquations {
    dx: |x, y, z| LORENTZ_SIGMA * (y - x),
    dy: |x, y, z| x * (LORENTZ_RHO - z) - y,
    dz: |x, y, z| x * y - LORENTZ_BETA * z,
    init_camera_position: CameraPosition {
        x: 100.0,
        y: 10.0,
        z: 75.0,
    },
    init_camera_look_at: CameraPosition {
        x: 0.0,
        y: 0.0,
        z: 30.0,
    },
};