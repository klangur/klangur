// use std::thread;
use conrod_core::{self, widget, Colorable, Positionable, Widget, widget_ids};
use conrod_glium::{self, Renderer};
use conrod_winit;
use glium::{self, glutin, Surface};
use winit;

mod support;

const WIN_W: u32 = 1080;
const WIN_H: u32 = 720;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("test")
        .with_dimensions((WIN_W, WIN_H).into());
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
        // .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64])
    // theme?
        .build();

    widget_ids!(struct Ids { text });
    let ids = Ids::new(ui.widget_id_generator());

    let mut renderer = Renderer::new(&*display).unwrap();

    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    let mut events = Vec::new();

    'render: loop {
        events.clear();
        events_loop.poll_events(|event| { events.push(event); });

        if events.is_empty() {
            events_loop.run_forever(|event| {
                events.push(event);
                glutin::ControlFlow::Break
            });
        }

        for event in events.drain(..) {
            match &event {
                glutin::Event::WindowEvent { event, .. } => {
                    use glutin::WindowEvent;
                    match event {
                        WindowEvent::CloseRequested |
                        WindowEvent::KeyboardInput {
                            input: glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'render,
                        _ => (),
                    }
                }
                _ => (),
            };

            let input = match convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };

            ui.handle_event(input);

            let ui = &mut ui.set_widgets();

            widget::Text::new("hello world")
                .middle_of(ui.window)
                .color(conrod_core::color::WHITE)
                .font_size(32)
                .set(ids.text, ui);
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&*display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&*display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }

    // use std::sync::mpsc;
    // let (event_tx, event_rx) = mpsc::channel();
    // let (render_tx, render_rx) = mpsc::channel();
    // let events_loop_proxy = events_loop.create_proxy();

    // thread::spawn(move || run_conrod());
}

// apparently you need to do this
conrod_winit::conversion_fns!();
