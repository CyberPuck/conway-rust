/// Engine for running Conway's Game of Life
use super::grid;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

pub struct ConwayEngine {
    grid: grid::Grid<usize>,
    height: f32,
    width: f32,
    update_rate: usize,
    number_of_steps: usize,
}

impl ConwayEngine {
    pub fn new(filename: &String, height: f32, width: f32) -> ConwayEngine {
        // read the file
        let mut file_data = read_engine_file(filename).expect("Failed to read in engine file");
        // parse the header
        let header_data =
            parse_header(file_data.remove(0)).expect("Failed to parse engine file header");
        // store elements to variables
        let row_size = header_data.0;
        let column_size = header_data.1;
        let update_rate = header_data.2;
        let number_of_steps = header_data.3;
        // generate the grid
        let grid = generate_grid(
            row_size,
            column_size,
            file_data[0..file_data.len()].to_vec(),
        )
        .expect("Failed to generate the grid");

        ConwayEngine {
            grid,
            height,
            width,
            update_rate,
            number_of_steps,
        }
    }

    /// Take a step in the simulation.
    /// This is where the rules of the game are applied to the application.
    pub fn take_step(&mut self) {
        // do not step forward if there are no more steps to take
        if self.get_number_of_steps() <= 0 {
            return;
        }

        // Generate new grid to fill in next steps
        let mut next_grid = self.grid.clone();
        let (row_size, column_size) = self.grid.size();
        for row_index in 0..row_size {
            for column_index in 0..column_size {
                let number_of_neighbors = self
                    .grid
                    .get_number_of_neighbors(row_index, column_index)
                    .expect("Failed to get the number of neighbors");
                let cell_status = self
                    .grid
                    .get(row_index, column_index)
                    .expect("Failed to get cell");
                if (number_of_neighbors < 2 || number_of_neighbors > 3) && *cell_status == 1 {
                    next_grid
                        .set(row_index, column_index, 0)
                        .expect("Failed to kill cell");
                } else if number_of_neighbors == 3 && *cell_status == 0 {
                    next_grid
                        .set(row_index, column_index, 1)
                        .expect("Failed to create cell");
                }
            }
        }
        // swap grids
        self.grid = next_grid;
        self.number_of_steps -= 1;
    }

    /// Based on update_rate, return a duration.
    /// Whole numbers of update_rate is seconds.
    /// Decimal of update_rate is milliseconds.
    /// # Equation
    /// Duration::new(update_rate, 0)
    /// # Returns
    /// Duration, self.update_rate as a duration
    pub fn get_update_rate_duration(&self) -> Duration {
        Duration::new(self.update_rate as u64, 0)
    }

    pub fn get_number_of_steps(&self) -> usize {
        self.number_of_steps
    }

    /// Calculate the spacing between rows and columns.
    /// The maths: (self.width / self.grid.column_size, self.height / self.grid.row_size)
    /// # Returns
    /// (f32, f32), (X spacing, Y spacing)
    pub fn get_grid_spacing(&self) -> (f32, f32) {
        (
            self.width / self.grid.size().1 as f32,
            self.height / self.grid.size().0 as f32,
        )
    }

    /// Get the row and column count for the grid
    /// # Returns
    /// (usize, usize), (row_size, column_size)
    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.grid.size()
    }

    /// Get the state of a cell.
    /// Returns either the cell contents or 0 on an error.
    /// # Params
    /// row_index: usize, row index in the engine grid.
    /// column_index: usize, column index in the engine grid.
    /// # Returns
    /// usize, Cell state or 0
    pub fn get_cell(&self, row_index: usize, column_index: usize) -> usize {
        match self.grid.get(row_index, column_index) {
            Ok(data) => *data,
            Err(_err) => 0,
        }
    }

    /// Replace the existing grid with a new grid.
    /// This is for changing the grid with each new step.  The rules of the game make the grid change
    /// all at once.  In order to accomplish changing earlier cells, a new grid is created representing
    /// the new state.  The old state needs to be overwritten and removed.
    /// # Params
    /// - new_grid: grid::Grid<usize>, the modified replacement grid
    /// # Returns
    /// - Result<(), &'static str>, result, empty OK if sucessful, err if the grids don't match in size.
    fn replace_grid(&mut self, new_grid: grid::Grid<usize>) -> Result<(), &'static str> {
        if self.grid.size().0 != new_grid.size().0 || self.grid.size().1 != new_grid.size().1 {
            return Err("Grids do not match in size");
        }
        // update the grid
        self.grid = new_grid;

        Ok(())
    }
}

/// Reads an input file, and produces a Grid object that can be used by the engine.
/// # File format
/// - First line is comma delimited header "row size, column size"
/// - There must be at least X+1 row size lines in the file.
/// - Each line must be at least column size (comma delimited).
/// - Each entry must be either a "0" or "1"
/// ## Ignored data
/// - Lines starting with '#' will be skipped as comments
/// - Any extra data will be ignored and not parsed.
/// ## Error conditions
/// - There are not enough lines to satisfy row size + 1
/// - A column is too small
/// - Cell entry is not a "0" or "1"
/// This function is static, no need to reference the struct.
/// # Params
/// filename: &String, the input file to generate the intial grid and engine state
/// # Returns
/// Result<grid::Grid<usize>, &'static str>, Grid on success, an error string if reading the file fails.
fn read_engine_file(filename: &String) -> Result<Vec<String>, &'static str> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_err) => return Err("Failed to open file"),
    };

    let line_iter = BufReader::new(file).lines();
    let mut file_data: Vec<String> = Vec::new();
    for line in line_iter {
        let line = match line {
            Ok(data) => data,
            Err(_err) => return Err("Failed to get line in file"),
        };
        // skip comments
        if !line.starts_with("#") {
            file_data.push(line);
        }
    }
    Ok(file_data)
}

/// Given a header array of strings, parse out:
/// - Row size
/// - Column size
/// - update rate
/// - number of steps
/// # Params
/// header_line: String, the raw header line from the file.
/// # Returns
/// (usize, usize, usize, usize), tuple containing: row size, column size, update rate, number of steps
fn parse_header(header_line: String) -> Result<(usize, usize, usize, usize), &'static str> {
    let header_data: Vec<&str> = header_line.split(',').collect();
    if header_data.len() < 4 {
        return Err("Parse error, header is too small.  Row and column size,  need.");
    }
    let row_size = header_data[0]
        .trim()
        .parse::<usize>()
        .expect("Failed to parse row size");
    let column_size = header_data[1]
        .trim()
        .parse::<usize>()
        .expect("Failed to parse column size");
    let update_rate = header_data[2]
        .trim()
        .parse::<usize>()
        .expect("Failed to parse update rate");
    let number_of_steps = header_data[3]
        .trim()
        .parse::<usize>()
        .expect("Failed to parse number of steps");

    Ok((row_size, column_size, update_rate, number_of_steps))
}

/// Generate the grid for the engine
/// # Params
/// - row_size: usize, size of rows
/// - column_size: usize, size of columns
/// - grid_lines: Vec<String>, vector of strings to be parsed for a row of grid data
/// # Returns
/// - Result<Grid<T>, str>, either a successfully created grid is returned or an error
fn generate_grid(
    row_size: usize,
    column_size: usize,
    grid_lines: Vec<String>,
) -> Result<grid::Grid<usize>, &'static str> {
    // setup the grid
    let mut grid = grid::Grid::new(row_size, column_size, 0);
    // parse the data, add to grid cells
    let mut row_counter = 0;
    for line in grid_lines {
        // check row bounds
        if row_counter > grid.size().0 {
            return Err("Row exceeds the grid");
        }
        let line_data: Vec<&str> = line.split(',').collect();
        // check column bounds
        if line_data.len() > grid.size().1 {
            return Err("Column is too large to fit in the grid");
        }
        for column_counter in 0..line_data.len() {
            let data = match line_data[column_counter].trim().parse::<usize>() {
                Ok(data) => data,
                Err(_err) => return Err("failed to parse cell data"),
            };
            match grid.set(row_counter, column_counter, data) {
                Ok(()) => (),
                Err(_err) => return Err("Failed to set grid cell"),
            };
        }
        row_counter += 1;
    }

    Ok(grid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let engine = ConwayEngine::new(&"test-files/test2.txt".to_string(), 768.0, 1024.0);
        for row in 0..engine.get_grid_dimensions().0 {
            let mut row_s: String = "".to_string();
            for column in 0..engine.get_grid_dimensions().1 {
                row_s += &engine.get_cell(row, column).to_string();
            }
            println!("{}", row_s);
        }
        assert_eq!(engine.get_cell(1, 2), 1);
        assert_eq!(engine.get_cell(2, 2), 1);
        assert_eq!(engine.get_cell(3, 2), 1);
    }

    #[test]
    fn test_read_engine_file() {
        let result = read_engine_file(&"test-files/test.txt".to_string());
        assert!(result.is_ok());
        let test_file_one: Vec<String> = vec![
            "5, 5, 1, 20".to_string(),
            "1,0,0,0,0".to_string(),
            "0,1,0,0,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,0,1,0".to_string(),
            "0,0,0,0,1".to_string(),
        ];
        assert_eq!(result.unwrap(), test_file_one);

        let result = read_engine_file(&"test-files/bad_test.txt".to_string());
        assert!(result.is_ok());
        let test_file_bad: Vec<String> = vec![
            "5, 5".to_string(),
            "1,0,0,0,0".to_string(),
            "0,1,0,a,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,0,1,c".to_string(),
        ];
        assert_eq!(result.unwrap(), test_file_bad);

        let result = read_engine_file(&"test-files/test2.txt".to_string());
        assert!(result.is_ok());
        let test_file_two: Vec<String> = vec![
            "5, 5, 1, 20".to_string(),
            "0,0,0,0,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,0,0,0".to_string(),
        ];
        assert_eq!(result.unwrap(), test_file_two);

        let result = read_engine_file(&"random_file_name.hello_world".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_header() {
        let data = parse_header("5, 5, 5, 5".to_string());
        assert!(data.is_ok());
        let data = data.unwrap();
        assert_eq!(data, (5, 5, 5, 5));

        let data = parse_header("1, 2, 3, 4, 6".to_string());
        assert!(data.is_ok());
        let data = data.unwrap();
        assert_eq!(data, (1, 2, 3, 4));

        let data = parse_header("5, 5, 20, 1".to_string());
        assert!(data.is_ok());
        let data = data.unwrap();
        assert_eq!(data, (5, 5, 20, 1));

        let data = parse_header("5, 5, 5".to_string());
        assert!(data.is_err());
    }

    #[test]
    fn test_empty_generated_grid() {
        let test_grid: Vec<String> = Vec::new();
        let grid = generate_grid(5, 5, test_grid);
        assert!(grid.is_ok());
        let grid = grid.unwrap();
        let test_grid_cells: Vec<usize> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        for row_index in 0..grid.size().0 {
            for column_index in 0..grid.size().1 {
                assert_eq!(
                    grid.get(row_index, column_index).unwrap(),
                    &test_grid_cells[row_index * grid.size().1 + column_index]
                );
            }
        }
    }

    #[test]
    fn test_generate_grid() {
        let test_grid: Vec<String> = vec![
            "1,0,0,0,0".to_string(),
            "0,1,0,0,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,0,1,0".to_string(),
            "0,0,0,0,1".to_string(),
        ];
        let test_grid_cells: Vec<usize> = vec![
            1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
        ];
        let grid = generate_grid(5, 5, test_grid);
        assert!(grid.is_ok());
        let grid = grid.unwrap();
        for row_index in 0..grid.size().0 {
            for column_index in 0..grid.size().1 {
                assert_eq!(
                    grid.get(row_index, column_index).unwrap(),
                    &test_grid_cells[row_index * grid.size().1 + column_index]
                );
            }
        }

        let test_grid: Vec<String> = vec![
            "1,0,0,0,0".to_string(),
            "0,1,0,0,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,0,1,0".to_string(),
            "0,0,0,0,1".to_string(),
        ];
        let grid = generate_grid(4, 5, test_grid);
        assert!(grid.is_err());

        let test_grid: Vec<String> = vec![
            "1,0,0,0,0".to_string(),
            "0,1,0,0,0".to_string(),
            "0,0,1,0,0".to_string(),
            "0,0,0,1,0".to_string(),
            "0,0,0,0,1".to_string(),
        ];
        let grid = generate_grid(5, 4, test_grid);
        assert!(grid.is_err());

        let test_grid: Vec<String> = vec![
            "1,0,0,0,0".to_string(),
            "0,1,0,0,0".to_string(),
            "0,0,a,0,0".to_string(),
            "0,0,0,1,0".to_string(),
            "0,0,0,0,1".to_string(),
        ];
        let grid = generate_grid(5, 5, test_grid);
        assert!(grid.is_err());
    }

    #[test]
    fn test_get_grid_spacing() {
        let engine = ConwayEngine::new(&"test-files/test2.txt".to_string(), 768.0, 1024.0);
        let (x_width, y_width) = engine.get_grid_spacing();
        assert_eq!(x_width, 204.8);
        assert_eq!(y_width, 153.6);

        let engine = ConwayEngine::new(&"test-files/test3.txt".to_string(), 768.0, 1024.0);
        let (x_width, y_width) = engine.get_grid_spacing();
        assert_eq!(x_width, 170.66667);
        assert_eq!(y_width, 153.6);
    }
}
