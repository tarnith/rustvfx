// minimal example of a nannou sketch
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(100.0, 100.0)
        .color(GREY);
    
    match view {
        KeyPressed(_key) => {
            println!("You pressed a key");
        }
    }


    draw.to_frame(app, &frame).unwrap();
}