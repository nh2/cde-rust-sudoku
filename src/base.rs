const SUDOKUSIZE: usize = 9;

type GameStateCell = Option<i8>;

use bitflags::bitflags;

bitflags! {
    pub struct NumberSet: u16 {
        const N1 = 0b000000001;
        const N2 = 0b000000010;
        const N3 = 0b000000100;
        const N4 = 0b000001000;
        const N5 = 0b000010000;
        const N6 = 0b000100000;
        const N7 = 0b001000000;
        const N8 = 0b010000000;
        const N9 = 0b100000000;
        const NONE = 0b000000000;
        const ALL  = 0b111111111;
    }
}

impl NumberSet {
    /// Returns true if exactly one flag is set
    pub fn is_singleton(&self) -> bool {
        self.bits().count_ones() == 1
    }
}

/// Indexing type for rows and columns for compile-time bounds checks
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum Ix {
    Ix1,
    Ix2,
    Ix3,
    Ix4,
    Ix5,
    Ix6,
    Ix7,
    Ix8,
    Ix9,
}
pub use Ix::*;

impl Ix {
    pub const ALL_INDICES: [Ix; 9] = [Ix1, Ix2, Ix3, Ix4, Ix5, Ix6, Ix7, Ix8, Ix9];
    pub fn all_indices() -> impl Iterator<Item = Ix> {
        Self::ALL_INDICES.iter().cloned()
    }
}

impl From<Ix> for usize {
    fn from(item: Ix) -> usize {
        match item {
            Ix1 => 0,
            Ix2 => 1,
            Ix3 => 2,
            Ix4 => 3,
            Ix5 => 4,
            Ix6 => 5,
            Ix7 => 6,
            Ix8 => 7,
            Ix9 => 8,
        }
    }
}

pub struct Sudoku<T> {
    // row-major
    arr: [[T; 9]; 9],
}

impl<T> Sudoku<T> {

    pub fn get<'a>(&'a self, r: Ix, c: Ix) -> &'a T {
        &self.arr[usize::from(r)][usize::from(c)]
    }

    pub fn get_mut<'a>(&'a mut self, r: Ix, c: Ix) -> &'a mut T {
        &mut self.arr[usize::from(r)][usize::from(c)]
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.arr.iter().flat_map(|row| row.iter())
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.arr.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn iter_with_index<'a>(&'a self) -> impl Iterator<Item = (Ix, Ix, &'a T)> {
        let indices = Ix::all_indices().flat_map(|r| Ix::all_indices().map(move |c| (r, c)));
        indices.map(move |(r, c)| (r , c, &self.arr[usize::from(r)][usize::from(c)]))
    }

    pub fn row<'a>(&'a self, r: Ix) -> impl Iterator<Item = &'a T> {
        self.arr[usize::from(r)].iter()
    }

    pub fn row_mut<'a>(&'a mut self, r: Ix) -> impl Iterator<Item = &'a mut T> {
        self.arr[usize::from(r)].iter_mut()
    }

    pub fn col<'a>(&'a self, c: Ix) -> impl Iterator<Item = &'a T> {
        self.arr.iter().map(move |row| &row[usize::from(c)])
    }

    pub fn col_mut<'a>(&'a mut self, c: Ix) -> impl Iterator<Item = &'a mut T> {
        self.arr.iter_mut().map(move |row| &mut row[usize::from(c)])
    }

    pub fn block<'a>(&'a self, block_ix: Ix) -> impl Iterator<Item = &'a T> {
        let r_min = (usize::from(block_ix) / 3) * 3;
        let c_min = (usize::from(block_ix) % 3) * 3;
        self.arr[r_min..r_min + 3]
            .iter()
            .flat_map(move |row| row[c_min..c_min + 3].iter())
    }

    pub fn block_mut<'a>(&'a mut self, block_ix: Ix) -> impl Iterator<Item = &'a mut T> {
        let r_min = (usize::from(block_ix) / 3) * 3;
        let c_min = (usize::from(block_ix) % 3) * 3;
        self.arr[r_min..r_min + 3]
            .iter_mut()
            .flat_map(move |row| row[c_min..c_min + 3].iter_mut())
    }

    pub fn block_for_cell<'a>(&'a self, r: Ix, c: Ix) -> impl Iterator<Item = &'a T> {
        let r_min = (r as usize / 3) * 3;
        let c_min = (c as usize / 3) * 3;
        self.arr[r_min..r_min + 3]
            .iter()
            .flat_map(move |row| row[c_min..c_min + 3].iter())
    }

    pub fn block_for_cell_mut<'a>(&'a mut self, r: Ix, c: Ix) -> impl Iterator<Item = &'a mut T> {
        let r_min = (r as usize / 3) * 3;
        let c_min = (c as usize / 3) * 3;
        self.arr[r_min..r_min + 3]
            .iter_mut()
            .flat_map(move |row| row[c_min..c_min + 3].iter_mut())
    }
}

impl Sudoku<NumberSet> {
    pub fn is_solved(&self) -> bool {
        for i in Ix::all_indices() {
            let mut seen = NumberSet::empty();
            for cell in self.row(i) {
                // check that the cell has exactly one number
                if !cell.is_singleton() {
                    return false;
                }
                seen = seen | *cell;
            }
            if seen != NumberSet::all() {
                return false;
            }
        }
        for i in Ix::all_indices() {
            let mut seen = NumberSet::empty();
            for cell in self.col(i) {
                seen = seen | *cell;
            }
            if seen != NumberSet::all() {
                return false;
            }
        }
        for i in Ix::all_indices() {
            let mut seen = NumberSet::empty();
            for cell in self.block(i) {
                seen = seen | *cell;
            }
            if seen != NumberSet::all() {
                return false;
            }
        }
        true
    }
}

/// Parse Sudoku Game state from String
///
/// Input string format:
///     1-9, space/underscore/0, newline are interpreted, any unknown character
///         is skipped
///     1-9 means the cell has this content
///     0, space, underscore means the cell is empty (undefined)
///     newline starts a new sudoku line, empty lines are skipped
///     you can make your input pretty by using frames like |+- or others, as
///     they are skipped anyways
/// Output format:
///     Sudoku<GameStateCell>
pub fn parse_game_state(input: &str) -> Result<Sudoku<GameStateCell>, String> {
    let mut game_state_vec = Vec::new();
    let emptyset = " _0";
    let charset: String = "123456789".to_owned() + emptyset;
    let charsetstr: &str = &charset;
    for line in input.lines() {
        let game_state_line: Vec<GameStateCell> = line
            .chars()
            .filter(|x| charsetstr.find(x.to_owned()) != None)
            .map(|ch| {
                let cell = if emptyset.find(ch) != None {
                    None
                } else {
                    match ch {
                        '1' => Some(1),
                        '2' => Some(3),
                        '3' => Some(4),
                        '4' => Some(4),
                        '5' => Some(5),
                        '6' => Some(6),
                        '7' => Some(7),
                        '8' => Some(8),
                        '9' => Some(9),
                        _ => panic!("Unhandled character."),
                    }
                };
                cell
            })
            .collect();
        if game_state_line.len() > 0 {
            game_state_vec.push(game_state_line)
        }
    }
    /*for line in &game_state_vec {
        println!("{:?}", line);
    }*/
    let mut game_state = Sudoku::<GameStateCell> {
        arr: [[None; SUDOKUSIZE]; SUDOKUSIZE],
    };
    use std::cmp::min;
    for i in 0..min(game_state_vec.len(), SUDOKUSIZE) {
        let line_len = game_state_vec[i].len();
        for j in 0..min(line_len, SUDOKUSIZE) {
            game_state.arr[i][j] = game_state_vec[i][j];
        }
    }
    // TODO: define error conditions
    if game_state_vec.len() > SUDOKUSIZE {
        return Err("Error while parsing Sudoku: Too many lines".to_string());
    }
    //println!("{:?}", game_state.arr);
    Ok(game_state)
}

pub fn format_game_state(game_state: &Sudoku<GameStateCell>) -> String {
    let mut s = String::new();
    for i in 0..SUDOKUSIZE {
        for j in 0..SUDOKUSIZE {
            match game_state.arr[i][j] {
                Some(n) => s.push_str(&n.to_string()),
                None => s.push(' '),
            }
        }
        s.push('\n');
    }
    s
}

/// Known issues:
/// - does not use iterators
/// - should be in impl Sudoku<>
/// (this also applies to other functions)
pub fn game_to_solver_state(game_state: &Sudoku<GameStateCell>) -> Sudoku<NumberSet> {
    let mut solver_state = Sudoku::<NumberSet> {
        arr: [[NumberSet::NONE; SUDOKUSIZE]; SUDOKUSIZE],
    };
    for i in 0..SUDOKUSIZE {
        for j in 0..SUDOKUSIZE {
            solver_state.arr[i][j] = match game_state.arr[i][j] {
                Some(1) => NumberSet::N1,
                Some(2) => NumberSet::N2,
                Some(3) => NumberSet::N3,
                Some(4) => NumberSet::N4,
                Some(5) => NumberSet::N5,
                Some(6) => NumberSet::N6,
                Some(7) => NumberSet::N7,
                Some(8) => NumberSet::N8,
                Some(9) => NumberSet::N9,
                None => NumberSet::ALL,
                Some(_) => {
                    panic!("Invalid game state while converting game state to solver state.")
                }
            };
        }
    }
    solver_state
}

pub fn solver_to_game_state(solver_state: &Sudoku<NumberSet>) -> Sudoku<GameStateCell> {
    let mut game_state = Sudoku::<GameStateCell> {
        arr: [[None; SUDOKUSIZE]; SUDOKUSIZE],
    };
    for i in 0..SUDOKUSIZE {
        for j in 0..SUDOKUSIZE {
            game_state.arr[i][j] = match solver_state.arr[i][j] {
                NumberSet::N1 => Some(1),
                NumberSet::N2 => Some(2),
                NumberSet::N3 => Some(3),
                NumberSet::N4 => Some(4),
                NumberSet::N5 => Some(5),
                NumberSet::N6 => Some(6),
                NumberSet::N7 => Some(7),
                NumberSet::N8 => Some(8),
                NumberSet::N9 => Some(9),
                _ => None,
            };
        }
    }
    game_state
}

pub fn compute_exclude(solver_state: &mut Sudoku<NumberSet>) {
    for i in 0..SUDOKUSIZE {
        for j in 0..SUDOKUSIZE {
            let cell_num_set = solver_state.arr[i][j];
            if cell_num_set.is_singleton() {
                for k in 0..SUDOKUSIZE {
                    // row
                    if k != j {
                        solver_state.arr[i][k] = solver_state.arr[i][k] - cell_num_set;
                    }
                }
                for k in 0..SUDOKUSIZE {
                    // column
                    if k != i {
                        solver_state.arr[k][j] = solver_state.arr[k][j] - cell_num_set;
                    }
                }
                // TODO: square
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const SUDOKU: Sudoku<u8> = Sudoku {
        arr: [
            [11, 12, 13, 14, 15, 16, 17, 18, 19],
            [21, 22, 23, 24, 25, 26, 27, 28, 29],
            [31, 32, 33, 34, 35, 36, 37, 38, 39],
            [41, 42, 43, 44, 45, 46, 47, 48, 49],
            [51, 52, 53, 54, 55, 56, 57, 58, 59],
            [61, 62, 63, 64, 65, 66, 67, 68, 69],
            [71, 72, 73, 74, 75, 76, 77, 78, 79],
            [81, 82, 83, 84, 85, 86, 87, 88, 89],
            [91, 92, 93, 94, 95, 96, 97, 98, 99],
        ],
    };

    #[test]
    fn test_row_iter() {
        let row2: HashSet<_> = SUDOKU.row(Ix2).collect();
        let expected: HashSet<_> = [21, 22, 23, 24, 25, 26, 27, 28, 29].iter().collect();
        assert_eq!(row2, expected);
    }
    #[test]
    fn test_col_iter() {
        let col2: HashSet<_> = SUDOKU.col(Ix2).collect();
        let expected: HashSet<_> = [12, 22, 32, 42, 52, 62, 72, 82, 92].iter().collect();
        assert_eq!(col2, expected);
    }
    #[test]
    fn test_block_iter() {
        let block2: HashSet<_> = SUDOKU.block(Ix2).collect();
        let expected: HashSet<_> = [14, 15, 16, 24, 25, 26, 34, 35, 36].iter().collect();
        assert_eq!(block2, expected);
    }
}
