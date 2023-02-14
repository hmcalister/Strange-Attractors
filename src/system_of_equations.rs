#![allow(unused_variables)]

extern crate kiss3d;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct SystemOfEquations {
    pub time_scale: f32,
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
    let mapper: HashMap<i32, SystemOfEquations> = HashMap::from([
        (0, LORENTZ_SYSTEM),
        (1, AIZAWA_SYSTEM),
        (2, THOMAS_CYCLICAL),
        (3, HALVORSEN),
        (4, FOUR_WING),
    ]);

    mapper[&selection]
}

const LORENTZ_RHO: f32 = 28.0;
const LORENTZ_SIGMA: f32 = 10.0;
const LORENTZ_BETA: f32 = 8.0 / 3.0;
const LORENTZ_SYSTEM: SystemOfEquations = SystemOfEquations {
    time_scale: 100.0,
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

/// Constants taken from https://www.cedrick.ai/posts/attractors.html
const AIZAWA_A: f32 = 0.95;
const AIZAWA_B: f32 = 0.7;
const AIZAWA_C: f32 = 0.65;
const AIZAWA_D: f32 = 3.5;
const AIZAWA_E: f32 = 0.25;
const AIZAWA_F: f32 = 0.1;
const AIZAWA_SYSTEM: SystemOfEquations = SystemOfEquations {
    time_scale: 50.0,
    dx: |x, y, z| (z - AIZAWA_B) * x - AIZAWA_D * y,
    dy: |x, y, z| AIZAWA_D * x + (z - AIZAWA_B) * y,
    dz: |x, y, z| {
        AIZAWA_C + AIZAWA_A * z - z.powi(3) / 3.0 - (x.powi(2) - y.powi(2)) * (1.0 + AIZAWA_E * z)
            + AIZAWA_F * z * x.powi(3)
    },
    init_camera_position: CameraPosition {
        x: 2.0,
        y: 0.1,
        z: 20.0,
    },
    init_camera_look_at: CameraPosition {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};

// Consts and equations taken from https://www.cedrick.ai/posts/attractors.html
const THOMAS_CYCLICAL_B: f32 = 0.208186;
const THOMAS_CYCLICAL: SystemOfEquations = SystemOfEquations {
    time_scale: 10.0,
    dx: |x, y, z| y.sin() - THOMAS_CYCLICAL_B * x,
    dy: |x, y, z| z.sin() - THOMAS_CYCLICAL_B * y,
    dz: |x, y, z| x.sin() - THOMAS_CYCLICAL_B * z,
    init_camera_position: CameraPosition {
        x: -4.0,
        y: 5.0,
        z: 8.0,
    },
    init_camera_look_at: CameraPosition {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};

/// Constants and equations from https://www.dynamicmath.xyz/strange-attractors/
const HALVORSEN_ALPHA: f32 = 1.89;
const HALVORSEN: SystemOfEquations = SystemOfEquations {
    time_scale: 100.0,
    dx: |x, y, z| -HALVORSEN_ALPHA * x - 4.0 * y - 4.0 * z - y.powi(2),
    dy: |x, y, z| -HALVORSEN_ALPHA * y - 4.0 * z - 4.0 * x - z.powi(2),
    dz: |x, y, z| -HALVORSEN_ALPHA * z - 4.0 * x - 4.0 * y - x.powi(2),
    init_camera_position: CameraPosition {
        x: 15.0,
        y: 10.0,
        z: 2.0,
    },
    init_camera_look_at: CameraPosition {
        x: -2.0,
        y: -2.0,
        z: -2.0,
    },
};

/// Constants and equations from https://www.dynamicmath.xyz/strange-attractors/
const FOUR_WING_A: f32 = 0.2;
const FOUR_WING_B: f32 = 0.01;
const FOUR_WING_C: f32 = -0.4;
const FOUR_WING: SystemOfEquations = SystemOfEquations {
    time_scale: 20.0,
    dx: |x, y, z| FOUR_WING_A * x + y * z,
    dy: |x, y, z| FOUR_WING_B * x + FOUR_WING_C * y - x * z,
    dz: |x, y, z| -z - x * y,
    init_camera_position: CameraPosition {
        x: 3.0,
        y: 5.0,
        z: 10.0,
    },
    init_camera_look_at: CameraPosition {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};
