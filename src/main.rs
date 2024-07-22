use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Sat View")
        .build();

    // Uncomment this to capture the cursor
    // rl.disable_cursor();

    let mut camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    // rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

    // Example TLE data (replace with actual TLE data)
    let tle = (
        "ISS (ZARYA)",
        "1 25544U 98067A   04236.56031392  .00020137  00000-0  16538-3 0  9993",
        "2 25544  51.6335 344.7760 0007976 126.2523 325.9359 15.70406856328906",
    );

    let elements = sgp4::Elements::from_tle(
        Some(String::from(tle.0)),
        tle.1.as_bytes(),
        tle.2.as_bytes(),
    )
    .expect("Failed to parse TLE");
    let mut points = Vec::new();

    let constants = sgp4::Constants::from_elements(&elements).expect("SGP4 propagation failed");

    // Plot each point every hour for 24 hours
    for hours in 0..24 {
        let prediction = constants
            .propagate(sgp4::MinutesSinceEpoch((hours * 60) as f64))
            .expect("Failed to propagate constants");
        points.push(Vector3::new(
            (prediction.position[0] / 1000.0f64) as f32,
            (prediction.position[1] / 1000.0f64) as f32,
            (prediction.position[2] / 1000.0f64) as f32,
        ));
    }

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d3d = d.begin_mode3D(camera);
            d3d.draw_grid(32, 3.2);

            // Earth
            d3d.draw_sphere_ex(Vector3::new(0.0, 0.0, 0.0), 6.3, 64, 64, Color::LIGHTGRAY);
            // Equator
            d3d.draw_circle_3D(Vector3::new(0.0, 0.0, 0.0), 6.4, Vector3::new(0.0, 0.0, 0.0), 0.0, Color::GREEN);

            // North
            d3d.draw_line_3D(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 10.0), Color::RED);

            // Draw the trajectory
            for point in &points {
                d3d.draw_sphere(*point, 0.1, Color::RED);
                d3d.draw_point3D(*point, Color::RED);
            }
        }
    }
}
