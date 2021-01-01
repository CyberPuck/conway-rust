#[macro_use]
extern crate clap;
mod gui;

fn main() {
    // handle CLI args
    let yaml = load_yaml!("cli.yml");
    // try setting the name, version
    let cli_app = clap::App::from_yaml(yaml);
    let matches = cli_app
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .get_matches();

    // read in height and width, deafult is 1024 x 768
    let height = matches
        .value_of("height")
        .unwrap_or("768.0")
        .parse::<f32>()
        .expect("Failed to parse height argument");

    let width = matches
        .value_of("width")
        .unwrap_or("1024.0")
        .parse::<f32>()
        .expect("Failed to parse width argument");

    // read in the update rate
    let update_rate = matches
        .value_of("rate")
        .unwrap_or("1")
        .parse::<usize>()
        .expect("Failed to parse rate argument");

    // read in the number of steps
    let number_of_steps = matches
        .value_of("steps")
        .unwrap_or("20")
        .parse::<usize>()
        .expect("Failed to parse number of steps argument");

    // read in the alive color
    // NOTE: All colors must be in lowercase to be parsed by the palette crate
    let alive_color = matches
        .value_of("alive")
        .unwrap_or("BLACK")
        .to_ascii_lowercase();

    // read in the dead color
    // NOTE: All colors must be in lowercase to be parsed by the palette crate
    let dead_color = matches
        .value_of("dead")
        .unwrap_or("WHITE")
        .to_ascii_lowercase();

    // read in the game file, default is empty (which will generate a default oscillator)
    let file_location = matches.value_of("file").unwrap_or("");

    let enable_grid = matches.is_present("grid");

    // Call the GUI class (empty struct with functions) to start the application
    gui::GUI::start(
        file_location.to_string(),
        number_of_steps,
        update_rate,
        height,
        width,
        alive_color.to_string(),
        dead_color.to_string(),
        enable_grid,
    );
}
