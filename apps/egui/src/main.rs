use radiant_runtime::{
    RadiantMessage, RadiantNodeType, RadiantPathNode, RadiantRectangleNode, RadiantResponse,
    RadiantRuntime, RadiantTextNode, RectangleTool,
};
use std::iter;
use winit::event::Event::RedrawRequested;

use egui::{FontDefinitions, Id};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};

struct RadiantAppController {
    pending_messages: Vec<RadiantMessage>,
}

impl RadiantAppController {
    fn new() -> Self {
        Self {
            pending_messages: Vec::new(),
        }
    }
}

impl RadiantAppController {
    fn update(&mut self, ctx: &egui::Context) {
        let frame = egui::Frame::none().fill(egui::Color32::TRANSPARENT);
        egui::TopBottomPanel::top(Id::new("top"))
            .frame(frame)
            .show(ctx, |ui| {
                ui.heading("Radiant App");
                if ui.button("Select").clicked() {
                    self.pending_messages
                        .push(RadiantMessage::SelectTool { id: 0 });
                }
                if ui.button("Rect").clicked() {
                    self.pending_messages
                        .push(RadiantMessage::SelectTool { id: 1 });
                }
                ui.add_space(10.0);
            });
    }
}

async fn run() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
        println!("Response: {:?}", response);
    });

    let mut runtime = RadiantRuntime::new(handler).await;
    runtime
        .app
        .scene
        .tool_manager
        .register_tool(Box::new(RectangleTool::new()));
    runtime
        .app
        .scene
        .add(RadiantNodeType::Rectangle(RadiantRectangleNode::new(
            1,
            [200.0, 200.0],
            [200.0, 200.0],
        )));
    runtime
        .app
        .scene
        .add(RadiantNodeType::Path(RadiantPathNode::new(
            2,
            [400.0, 400.0],
        )));
    runtime
        .app
        .scene
        .add(RadiantNodeType::Text(RadiantTextNode::new(
            3,
            [400.0, 600.0],
            [200.0, 200.0],
        )));
    // app.scene.handle_message(RadiantMessage::AddText {
    //     text: String::from("Hello World!"),
    //     position: [400.0, 600.0],
    // });
    runtime.handle_message(RadiantMessage::AddImage {
        name: String::from("test"),
        path: String::from("test.png"),
    });

    let size = runtime.app.window.inner_size();
    let scale_factor = runtime.app.window.scale_factor();

    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor,
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut egui_rpass = RenderPass::new(
        &runtime.app.scene.render_manager.device,
        runtime.app.scene.render_manager.config.format,
        1,
    );
    let mut demo_app = RadiantAppController::new();

    if let Some(event_loop) = std::mem::replace(&mut runtime.app.event_loop, None) {
        event_loop.run(move |event, _, control_flow| {
            if demo_app.pending_messages.len() > 0 {
                for message in demo_app.pending_messages.drain(..) {
                    runtime.handle_message(message);
                }
            }

            platform.handle_event(&event);

            if !platform.captures_event(&event) {
                if let Some(message) = runtime.app.handle_event(&event, control_flow) {
                    if let Some(response) = runtime.handle_message(message) {
                        println!("Response: {:?}", response);
                    }
                }
            }

            match event {
                RedrawRequested(..) => {
                    let output_frame = std::mem::replace(
                        &mut runtime.app.scene.render_manager.current_texture,
                        None,
                    );
                    let output_frame = output_frame.unwrap();

                    let output_view = runtime
                        .app
                        .scene
                        .render_manager
                        .current_view
                        .as_ref()
                        .unwrap();

                    platform.begin_frame();

                    demo_app.update(&platform.context());

                    let full_output = platform.end_frame(Some(&runtime.app.window));
                    let paint_jobs = platform.context().tessellate(full_output.shapes);

                    // Upload all resources for the GPU.
                    let screen_descriptor = ScreenDescriptor {
                        physical_width: runtime.app.scene.render_manager.config.width,
                        physical_height: runtime.app.scene.render_manager.config.height,
                        scale_factor: scale_factor as f32,
                    };
                    let tdelta: egui::TexturesDelta = full_output.textures_delta;
                    egui_rpass
                        .add_textures(
                            &runtime.app.scene.render_manager.device,
                            &runtime.app.scene.render_manager.queue,
                            &tdelta,
                        )
                        .expect("add texture ok");
                    egui_rpass.update_buffers(
                        &runtime.app.scene.render_manager.device,
                        &runtime.app.scene.render_manager.queue,
                        &paint_jobs,
                        &screen_descriptor,
                    );

                    let mut encoder = runtime
                        .app
                        .scene
                        .render_manager
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        });
                    // Record all render passes.
                    egui_rpass
                        .execute(
                            &mut encoder,
                            &output_view,
                            &paint_jobs,
                            &screen_descriptor,
                            None,
                            // Some(wgpu::Color::BLACK),
                        )
                        .unwrap();
                    // Submit the commands.
                    runtime
                        .app
                        .scene
                        .render_manager
                        .queue
                        .submit(iter::once(encoder.finish()));

                    // Redraw egui
                    output_frame.present();

                    egui_rpass
                        .remove_textures(tdelta)
                        .expect("remove texture ok");
                }
                _ => (),
            }
        });
    }
}

fn main() {
    pollster::block_on(run());
}
