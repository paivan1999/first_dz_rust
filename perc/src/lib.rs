#![forbid(unsafe_code)]

extern crate rand;

use std::io::stdin;

use rand::random;

use rand::Rng;

#[derive(Debug)]
////////////////////////////////////////////////////////////////////////////////

/// Represents a grid of boolean values.
pub struct BoolGrid {
    width: usize,
    height: usize,
    data: Vec<Vec<bool>>,
}

impl BoolGrid {
    /// Creates a new grid with all values initialized as `false`.
    ///
    /// # Arguments
    ///
    /// * `width` - grid width.
    /// * `height` - grid height.
    pub fn new(width: usize, height: usize) -> Self {
        // TODO: your code here.
        Self {
            width,
            height,
            data: vec![vec![false; height]; width],
        }
    }

    /// Creates a new grid with every value initialized randomly.
    ///
    /// # Arguments
    ///
    /// * `width` - grid width.
    /// * `height` - grid height.
    /// * `vacancy` - probability of any given value being equal
    /// to `false`.
    pub fn random(width: usize, height: usize, vacancy: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut data = vec![vec![false; height]; width];
        for row in data.iter_mut() {
            for el in row.iter_mut() {
                *el = rng.gen::<f64>() >= vacancy;
            }
        }
        Self {
            width,
            height,
            data,
        }
    }

    /// Returns grid width.
    pub fn width(&self) -> usize {
        // TODO: your code here.
        self.width
    }

    /// Returns grid height.
    pub fn height(&self) -> usize {
        // TODO: your code here.
        self.height
    }

    /// Returns the current value of a given cell.
    /// The caller must ensure that `x` and `y` are valid.
    ///
    /// # Arguments
    ///
    /// * `x` - must be >= 0 and < grid width.
    /// * `y` - must be >= 0 and < grid height.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is out of bounds, this method may panic
    /// (or return incorrect result).
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.data[x][y]
    }

    /// Sets a new value to a given cell.
    /// The caller must ensure that `x` and `y` are valid.
    ///
    /// # Arguments
    ///
    /// * `x` - must be >= 0 and < grid width.
    /// * `y` - must be >= 0 and < grid height.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is out of bounds, this method may panic
    /// (or set value to some other unspecified cell).
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.data[x][y] = value;
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Returns `true` if the given grid percolates. That is, if there is a path
/// from any cell with `y` == 0 to any cell with `y` == `height` - 1.
/// If the grid is empty (`width` == 0 or `height` == 0), it percolates.
pub fn percolates(grid: &BoolGrid) -> bool {
    // TODO: your code here.
    let w = grid.width();
    let h = grid.height();
    if w == 0 || h == 0 {
        return true;
    }
    let mut lowest: usize = 0;
    pub fn fill(
        grid: &BoolGrid,
        visited: &mut BoolGrid,
        start_x: usize,
        start_y: usize,
        lowest: &mut usize,
    ) {
        let w = grid.width();
        let h = grid.height();
        if !visited.data[start_x][start_y] && !grid.data[start_x][start_y] {
            if start_y > *lowest {
                *lowest = start_y
            }
            visited.data[start_x][start_y] = true;
            let mut good_neibours: Vec<(usize, usize)> = Vec::new();
            if start_x > 0 {
                good_neibours.push((start_x - 1, start_y));
            }
            if start_x < w - 1 {
                good_neibours.push((start_x + 1, start_y));
            }
            if start_y > 0 {
                good_neibours.push((start_x, start_y - 1));
            }
            if start_y < h - 1 {
                good_neibours.push((start_x, start_y + 1));
            }
            for (x, y) in good_neibours.iter() {
                fill(grid, visited, *x, *y, lowest);
            }
        }
    }
    let mut visited: &mut BoolGrid = &mut BoolGrid::new(w, h);
    for x in 0..w {
        fill(grid, visited, x, 0, &mut lowest);
    }
    lowest == h - 1
}

////////////////////////////////////////////////////////////////////////////////

const N_TRIALS: u64 = 10000;

/// Returns an estimate of the probability that a random grid with given
/// `width, `height` and `vacancy` probability percolates.
/// To compute an estimate, it runs `N_TRIALS` of random experiments,
/// in each creating a random grid and checking if it percolates.
pub fn evaluate_probability(width: usize, height: usize, vacancy: f64) -> f64 {
    let mut perc_count = 0;
    for _ in 0..N_TRIALS {
        let grid = BoolGrid::random(width, height, vacancy);
        if percolates(&grid) {
            perc_count += 1;
        }
    }
    //sprintln!("{}",perc_count as f64 / N_TRIALS as f64);
    return perc_count as f64 / N_TRIALS as f64;
}
