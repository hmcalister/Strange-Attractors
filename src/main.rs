extern crate kiss3d;

mod system_of_equations;
mod axes;
mod simulation_points;

use axes::{get_x_axis, get_y_axis, get_z_axis};
use clap::Parser;
use kiss3d::nalgebra::{Point3, Translation3};
use rand::{thread_rng, Rng};
use simulation_points::SimulationPoint;
use system_of_equations::*;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of spheres to add to the simulation
    #[arg(short, long, default_value_t=1000)]
    num_points: usize,

    /// Starting random position scale for simulation points.
    /// 
    /// A larger scale will result in more chaotic behavior sooner
    #[arg(short, long, default_value_t=1.0)]
    initial_position_scale: f32,

    /// The system of equations to simulate. Use  the --help flag to see options.
    ///
    /// Options include:
    ///
    /// * 0: Lorentz
    /// * 1: Aizawa
    /// * 2: Thomas Cyclical
    #[arg(short, long, default_value_t=0)]
    system: i32,
}

fn main() {
    let args = Args::parse();
    let system = map_user_selection_to_system(args.system);

    let mut rng = thread_rng();
    let position_dist = rand_distr::Uniform::from(-args.initial_position_scale..args.initial_position_scale);
    let color_red_dist = rand_distr::Uniform::from(0.4..0.6);
    let color_green_dist = rand_distr::Uniform::from(0.75..0.95);
    let color_blue_dist = rand_distr::Uniform::from(0.7..0.9);


    let mut window = Window::new("Strange Attractors");
    let mut camera = ArcBall::new(
        Point3::new(
            system.init_camera_position.x,
            system.init_camera_position.y,
            system.init_camera_position.z,
        ),
        Point3::new(
            system.init_camera_look_at.x,
            system.init_camera_look_at.y,
            system.init_camera_look_at.z,
        ),
    );
    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));

    let x_axis = get_x_axis();
    let y_axis = get_y_axis();
    let z_axis = get_z_axis();

    let mut simulation_points: Vec<SimulationPoint> = Vec::with_capacity(args.num_points);
    for _ in 0..args.num_points {
        let mut scene_node = window.add_sphere(0.05);
        scene_node.set_color(rng.sample(color_red_dist), rng.sample(color_green_dist), rng.sample(color_blue_dist));
        let x = rng.sample(position_dist);
        let y = rng.sample(position_dist);
        let z = rng.sample(position_dist);
        scene_node.set_local_translation(Translation3::new(x, y, z));

        simulation_points.push(SimulationPoint {
            scene_node: scene_node,
            x: x,
            y: y,
            z: z,
        });
    }



    while window.render_with_camera(&mut camera) {
        x_axis.draw_axis(&mut window);
        y_axis.draw_axis(&mut window);
        z_axis.draw_axis(&mut window);
        for sim_point in &mut simulation_points {
            sim_point.update(&system);
        }
    }
}
