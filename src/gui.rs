/// Handles drawing and updating the GUI.
/// This allows for CLI parameters to be fed in a read from Nannou's model function.
#[path = "conway_engine.rs"]
mod conway_engine;
use nannou::color::named;
use nannou::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy)]
struct ConfigParams {
    file_name: &'static str,
    number_of_steps: usize,
    update_rate: usize,
    // TODO: height and width are added during construction, are these fields needed globally?
    height: u32,
    width: u32,
    // TODO:  Need to convert CLI params to Nannou color
    alive_color: nannou::color::rgb::Srgb<u8>,
    dead_color: nannou::color::rgb::Srgb<u8>,
    enable_grid: bool,
}

// Empty struct, needed to expose start function
pub struct GUI {}

// set the global params to default values
// NOTE:  Needed since the ```model``` function can't take extra parameters.
static mut GLOBAL_PARAMS: ConfigParams = ConfigParams {
    file_name: "",
    number_of_steps: 20,
    update_rate: 1,
    height: 768,
    width: 1024,
    alive_color: BLACK,
    dead_color: WHITE,
    enable_grid: false,
};

struct Model {
    engine: conway_engine::ConwayEngine,
    window_height: f32,
    window_width: f32,
    time: Duration,
    params: ConfigParams,
}

impl GUI {
    /// Start the GUI up with the given parameters.
    /// # NOTE
    /// This function will take over the main thread calling it and not exit (it's running the GUI after all).
    /// # Params
    /// - file_name: String, location of file to load
    /// - number_of_steps: usize, number of steps for simulation to take; 0 is infinite
    /// - update_rate: usize, in seconds how long between each simulation step
    /// - height: u32, height of window GUI in pixels
    /// - width: u32, width of window GUI in pixels
    /// - alive_color: String, representation of the expected color of the living cells
    /// - dead_color: String, representation of the expected color of the dead cells
    /// - enable_grid: bool, flag indicating if the grid should be drawn
    pub fn start(
        file_name: String,
        number_of_steps: usize,
        update_rate: usize,
        height: u32,
        width: u32,
        alive_color: String,
        dead_color: String,
        enable_grid: bool,
    ) {
        // Since the GUI application is static (we intend for the GUI to be up for the duration of the program), we need to copy
        // the String to a String with a 'static lifetime
        let copy_file_name: &'static str = Box::leak(file_name.into_boxed_str());

        let alive_color = match named::from_str(&alive_color) {
            Some(color) => color,
            None => BLACK,
        };

        let dead_color = match named::from_str(&dead_color) {
            Some(color) => color,
            None => WHITE,
        };

        // Updating static data for model access
        unsafe {
            GLOBAL_PARAMS.file_name = &copy_file_name;
            GLOBAL_PARAMS.number_of_steps = number_of_steps;
            GLOBAL_PARAMS.update_rate = update_rate;
            GLOBAL_PARAMS.height = height;
            GLOBAL_PARAMS.width = width;
            GLOBAL_PARAMS.alive_color = alive_color;
            GLOBAL_PARAMS.dead_color = dead_color;
            GLOBAL_PARAMS.enable_grid = enable_grid;
        }

        // start the GUI application
        nannou::app(GUI::model)
            .size(height, width)
            .update(GUI::update)
            .run();
    }

    /// Create the model for the Nannou GUI.  This will also read in the GLOBAL_PARAMS static mut config object.
    /// Static global config object is needed to feed in data from the CLI options entered during start up.
    /// # NOTE
    /// ```unsafe``` code is used in this function to get the static mut global configuration object.
    /// # Params
    /// app: &App, reference to the Nannou App object (primary object that represents the GUI)
    /// # Returns
    /// Model, Model object that contains the business state of the GUI.
    fn model(app: &App) -> Model {
        // }:)  unsafe saves the day, since GLOBAL_PARMS or its mutable data might be garbage
        // NOTE:  Feel like I'm making a noob mistake having to declare unsafe here
        unsafe {
            // generate the window title
            let name = if GLOBAL_PARAMS.file_name.len() == 0 {
                format!(
                    "Conway-Rust v{} No file found, using default pattern",
                    crate_version!()
                )
            } else {
                format!(
                    "Conway-Rust v{} File - {}",
                    crate_version!(),
                    GLOBAL_PARAMS.file_name
                )
            };

            // add a window to the view
            app.new_window()
                .title(name)
                .view(GUI::view)
                .build()
                .unwrap();

            // Get the dimensions of the main window
            let window_data = app.main_window().rect();
            // setup the game
            let engine = conway_engine::ConwayEngine::new(
                &GLOBAL_PARAMS.file_name.to_string(),
                window_data.h(),
                window_data.w(),
                GLOBAL_PARAMS.update_rate,
                GLOBAL_PARAMS.number_of_steps,
            );

            // return the model
            Model {
                engine,
                window_height: window_data.h(),
                window_width: window_data.w(),
                time: Duration::new(0, 0),
                params: GLOBAL_PARAMS,
            }
        }
    }

    fn update(_app: &App, model: &mut Model, _update: Update) {
        // use _update.since_last as how long it has been since last step
        model.time += _update.since_last;
        if model.time > model.engine.get_update_rate_duration() {
            model.engine.take_step();
            model.time = Duration::new(0, 0);
        }
    }

    fn view(app: &App, model: &Model, frame: Frame) {
        // get canvas to draw on
        let draw = app.draw();

        // set background to blue
        draw.background().color(model.params.dead_color);

        // Draw the scene
        GUI::draw_scene(model, &draw);
        // drawing grid
        //if(model.params.enable_grid)
        //GUI::draw_grid(model, &draw);

        // put everything on the frame
        draw.to_frame(app, &frame).unwrap();
    }

    /// Draws cells based on if they are > 1
    fn draw_scene(model: &Model, draw: &Draw) {
        let (row_width, column_width) = model.engine.get_grid_spacing();
        let (row_count, column_count) = model.engine.get_grid_dimensions();
        for row_number in 0..row_count {
            for column_number in 0..column_count {
                let (x, y) = GUI::convert_coordinates(row_number, column_number, model);
                if model.engine.get_cell(row_number, column_number) > 0 {
                    draw.rect()
                        .color(model.params.alive_color)
                        .w(row_width - 1.0)
                        .h(column_width - 1.0)
                        .x_y(x + 0.5, y + 0.5);
                }
            }
        }
    }

    #[allow(dead_code)]
    /// Simple test function that will print out a red and black checkerboard.
    /// This is a flagged option allowing users to see individual cells if all are dead or alive.
    fn draw_grid(model: &Model, draw: &Draw) {
        let (row_width, column_width) = model.engine.get_grid_spacing();
        let (row_count, column_count) = model.engine.get_grid_dimensions();
        for row_number in 0..row_count {
            for column_number in 0..column_count {
                let (x, y) = GUI::convert_coordinates(row_number, column_number, model);
                if (row_number % 2 == 0 && column_number % 2 == 0)
                    || (row_number % 2 != 0 && column_number % 2 != 0)
                {
                    draw.rect()
                        .color(BLACK)
                        .w(row_width - 1.0)
                        .h(column_width - 1.0)
                        .x_y(x + 0.5, y + 0.5);
                } else {
                    draw.rect()
                        .color(RED)
                        .w(row_width - 1.0)
                        .h(column_width - 1.0)
                        .x_y(x + 0.5, y + 0.5);
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
}
