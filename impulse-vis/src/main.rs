use std::error::Error;

use three_d::*;

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
        5.0 * camera.target().distance(*camera.position()),
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

    let ground = Model::new(&context, &CPUMesh::cube())?;

    // Start the main render loop
    window.render_loop(move |mut frame_input: FrameInput| {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        gui.update(&mut frame_input, |gui_context| {
            use three_d::egui::*;

            Window::new("Visualization Parameters")
                // .default_size((0.0, 0.0))
                .show(&gui_context, |ui| {
                    ui.heading("Camera Position");

                    ui.label(format!("{:?}", camera.position()));
                    ui.label(format!("{:?}", camera.target()));

                    // ui.allocate_space(ui.available_size());
                });
        })
        .unwrap();

        camera.set_viewport(frame_input.viewport).unwrap();
        control
            .handle_events(&mut camera, &mut frame_input.events)
            .unwrap();

        // Start writing to the screen and clears the color and depth
        Screen::write(
            &context,
            ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0),
            || {
                // axes.render(&camera)?;
                ground.render_with_lighting(
                    &camera,
                    &material,
                    LightingModel::Blinn,
                    Some(&AmbientLight {
                        color: Color::WHITE,
                        intensity: 0.1,
                    }),
                    &[],
                    &[&SpotLight::new(
                        &context,
                        2.0,
                        Color::WHITE,
                        &vec3(0.0, 100000.0, 1.0),
                        &vec3(0.0, -1.0, 0.0),
                        f32::MAX,
                        0.0,
                        0.0,
                        0.0,
                    )?],
                    &[],
                )?;
                ground.render_normals(&camera)?;

                Ok(())
            },
        )
        .unwrap();

        gui.render().unwrap();

        // Returns default frame output to end the frame
        FrameOutput::default()
    })?;

    Ok(())
}
