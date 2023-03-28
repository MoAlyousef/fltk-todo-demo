use fltk::{
    enums::Mode,
    prelude::*,
    *,
    image::IcoImage
};
use libmpv::{
    render::{
        OpenGLInitParams,
        RenderContext,
        RenderParam,
        RenderParamApiType
    },
    FileState, Mpv,
};
use std::os::raw::c_void;

pub fn get_proc_address(win: &window::GlWindow, name: &str) -> *mut c_void {
    win.get_proc_address(name) as _
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: mpv <video file>");
        std::process::exit(1);
    }
    let a = app::App::default().with_scheme(app::Scheme::Gleam);
    app::get_system_colors();
    let mut win = window::Window::default().with_size(800, 600);
    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    win.make_resizable(true);
    win.set_icon(Some(icon));
    let mut mpv_win = window::GlWindow::new(5, 5, 790, 530, None);
    mpv_win.set_mode(Mode::Opengl3);
    let mut btn = button::Button::new(360, 545, 80, 40, "@||");
    win.end();
    win.make_resizable(true);
    win.show();
    mpv_win.make_current();

    let mut mpv = Mpv::new().expect("Error while creating MPV");
    let render_context = RenderContext::new(
        unsafe { mpv.ctx.as_mut() },
        vec![
            RenderParam::ApiType(RenderParamApiType::OpenGl),
            RenderParam::InitParams(OpenGLInitParams {
                get_proc_address,
                ctx: mpv_win.clone(),
            }),
        ],
    )
    .expect("Failed creating render context");
    mpv.event_context_mut().disable_deprecated_events().unwrap();
    mpv.playlist_load_files(&[(&args[1], FileState::AppendPlay, None)])
        .unwrap();

    btn.set_callback(move |b| {
        let prop: bool = mpv.get_property("pause").unwrap();
        mpv.set_property("pause", !prop).unwrap();
        if prop {
            b.set_label("@||");
        } else {
            b.set_label("@>");
        }
    });

    mpv_win.draw(move |w| {
        render_context
            .render::<window::GlWindow>(0, w.w() as _, w.h() as _, true)
            .expect("Failed to draw on GlWindow");
        w.swap_buffers();
    });

    app::add_idle(move || {
        mpv_win.redraw();
    });

    a.run().unwrap();
}
