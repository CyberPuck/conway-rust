/*use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return
    }
    let filename = &args[1];
    // TODO: Implement the following
    //read_input_file(filename);
    // let game = create_game(filename);
    // game.run();
}

fn print_help() {
    unimplemented!();
}
*/
mod conway_engine;
mod grid;
use nannou::prelude::*;

struct Model {
    position_x: f32,
    position_y: f32,
    engine: conway_engine::ConwayEngine,
    window_height: f32,
    window_width: f32,
}

fn main() {
    let grid: grid::Grid<usize> = grid::Grid::new(5, 6, 0);
    // TODO: Setup the game here?
    nannou::app(model)
        //.event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    // NOTE:  Should the game be setup in the model?
    let window_data = app.main_window().rect();
    let engine = conway_engine::ConwayEngine::new(
        &"test.txt".to_string(),
        window_data.h(),
        window_data.w(),
        1,
        100,
    );
    Model {
        position_x: 50.0,
        position_y: 50.0,
        engine,
        window_height: window_data.h(),
        window_width: window_data.w(),
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    unimplemented!();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.position_x > 150.0 {
        model.position_x = 50.0;
    }
    if model.position_y > 150.0 {
        model.position_y = 50.0;
    }
    model.position_x += 1.0;
    model.position_y += 1.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(WHITE);

    draw_scene(model, &draw);

    // update the squares
    /*draw.rect()
        .color(RED)
        .w(25.0)
        .h(25.0)
        .x_y(model.position_x, model.position_y);
    */
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

fn draw_scene(model: &Model, draw: &Draw) {
    // TODO: Draw a checkerboard as a first test
    for row_number in 0..((model.window_height / model.position_y) as i32) {
        for column_number in 0..((model.window_width / model.position_x) as i32) {
            if row_number % 2 == 0 && column_number % 2 == 0 {
                draw.rect().color(BLACK).w(50.0).h(50.0).x_y(
                    (row_number as f32) * model.position_y,
                    (column_number as f32) * model.position_x,
                );
            }
        }
    }
}
