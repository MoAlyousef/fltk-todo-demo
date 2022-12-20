# fltk-rs demos

This is a repo for fltk-rs demo examples. These are usually associated with video tutorials from this [playlist](https://www.youtube.com/playlist?list=PLHqrrowPLkDu9U-uk60sGM-YWLOJFfLoE) on youtube.

Also the fltk-rs repo has an [examples directory](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples) for several standalone examples. These demo apps mainly show interop with other projects in the Rust ecosystem.

The current demos include:
- web-todo: Creating an async web todo app using fltk, reqwest, serde and tokio.
- web-todo2: Creating an async web todo app using fltk, surf, serde and async-std.
- libvlc: Creating a media player using fltk and the vlc crate.
- musicplayer: Creating a music player using custom widgets and the soloud crate. 
- opengl: Raw OpenGL drawing in an fltk GlWindow.
- glut: Use the gl crate (An OpenGL function pointer loader) to do OpenGL drawing.
- wgpu: Use wgpu-rs for gpu accelerated drawing.
- pixels: Use the pixels crate to draw a wgpu accelerated framebuffer.
- framebuffer: Using fltk for framebuffer drawing.
- plotters: Use plotters for live plotting (drawing animations) with fltk.
- raqote: Use raqote for custom drawing (paint example).
- tinyskia: Use tiny-skia for custom drawing.
- systray: Use nwg to create an fltk app with systray functionalities on Windows.
- glow: Use the glow crate to do OpengGL drawing.
- glium: Use the glium crate for OpenGL drawing.
- calendar: Uses the chrono crate to create an fltk calendar dialog.
- image: Uses rust-embed and the image crates to load images into fltk.
- speedy2d: Uses speedy2D crate to do 2D drawings of a circle and an RGB image in a GlWindow.
- femtovg: Uses femtovg for 2D drawing in a GlWindow.
- ffmpeg: Uses ffmpeg for software video rendering.
- webview: Embeds a webview inside an fltk app.
- csv: Uses serde and csv to perform custom drawing of data.
- rounded-svg: Use the svg crate along with fltk to create images with rounded borders.
- libmpv: use libmpv to play a video inside an fltk GlWindow.
- mpv: Use mpv (the command line app) to play a video inside an fltk window.
- xterm: embed an xterm window inside an fltk window.
- egui-demo: Use fltk as a backend for egui.
- gst: Use libgstreamer to play a video inside an fltk window
- cairo-demo: Use cairo for custom drawing inside fltk widgets
- glyphmap: Maps glyphs (specifically font icons) to their unicode codepoint.
- terminal: A minimal terminal emulator.

The demos can be run by accessing any of the directories and running:
```
cargo run --release
```

## Screenshots:
- musicplayer:
![alt_test](musicplayer/musicplayer.png)
- web-todo:
![alt_test](web-todo/ex.jpg)
- libvlc
![alt_test](libvlc/ex.jpg)
- opengl
![alt_test](opengl/ex.jpg)
- glut
![alt_test](glut/ex.png)
- pixels
![alt_test](pixels/ex.jpg)
- plotters
![alt_test](plotters/ex.jpg)
- raqote
![alt_test](raqote/ex.jpg)
- tinyskia
![alt_test](tinyskia/ex.jpg)
- glow
![alt_test](glow/ex.jpg)
- glium
![alt_test](glium/ex.jpg)
- calendar
![alt_test](calendar/ex.jpg)
- speedy2d
![alt_test](speedy2d/ex.jpg)
- femtovg
![alt_test](femtovg/ex.png)
- webview
![alt_test](webview/ex.jpg)
- csv
![alt_test](csv/ex.jpg)
- egui-demo
![alt_test](egui-demo/egui.jpg)
- glyphmap
![alt_test](glyphmap/image.jpg)
