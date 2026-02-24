use std::{cell::RefCell, rc::Rc};

use three_d::*;
mod custom;
mod game_engine;

use game_engine::{Instance, InstanceRef, Part, Workspace};
use game_engine::Object;

#[tokio::main]
async fn main() {
    run().await;
}

pub async fn run() {
    // =========================
    // Scene Setup
    // =========================

    let workspace: InstanceRef = Rc::new(RefCell::new(Workspace::new()));

    let mut window = Window::new(WindowSettings {
        title: "True".to_string(),
        max_size: Some((1080, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let part: InstanceRef = Rc::new(RefCell::new(Part::new(
        "MyPart",
        Mesh::new(&context, &CpuMesh::cube()),
    )));

    workspace.borrow_mut().add_child(&part);

    // =========================
    // Selection
    // =========================

    let mut selected: Option<InstanceRef> = None;

    // =========================
    // Camera
    // =========================

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.5, 0.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut controller = custom::custom_controller {
        current_position: camera.position(),
        target: camera.target(),
        yaw: 0.0,
        pitch: 0.0,
        distance: 5.0,
        ..Default::default()
    };

    let ambient = AmbientLight::new(&context, 1.0, Srgba::WHITE);
    let mut gui = GUI::new(&context);

    let mut x_str = String::new();
    let mut y_str = String::new();
    let mut z_str = String::new();
    let mut x_rot_str = String::new();
    let mut y_rot_str = String::new();
    let mut z_rot_str = String::new();
    let mut red_str = "0".to_owned();
    let mut green_str = "0".to_owned();
    let mut blue_str = "0".to_owned();

    // =========================
    // Render Loop
    // =========================

    window.render_loop(move |mut frame_input| {
        controller.handle_events(&mut camera, &mut frame_input.events);

        // ================= GUI =================
        let mut panel_width = 0.0;
        let mut scene_viewport = frame_input.viewport;

        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                use three_d::egui::*;

                SidePanel::left("explorer").show(ctx, |ui|{
                    ui.heading("EXPLORER");
                    let thingys = ui.button("Add new Part");

                    if thingys.clicked(){
                        let part: InstanceRef = Rc::new(RefCell::new(Part::new(
                            "MyPart",
                            three_d::Mesh::new(&context, &CpuMesh::cube()),
                        )));

                        let mut workspace_borrowed = workspace.borrow_mut();

                        workspace_borrowed.get_children_mut().push(part);
                    }

                    egui::ScrollArea::vertical().show(ui, |ui|{
                        let workspace_borrowed = workspace.borrow();
                        let children = workspace_borrowed.get_children();

                        for child in children{
                            let borrowed = child.borrow();
                            let response = ui.add(Label::new(borrowed.get_name()).sense(egui::Sense::click()));

                            if response.clicked(){
                                selected = Some(child.clone());
                            }
                        }
                    })
                });

                SidePanel::right("properties").show(ctx, |ui| {
                    ui.heading("Properties");
                    panel_width = ui.available_width();

                    if let Some(sel) = &selected {
                        let mut borrowed = sel.borrow_mut();

                        if let Some(part) =
                            borrowed.as_any_mut().downcast_mut::<Part>()
                        {
                            ui.heading(part.get_name());

                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(TextEdit::singleline(&mut x_str).desired_width(40.0));
                                ui.label("Y:");
                                ui.add(TextEdit::singleline(&mut y_str).desired_width(40.0));
                                ui.label("Z:");
                                ui.add(TextEdit::singleline(&mut z_str).desired_width(40.0));
                            });

                            ui.horizontal(|ui| {
                                ui.label("X_ROT:");
                                ui.add(TextEdit::singleline(&mut x_rot_str).desired_width(40.0));
                                ui.label("Y_ROT:");
                                ui.add(TextEdit::singleline(&mut y_rot_str).desired_width(40.0));
                                ui.label("Z_ROT:");
                                ui.add(TextEdit::singleline(&mut z_rot_str).desired_width(40.0));
                            });

                            ui.horizontal(|ui| {
                                ui.label("RED:");
                                ui.add(TextEdit::singleline(&mut red_str).desired_width(40.0));
                                ui.label("GREEN:");
                                ui.add(TextEdit::singleline(&mut green_str).desired_width(40.0));
                                ui.label("BLUE:");
                                ui.add(TextEdit::singleline(&mut blue_str).desired_width(40.0));
                            });

                            panel_width = ui.available_width();
                        }
                    }
                });
            },
        );

        if let Some(thin) = &selected{
            let mut borrowed = thin.borrow_mut();
            let thing = borrowed.as_any_mut().downcast_mut::<Part>().unwrap();
            let x: f32 = x_str.parse().unwrap_or(0.0);
            let y: f32 = y_str.parse().unwrap_or(0.0);
            let z: f32 = z_str.parse().unwrap_or(0.0);
            let x_rot: f32 = x_rot_str.parse().unwrap_or(0.0);
            let y_rot: f32 = y_rot_str.parse().unwrap_or(0.0);
            let z_rot: f32 = z_rot_str.parse().unwrap_or(0.0);

            let red = red_str.parse().unwrap_or(0);
            let green = green_str.parse().unwrap_or(0);
            let blue = blue_str.parse().unwrap_or(0);

            thing.position = vec3(x,y,z);
            thing.rotation = vec3(x_rot,y_rot,z_rot);
            thing.color = Srgba{
                r: red,
                g: green,
                b: blue,
                a: 255
            };
        }

        // ================= Picking =================

        for event in &frame_input.events {
            if let Event::MousePress {
                button: MouseButton::Left,
                position,
                handled,
                ..
            } = event
            {
                if !*handled {
                    let vp = frame_input.viewport;

                    let x = (2.0 * position.x as f32) / vp.width as f32 - 1.0;
                    let y = 1.0 - (2.0 * position.y as f32) / vp.height as f32;

                    let inv = (camera.projection() * camera.view())
                        .invert()
                        .unwrap();

                    let near = inv * vec4(x, y, -1.0, 1.0);
                    let far = inv * vec4(x, y, 1.0, 1.0);

                    let near_world = near.truncate() / near.w;
                    let far_world = far.truncate() / far.w;

                    let dir = (far_world - near_world).normalize();

                    let mut closest_dist = f32::MAX;
                    let mut closest: Option<InstanceRef> = None;

                    for node in workspace.borrow().get_descendants() {
                        let mut borrowed = node.borrow_mut();

                        if let Some(part) = borrowed.as_any_mut().downcast_mut::<Part>() {
                            let aabb = part.get_renderable().aabb();

                            let mut current = 0.0;
                            while current <= 250.0 {
                                let position = near_world + current * dir;

                                if aabb.is_inside(position) {
                                    let dist = (position - near_world).magnitude();

                                    if dist < closest_dist {
                                        closest_dist = dist;
                                        closest = Some(node.clone());

                                        let center = aabb.center();
                                        x_str = center.x.to_string();
                                        y_str = center.y.to_string();
                                        z_str = center.z.to_string();
                                    }

                                    break; // stop stepping once we hit this AABB
                                }

                                current += 0.05;
                            }
                        }
                    }

                    selected = closest;
                }
            }
        }

        // ================= Rendering =================

        camera.set_viewport(scene_viewport);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0));

        let descendants = workspace.borrow().get_descendants();

        for node in descendants {
            let mut borrowed = node.borrow_mut();

            if let Some(part) = borrowed.as_any_mut().downcast_mut::<Part>() {
                let pos = part.position.clone();
                let rot = part.rotation.clone();
                let color = part.color.clone();
                part.get_renderable_mut().set_transformation(Mat4::from_translation(pos) * Mat4::from_angle_x(degrees(rot.x)) * Mat4::from_angle_y(degrees(rot.y)) * Mat4::from_angle_z(degrees(rot.z)));
                part.get_renderable_mut().material.color = [color.r as f32/255.0, color.g as f32/255.0, color.b as f32/255.0];
                frame_input
                    .screen()
                    .render(&camera, std::iter::once(part.get_renderable_mut()), &[&ambient]);
            }
        }

        frame_input.screen()
            .write(|| gui.render())
            .unwrap();

        FrameOutput::default()
    });
}