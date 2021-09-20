use std::{collections::VecDeque, error::Error, iter, time::Instant};

use three_d::{core::Indices, *};

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().pretty().init();

    // Create a window
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })?;

    // Get the graphics context from the window
    let context = window.gl()?;

    // Create the lighting pipeline
    let mut pipeline = DeferredPipeline::new(&context)?;

    // Create a camera
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 20.0, 20.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    )?;
    let mut control = OrbitControl::new(
        *camera.target(),
        0.5 * camera.target().distance(*camera.position()),
        10.0 * camera.target().distance(*camera.position()),
    );

    let axes = Axes::new(&context, 2.0, 10.0)?;
    let mut gui = GUI::new(&context)?;

    let material = Material::new(
        &context,
        &CPUMaterial {
            albedo: Color::GREEN,
            ..Default::default()
        },
    )?;

    let ground = Model::new(
        &context,
        &CPUMesh {
            name: "ground".into(),
            material_name: None,
            positions: [
                vec3(-10000.0, 0.0, 10000.0),
                vec3(10000.0, 0.0, 10000.0),
                vec3(0.0, 0.0, -10000.0),
            ]
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect(),
            indices: None,
            normals: Some(
                iter::repeat(vec3(0.0, 1.0, 0.0))
                    .take(3)
                    .flat_map(|v| [v.x, v.y, v.z])
                    .collect(),
            ),
            uvs: None,
            colors: None,
        },
    )?;

    let mut cube = Model::new(&context, &CPUMesh::cube())?;
    cube.set_transformation(Mat4::from_translation(Vec3::new(0.0, 10.0, 2.0)));

    let mut sunlight = DirectionalLight::new(&context, 0.5, Color::WHITE, &vec3(0.0, -1.0, 0.2))?;

    let mut fps_rolling_average = VecDeque::with_capacity(1000);
    let mut frame_time_rolling_average = VecDeque::with_capacity(1000);

    // Start the main render loop
    window.render_loop(move |mut frame_input: FrameInput| {
        let frame_start = Instant::now();

        if fps_rolling_average.len() == fps_rolling_average.capacity() {
            fps_rolling_average.pop_back();
        }
        fps_rolling_average.push_front(1000.0 / frame_input.elapsed_time);

        // Ensure the viewport matches the current window viewport which changes if the window is resized
        gui.update(&mut frame_input, |gui_context| {
            use three_d::egui::{plot::*, *};

            Window::new("Visualization Properties")
                // .default_size((0.0, 0.0))
                .show(&gui_context, |ui| {
                    ui.heading("Camera Position");
                    ui.label(format!("{:?}", camera.position()));
                    ui.heading("Camera Target");
                    ui.label(format!("{:?}", camera.target()));

                    ui.separator();
                    ui.heading("Frame Time");
                    ui.label(format!(
                        "{} ms",
                        frame_time_rolling_average.iter().sum::<u128>()
                            / frame_time_rolling_average.len().max(1) as u128
                    ));
                    ui.add(
                        Plot::new("Frame Time")
                            .line(
                                Line::new(Values::from_values_iter(
                                    frame_time_rolling_average
                                        .iter()
                                        .enumerate()
                                        .map(|(x, &y)| Value::new(-(x as f64), y as f64)),
                                ))
                                .highlight(),
                            )
                            .allow_zoom(false)
                            .allow_drag(false)
                            .include_y(1.0 / 60.0)
                            .include_y(0)
                            .view_aspect(2.0),
                    );
                    ui.heading("Frame Per Second");
                    ui.label(format!(
                        "{:.1}",
                        fps_rolling_average.iter().sum::<f64>() / fps_rolling_average.len() as f64
                    ));
                    ui.label(format!("{}", fps_rolling_average.len()));
                    ui.add(
                        Plot::new("Frame Time")
                            .line(
                                Line::new(Values::from_values_iter(
                                    fps_rolling_average
                                        .iter()
                                        .enumerate()
                                        .map(|(x, &y)| Value::new(-(x as f64), y)),
                                ))
                                .highlight(),
                            )
                            .allow_zoom(false)
                            .allow_drag(false)
                            .include_y(0)
                            .view_aspect(2.0),
                    );

                    // ui.allocate_space(ui.available_size());
                });
        })
        .unwrap();

        // Setup camera
        camera.set_viewport(frame_input.viewport).unwrap();
        control
            .handle_events(&mut camera, &mut frame_input.events)
            .unwrap();

        // Move cube
        cube.set_transformation(Mat4::from_angle_x(radians(0.01)).concat(cube.transformation()));

        // Calculate shadows
        sunlight
            .generate_shadow_map(&Vec3::new(0.0, 0.0, 0.0), 40.0, 200.0, 1024, 1024, &[&cube])
            .unwrap();

        // Calculate geometry
        pipeline
            .geometry_pass(&camera, &[(&ground, &material), (&cube, &material)])
            .unwrap();

        // Start writing to the screen and clears the color and depth
        Screen::write(
            &context,
            ClearState::color_and_depth(0.2, 0.2, 0.2, 1.0, 1.0),
            || {
                // axes.render(&camera)?;
                pipeline.light_pass(&camera, None, &[&sunlight], &[], &[])?;

                gui.render()?;

                Ok(())
            },
        )
        .unwrap();

        if frame_time_rolling_average.len() == frame_time_rolling_average.capacity() {
            frame_time_rolling_average.pop_back();
        }
        frame_time_rolling_average.push_front((Instant::now() - frame_start).as_millis());

        // Returns default frame output to end the frame
        FrameOutput::default()
    })?;

    Ok(())
}
