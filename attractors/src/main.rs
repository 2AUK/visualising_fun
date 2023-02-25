use nannou::prelude::*;
use nannou_egui::{egui, Egui};


struct Settings {
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
}

struct Model {
    points: Vec<Vec3>,
    settings: Settings,
    egui: Egui,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const N_POINTS: usize = 1;

fn model(app: &App) -> Model {
    let window_id = app.new_window().size(600, 800).view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();

    let mut points = Vec::new();
    let egui = Egui::from_window(&window);

    let settings = Settings { x_rotation: 0.0, y_rotation: 0.0, z_rotation: 0.0 };


    for i in 0..N_POINTS {
        points.push(vec3(
            0.1,
            0.0,
            0.0
        ));
    }


    Model { points, settings, egui}
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("X Rotation:");
        let x_clicked = ui.add(egui::Slider::new(&mut settings.x_rotation, 0.0..=360.0)).changed();

        ui.label("Y Rotation:");
        let y_clicked = ui.add(egui::Slider::new(&mut settings.y_rotation, 0.0..=360.0)).changed();

        ui.label("Z Rotation:");
        let z_clicked = ui.add(egui::Slider::new(&mut settings.z_rotation, 0.0..=360.0)).changed();

    });

    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;
    let timestep = 0.01;

    for point in model.points.iter_mut() {
        point[0] += (sigma * (point[1] - point[0])) * timestep;
        point[1] += (point[0] * (rho - point[2]) - point[1]) * timestep;
        point[2] += ((point[0] * point[1]) - (beta * point[2])) * timestep;
    }
}

fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let scale = 10.0;
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let x_rot = Mat3::from_rotation_x(deg_to_rad(settings.x_rotation));
    let y_rot = Mat3::from_rotation_x(deg_to_rad(settings.y_rotation));
    let z_rot = Mat3::from_rotation_x(deg_to_rad(settings.z_rotation));

    for point in model.points.iter() {
        let mut mod_point = point.clone();
        mod_point = x_rot * y_rot * z_rot * mod_point;
        draw.ellipse().x_y_z(mod_point[0] * scale, mod_point[1] * scale, mod_point[2] * scale).radius(0.7).color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
    //model.egui.draw_to_frame(&frame).unwrap();
}
