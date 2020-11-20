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
    nannou::app(model)
        //.event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    // setup the game
    let window_data = app.main_window().rect();
    let engine = conway_engine::ConwayEngine::new(
        &"test-files/test2.txt".to_string(),
        window_data.h(),
        window_data.w(),
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

    // Draw the scene
    draw_scene(model, &draw);
    // test drawing checkerboard
    //draw_checkboard(model, &draw);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

/// Draws cells based on if they are > 1
fn draw_scene(model: &Model, draw: &Draw) {
    let (row_width, column_width) = model.engine.get_grid_spacing();
    let (row_count, column_count) = model.engine.get_grid_dimensions();
    for row_number in 0..row_count {
        for column_number in 0..column_count {
            let (x, y) = convert_coordinates(row_number, column_number, model);
            if model.engine.get_cell(row_number, column_number) > 0 {
                draw.rect()
                    .color(BLACK)
                    .w(row_width)
                    .h(column_width)
                    .x_y(x, y);
            }
        }
    }
}

#[allow(dead_code)]
/// Simple test function that will print out a red and black checkerboard
fn draw_checkboard(model: &Model, draw: &Draw) {
    let (row_width, column_width) = model.engine.get_grid_spacing();
    let (row_count, column_count) = model.engine.get_grid_dimensions();
    for row_number in 0..row_count {
        for column_number in 0..column_count {
            let (x, y) = convert_coordinates(row_number, column_number, model);
            if (row_number % 2 == 0 && column_number % 2 == 0)
                || (row_number % 2 != 0 && column_number % 2 != 0)
            {
                draw.rect()
                    .color(BLACK)
                    .w(row_width)
                    .h(column_width)
                    .x_y(x, y);
            } else {
                draw.rect()
                    .color(RED)
                    .w(row_width)
                    .h(column_width)
                    .x_y(x, y);
            }
        }
    }
}

/// Given the row and column index, calculate the center draw position.
/// Return float values as (X, Y).
/// # Params
/// - row_index: usize, row index in the grid
/// - Column_index: usize, column index in the grid
/// - model: &Model, model that contains the engine and grid dimensions
/// # Returns
/// - (f32, f32), (X, Y) screen coordinates for the given grid cell
fn convert_coordinates(row_index: usize, column_index: usize, model: &Model) -> (f32, f32) {
    let lower_x = (model.window_width / 2.0) * -1.0;
    let lower_y = (model.window_height / 2.0) * -1.0;

    let (x_width, y_width) = model.engine.get_grid_spacing();
    let coordinate_x = lower_x + (column_index as f32 * x_width + x_width / 2.0);
    let coordinate_y = (-1.0 * lower_y) - (row_index as f32 * y_width + y_width / 2.0);
    (coordinate_x, coordinate_y)
}
