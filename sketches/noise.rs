// minimal example of a nannou sketch
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    

    draw.to_frame(app, &frame).unwrap();
}