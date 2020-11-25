/// Handles a logical grid layout, each cell contains a ganeric type of data

pub struct Grid<T> {
    row_size: usize,
    column_size: usize,
    cells: Vec<T>,
}

impl<T: Copy + std::cmp::PartialOrd<usize>> Grid<T> {
    /// Creates a new Grid object.
    /// # Params
    /// - row_size, usize: size of the row
    /// - column_size, usize: size of the columns
    /// - init_data, T:  Initial state of the cells to be filled in
    /// # Returns
    /// - Grid<T>, a grid of the given dimensions with cells filled tihe init_data
    pub fn new(row_size: usize, column_size: usize, init_data: T) -> Grid<T> {
        let mut grid = Grid {
            row_size,
            column_size,
            cells: Vec::with_capacity(row_size * column_size),
        };

        // setup the data
        for _ in 0..grid.cells.capacity() {
            grid.cells.push(init_data.clone());
        }

        grid
    }

    /// Get the row and column sizes of the grid.
    /// # Returns
    /// (usize, usize), Tuple representing (row size, column size)
    pub fn size(&self) -> (usize, usize) {
        (self.row_size, self.column_size)
    }

    /// Simple cloning function.  Produces a brand new Grid that is identical to self.
    /// # Returns
    /// Grid<T>, identical Grid to self
    pub fn clone(&self) -> Grid<T> {
        Grid {
            row_size: self.row_size,
            column_size: self.column_size,
            cells: self.cells.to_vec(),
        }
    }

    /// Gets a specified element in the grid.  Will check row and column input ranges.
    /// # Params
    /// row, usize:  0 based row of the desired cell
    /// column, usize, 0 based column of the desired cell
    /// # Returns
    /// Result<T, &'static str>, returns a result with either the cell data or an error
    pub fn get(&self, row: usize, column: usize) -> Result<&T, &'static str> {
        // check inputs
        if row >= self.row_size {
            return Err("Get row is out of bounds");
        }
        if column >= self.column_size {
            return Err("Get column is out of bounds");
        }
        let data = self
            .cells
            .get(row * self.column_size + column)
            .expect("Failed to get data from grid");
        Ok(data)
    }

    /// Sets a cell with the given data.
    pub fn set(&mut self, row: usize, column: usize, data: T) -> Result<(), &'static str> {
        // check inputs
        if row >= self.row_size {
            return Err("Given row is out of grid bounds");
        }
        if column >= self.column_size {
            return Err("Given column is out of grid bounds");
        }
        self.cells[row * self.column_size + column] = data;
        Ok(())
    }

    /// This function will check all surrounding cells for living cells and return the number of cells around the given
    /// coordinates that have a value greater than 0.
    /// # Params
    /// - row_index: usize, row coordinate of center cell
    /// - column_index: usize, column coordinate of center cell
    /// # Return
    /// - Result<usize, &'static str>, either the number of living cells surrounding the coordinates, or an error string.
    pub fn get_number_of_neighbors(
        &self,
        row_index: usize,
        column_index: usize,
    ) -> Result<usize, &'static str> {
        // verify the inputs are valid
        if row_index > self.size().0 {
            return Err("Center row is out of bounds");
        } else if column_index > self.size().1 {
            return Err("Center column is out of bounds");
        }

        let mut number_of_neighbors = 0;
        // setup row range
        let row_min = if row_index == 0 { 0 } else { row_index - 1 };
        let row_max = if row_index + 1 >= self.size().0 {
            self.size().0 - 1
        } else {
            row_index + 1
        };
        // setup column range
        let column_min = if column_index == 0 {
            0
        } else {
            column_index - 1
        };
        let column_max = if column_index + 1 >= self.size().1 {
            self.size().1 - 1
        } else {
            column_index + 1
        };

        // loop through neighbor coordinates
        for neighbor_row_index in row_min..=row_max {
            for neighbor_column_index in column_min..=column_max {
                // skip center coordinate
                if neighbor_row_index == row_index && neighbor_column_index == column_index {
                    continue;
                }
                match self.get(neighbor_row_index, neighbor_column_index) {
                    Ok(cell_data) => {
                        if *cell_data > 0 {
                            number_of_neighbors += 1;
                        }
                    }
                    Err(_err) => (println!("{}", _err)), // skip over error, probably out of bounds
                };
            }
        }
        Ok(number_of_neighbors)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /// helper function for the unit tests
    fn setup_grid() -> Grid<usize> {
        Grid::new(5, 6, 0)
    }

    #[test]
    fn test_grid() {
        let grid = setup_grid();
        assert_eq!(grid.size(), (5, 6));
    }

    #[test]
    fn test_set_get() {
        let mut grid = setup_grid();
        // test set function
        for index in 0..4 {
            println!("{}", index);
            let result = grid.set(index, index, index);
            assert!(result.is_ok());
        }
        // test out of bounds
        let result = grid.set(6, 6, 6);
        assert!(result.is_err());

        // test get function
        for index in 0..4 {
            let result = grid.get(index, index);
            assert!(result.is_ok());
            assert_eq!(*result.unwrap(), index);
        }
        // test out of bounds
        let result = grid.get(6, 6);
        assert!(result.is_err());
    }

    #[test]
    fn test_number_neighbors() {
        let mut grid = Grid::new(3, 3, 0);
        let (row_size, column_size) = grid.size();
        for row_index in 0..row_size {
            for column_index in 0..column_size {
                if row_index % 2 == 1 {
                    let result = grid.set(row_index, column_index, 1);
                    assert!(result.is_ok());
                } else {
                    let result = grid.set(row_index, column_index, 0);
                    assert!(result.is_ok());
                }
            }
        }

        // print the array
        for row_index in 0..row_size {
            //let mut row = "".to_string();
            for column_index in 0..column_size {
                print!("{}", grid.get(row_index, column_index).unwrap());
            }
            println!();
        }

        let num_n = grid.get_number_of_neighbors(0, 0).unwrap();
        assert_eq!(num_n, 2);
        let num_n = grid.get_number_of_neighbors(0, 1).unwrap();
        assert_eq!(num_n, 3);
        let num_n = grid.get_number_of_neighbors(0, 2).unwrap();
        assert_eq!(num_n, 2);
        let num_n = grid.get_number_of_neighbors(1, 0).unwrap();
        assert_eq!(num_n, 1);
        let num_n = grid.get_number_of_neighbors(1, 1).unwrap();
        assert_eq!(num_n, 2);
        let num_n = grid.get_number_of_neighbors(1, 2).unwrap();
        assert_eq!(num_n, 1);
        let num_n = grid.get_number_of_neighbors(2, 0).unwrap();
        assert_eq!(num_n, 2);
        let num_n = grid.get_number_of_neighbors(2, 1).unwrap();
        assert_eq!(num_n, 3);
        let num_n = grid.get_number_of_neighbors(2, 2).unwrap();
        assert_eq!(num_n, 2);

        // access outside grid
        let num_n = grid.get_number_of_neighbors(4, 2);
        assert!(num_n.is_err());

        let num_n = grid.get_number_of_neighbors(2, 4);
        assert!(num_n.is_err());

        // uneven grid
        let mut grid = Grid::new(4, 4, 0);
        // row 0 empty
        // row 1 0,1,1,1
        let result = grid.set(1, 1, 1);
        assert!(result.is_ok());
        let result = grid.set(1, 2, 1);
        assert!(result.is_ok());
        let result = grid.set(1, 3, 1);
        assert!(result.is_ok());
        // row 2 1,1,1,0
        let result = grid.set(2, 0, 1);
        assert!(result.is_ok());
        let result = grid.set(2, 1, 1);
        assert!(result.is_ok());
        let result = grid.set(2, 2, 1);
        assert!(result.is_ok());
        //row 3 empty
        // Run through all cells for neighbors
        // Row 1
        let num_n = grid.get_number_of_neighbors(0, 0);
        assert!(num_n.is_ok());
        assert_eq!(1, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(0, 1);
        assert!(num_n.is_ok());
        assert_eq!(2, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(0, 2);
        assert!(num_n.is_ok());
        assert_eq!(3, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(0, 3);
        assert!(num_n.is_ok());
        assert_eq!(2, num_n.unwrap());
        // row 2
        let num_n = grid.get_number_of_neighbors(1, 0);
        assert!(num_n.is_ok());
        assert_eq!(3, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(1, 1);
        assert!(num_n.is_ok());
        assert_eq!(4, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(1, 2);
        assert!(num_n.is_ok());
        assert_eq!(4, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(1, 3);
        assert!(num_n.is_ok());
        assert_eq!(2, num_n.unwrap());
        // row 3
        let num_n = grid.get_number_of_neighbors(2, 0);
        assert!(num_n.is_ok());
        assert_eq!(2, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(2, 1);
        assert!(num_n.is_ok());
        assert_eq!(4, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(2, 2);
        assert!(num_n.is_ok());
        assert_eq!(4, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(2, 3);
        assert!(num_n.is_ok());
        assert_eq!(3, num_n.unwrap());
        // row 4
        let num_n = grid.get_number_of_neighbors(3, 0);
        assert!(num_n.is_ok());
        assert_eq!(2, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(3, 1);
        assert!(num_n.is_ok());
        assert_eq!(3, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(3, 2);
        assert!(num_n.is_ok());
        assert_eq!(2, num_n.unwrap());
        let num_n = grid.get_number_of_neighbors(3, 3);
        assert!(num_n.is_ok());
        assert_eq!(1, num_n.unwrap());
    }
}
