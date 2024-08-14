use renderer::{create_toroid, Renderer};
use nalgebra::{*};

mod renderer;
fn main() {
    let size = termsize::get().unwrap();
    let mut renderer = Renderer::new(1.0, 20.0, size.rows, size.cols);

    let mut elements = create_toroid(0.4, 0.3, 30, 100);

    let translation = Vector3::new(0.0, 0.0,4.0);
    for vertex in elements.iter_mut() {
        vertex.0 += translation;
    }

    let step_z = 0.02;
    let step_y = 0.03;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));

        for (vertex, normal) in elements.iter_mut() {
            let rotation = Rotation3::from_axis_angle(&Vector3::y_axis(), step_z);
            let rotation2 = Rotation3::from_axis_angle(&Vector3::z_axis(), step_y); 

            *vertex -= translation;
            
            *vertex = rotation.transform_vector(vertex);
            *vertex = rotation2.transform_vector(vertex); 
            *normal = rotation.transform_vector(normal);
            *normal = rotation2.transform_vector(normal);

            *vertex += translation; 
        }

        renderer.render_vertices_light(&mut elements, Vector3::new(1.0, 1.0, 0.0).normalize());
        renderer.print_buffer();
    }
}
