use kiss3d::{nalgebra::Point3, window::Window};

pub struct Axis {
    a: Point3<f32>,
    b: Point3<f32>,
    color: Point3<f32>,
}

impl Axis {
    pub fn draw_axis(self: &Axis, window: &mut Window){
        window.draw_line(&self.a, &self.b, &self.color);
    }
}

pub fn get_x_axis() -> Axis {
    Axis {
        a: Point3::new(-100.0, 0.0, 0.0),
        b: Point3::new(100.0, 0.0, 0.0),
        color: Point3::new(1.0, 0.0, 0.0),
    }
}

pub fn get_y_axis() -> Axis {
    Axis {
        a: Point3::new(0.0, -100.0, 0.0),
        b: Point3::new(0.0, 100.0, 0.0),
        color: Point3::new(0.0, 1.0, 0.0),
    }
}

pub fn get_z_axis() -> Axis {
    Axis {
        a: Point3::new(0.0, 0.0, -100.0),
        b: Point3::new(0.0, 0.0, 100.0),
        color: Point3::new(0.0, 0.0, 1.0),
    }
}
