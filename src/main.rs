use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).event(event).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window().maximized(true).build().unwrap();

    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}
