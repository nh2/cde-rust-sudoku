use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
    pub const VALUES: [NumberSet; 9] = [
        NumberSet::N1,
        NumberSet::N2,
        NumberSet::N3,
        NumberSet::N4,
        NumberSet::N5,
        NumberSet::N6,
        NumberSet::N7,
        NumberSet::N8,
        NumberSet::N9,
    ];
    /// Returns true if exactly one flag is set
    pub fn is_singleton(&self) -> bool {
        self.bits().count_ones() == 1
    }
}

impl TryFrom<char> for NumberSet {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            ' ' => NumberSet::all(),
            '1' => NumberSet::N1,
            '2' => NumberSet::N2,
            '3' => NumberSet::N3,
            '4' => NumberSet::N4,
            '5' => NumberSet::N5,
            '6' => NumberSet::N6,
            '7' => NumberSet::N7,
            '8' => NumberSet::N8,
            '9' => NumberSet::N9,
            x => {
                return Err(format!("Not a valid value: {}", x));
            }
        })
    }
}

impl Display for NumberSet {
    fn fmt(&self, mut formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        let n: char = match self {
            &NumberSet::N1 => '1',
            &NumberSet::N2 => '2',
            &NumberSet::N3 => '3',
            &NumberSet::N4 => '4',
            &NumberSet::N5 => '5',
            &NumberSet::N6 => '6',
            &NumberSet::N7 => '7',
            &NumberSet::N8 => '8',
            &NumberSet::N9 => '9',
            _ => ' ',
        };
        write!(&mut formatter, "{}", n)
    }
}

/// Indexing type for rows and columns for compile-time bounds checks
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct Sudoku<T> {
    // row-major
    arr: [[T; 9]; 9],
}

pub fn conv_sudoku_type<T: Copy, U: From<T>>(input_T: Sudoku<T>, output_U: &mut Sudoku<U>) {
    for i in Ix::all_indices() {
        for j in Ix::all_indices() {
            *output_U.get_mut(i, j) = U::from(*input_T.get(i, j))
        }
    }
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
        indices.map(move |(r, c)| (r, c, &self.arr[usize::from(r)][usize::from(c)]))
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
        let r_min = (usize::from(r) / 3) * 3;
        let c_min = (usize::from(c) / 3) * 3;
        self.arr[r_min..r_min + 3]
            .iter()
            .flat_map(move |row| row[c_min..c_min + 3].iter())
    }

    pub fn block_for_cell_mut<'a>(&'a mut self, r: Ix, c: Ix) -> impl Iterator<Item = &'a mut T> {
        let r_min = (usize::from(r) as usize / 3) * 3;
        let c_min = (usize::from(c) as usize / 3) * 3;
        self.arr[r_min..r_min + 3]
            .iter_mut()
            .flat_map(move |row| row[c_min..c_min + 3].iter_mut())
    }

    pub fn regions<'a>(&'a self) -> Regions<'a, T> {
        unimplemented!()
    }
}

#[derive(Copy, Clone)]
enum RegionType {
    Row,
    Col,
    Block,
    Ended,
}

impl RegionType {
    fn succ(self) -> Self {
        match self {
            Self::Row => Self::Col,
            Self::Col => Self::Block,
            Self::Block => Self::Ended,
            Self::Ended => Self::Ended,
        }
    }
}

pub struct Regions<'a, T> {
    sudoku: &'a Sudoku<T>,
    ix: Ix,
    ty: RegionType,
}

impl<'a, T> Regions<'a, T> {
    pub fn new(sudoku: &'a Sudoku<T>) -> Self {
        Self {
            sudoku,
            ix: Ix1,
            ty: RegionType::Row,
        }
    }
}

impl<'a, T> Iterator for Regions<'a, T> {
    type Item = Box<dyn Iterator<Item = &'a T> + 'a>;

    fn next(&'_ mut self) -> Option<Self::Item> {
        let res = match self.ty {
            RegionType::Row => {
                Box::new(self.sudoku.row(self.ix)) as Box<dyn Iterator<Item = &'a T>>
            }
            RegionType::Col => {
                Box::new(self.sudoku.col(self.ix)) as Box<dyn Iterator<Item = &'a T>>
            }
            RegionType::Block => {
                Box::new(self.sudoku.block(self.ix)) as Box<dyn Iterator<Item = &'a T>>
            }
            RegionType::Ended => {
                return None;
            }
        };

        if Ix9 == self.ix {
            self.ty = self.ty.succ();
        };

        self.ix = match self.ix {
            Ix1 => Ix2,
            Ix2 => Ix3,
            Ix3 => Ix4,
            Ix4 => Ix5,
            Ix5 => Ix6,
            Ix6 => Ix7,
            Ix7 => Ix8,
            Ix8 => Ix9,
            Ix9 => Ix1,
        };

        Some(res)
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
            seen = NumberSet::empty();
            for cell in self.col(i) {
                seen = seen | *cell;
            }
            if seen != NumberSet::all() {
                return false;
            }
            seen = NumberSet::empty();
            for cell in self.block(i) {
                seen = seen | *cell;
            }
            if seen != NumberSet::all() {
                return false;
            }
        }
        true
    }

    /// Checks if any number occurs twice in a region or if there are any empty cells.
    pub fn is_invalid(&self) -> bool {
        for i in Ix::all_indices() {
            let mut seen = NumberSet::empty();
            for cell in self.row(i) {
                if cell.is_singleton() {
                    if cell.intersects(seen) {
                        return true;
                    }
                    seen = seen | *cell;
                } else if cell.is_empty() {
                    return true;
                }
            }
            seen = NumberSet::empty();
            for cell in self.col(i) {
                if cell.is_singleton() {
                    if cell.intersects(seen) {
                        return true;
                    }
                    seen = seen | *cell;
                } else if cell.is_empty() {
                    return true;
                }
            }
            seen = NumberSet::empty();
            for cell in self.block(i) {
                if cell.is_singleton() {
                    if cell.intersects(seen) {
                        return true;
                    }
                    seen = seen | *cell;
                } else if cell.is_empty() {
                    return true;
                }
            }
        }
        false
    }

    /* pub fn is_unsolved(&self) -> bool {
        !self.is_solved && !self.contradiction
    } */

    pub fn all_numbers_possible(&self) -> bool {
        //check that any number is possible in all rows, cols and blocks
        for i in Ix::all_indices() {
            let mut seen_row = NumberSet::empty();
            let mut seen_col = NumberSet::empty();
            let mut seen_block = NumberSet::empty();
            for cell in self.row(i) {
                seen_row = seen_row | *cell;
            }
            if seen_row != NumberSet::all() {
                return false;
            }
            for cell in self.col(i) {
                seen_col = seen_col | *cell;
            }
            if seen_col != NumberSet::all() {
                return false;
            }
            for cell in self.block(i) {
                seen_block = seen_block | *cell;
            }
            if seen_block != NumberSet::all() {
                return false;
            }
        }
        true
    }

    pub fn has_no_contradiction(&self) -> bool {
        //check that any number is possible in all rows, cols and blocks
        //check that any set number is only in one cell in every row, col and block
        //check that any cell has min one number
        for i in Ix::all_indices() {
            let mut seen_row = NumberSet::empty();
            let mut seen_col = NumberSet::empty();
            let mut seen_block = NumberSet::empty();
            let mut seen_row_set = NumberSet::empty();
            let mut seen_col_set = NumberSet::empty();
            let mut seen_block_set = NumberSet::empty();
            for cell in self.row(i) {
                if *cell == NumberSet::empty() {
                    return false;
                }
                if cell.is_singleton() {
                    if *cell & seen_row_set != NumberSet::empty() {
                        //number was set before
                        return false;
                    } else {
                        seen_row_set = seen_row_set | *cell;
                    }
                }
                seen_row = seen_row | *cell;
            }
            if seen_row != NumberSet::all() {
                return false;
            }
            for cell in self.col(i) {
                if *cell == NumberSet::empty() {
                    return false;
                }
                if cell.is_singleton() {
                    if *cell & seen_col_set != NumberSet::empty() {
                        //number was set before
                        return false;
                    } else {
                        seen_col_set = seen_col_set | *cell;
                    }
                }
                seen_col = seen_col | *cell;
            }
            if seen_col != NumberSet::all() {
                return false;
            }
            for cell in self.block(i) {
                if *cell == NumberSet::empty() {
                    return false;
                }
                if cell.is_singleton() {
                    if *cell & seen_block_set != NumberSet::empty() {
                        //number was set before
                        return false;
                    } else {
                        seen_block_set = seen_block_set | *cell;
                    }
                }
                seen_block = seen_block | *cell;
            }
            if seen_block != NumberSet::all() {
                return false;
            }
        }
        true
    }
}

impl Display for Sudoku<NumberSet> {
    fn fmt(&self, mut formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        for row in 0..9 {
            for col in 0..9 {
                if col == 3 || col == 6 {
                    write!(&mut formatter, "│")?;
                }
                write!(&mut formatter, "{}", self.arr[row][col])?;
            }
            if row == 2 || row == 5 {
                writeln!(&mut formatter, "\n───┼───┼───")?;
            } else {
                writeln!(&mut formatter, "")?;
            }
        }
        Ok(())
    }
}

impl FromStr for Sudoku<NumberSet> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arr = [[NumberSet::all(); 9]; 9];
        let chars: Vec<_> = s.chars().collect();
        for row in 0..9 {
            for col in 0..9 {
                let value = chars[21 + 40 * row + 2 * col];
                arr[row][col] = NumberSet::try_from(value)?;
            }
        }
        Ok(Sudoku { arr })
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

// conversion of field indices: row/column to outer_square/inner_square
fn ij2sk(i: usize, j: usize) -> (usize, usize) {
    let s = i / 3 * 3 + j / 3;
    let k = i % 3 * 3 + j % 3;
    return (s, k);
}

// conversion of field indices: outer_square/inner_square to row/column
fn sk2ij(s: usize, k: usize) -> (usize, usize) {
    let i = s / 3 * 3 + k / 3;
    let j = s % 3 * 3 + k % 3;
    return (i, j);
}

/// compute field constraints: use known fields to remove options
///
/// Strategy:
/// 1. iterate over all cells to finde known cell
/// 2. for every known cell, remove value from every
///   - row
///   - column
///   - square
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
                let (sq, sqi) = ij2sk(i, j);
                for k in 0..SUDOKUSIZE {
                    // square
                    if k != sqi {
                        let (i2, j2) = sk2ij(sq, k);
                        solver_state.arr[i2][j2] = solver_state.arr[i2][j2] - cell_num_set;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
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

    const N1: NumberSet = NumberSet::N1;
    const N2: NumberSet = NumberSet::N2;
    const N3: NumberSet = NumberSet::N3;
    const N4: NumberSet = NumberSet::N4;
    const N5: NumberSet = NumberSet::N5;
    const N6: NumberSet = NumberSet::N6;
    const N7: NumberSet = NumberSet::N7;
    const N8: NumberSet = NumberSet::N8;
    const N9: NumberSet = NumberSet::N9;
    const NALL: NumberSet = NumberSet::all();
    const VALID_SUDOKU: Sudoku<NumberSet> = Sudoku {
        arr: [
            [N5, N6, N3, N2, N1, N7, N9, N8, N4],
            [N9, N8, N1, N3, N4, N6, N7, N5, N2],
            [N2, N4, N7, N9, N8, N5, N1, N3, N6],
            [N7, N1, N8, N5, N6, N9, N2, N4, N3],
            [N4, N9, N6, N1, N2, N3, N5, N7, N8],
            [N3, N2, N5, N4, N7, N8, N6, N9, N1],
            [N1, N3, N4, N7, N5, N2, N8, N6, N9],
            [N6, N7, N9, N8, N3, N1, N4, N2, N5],
            [N8, N5, N2, N6, N9, N4, N3, N1, N7],
        ],
    };
    const CONTRADICTION_SUDOKU1: Sudoku<NumberSet> = Sudoku {
        //empty cell
        arr: [
            [N5, N6, N3, N2, N1, N7, N9, N8, N4],
            [N9, N8, N1, N3, N4, N6, N7, N5, N2],
            [N2, N4, N7, N9, N8, N5, N1, N3, N6],
            [N7, N1, N8, N5, N6, N9, N2, N4, N3],
            [N4, N9, N6, N1, N2, N3, N5, N7, N8],
            [N3, N2, N5, N4, N7, N8, N6, N9, N1],
            [N1, N3, N4, N7, N5, N2, N8, N6, N9],
            [N6, N7, N9, N8, N3, NumberSet::empty(), N4, N2, N5],
            [N8, N5, N2, N6, N9, N4, N3, N1, N7],
        ],
    };
    lazy_static! {
        static ref CONTRADICTION_SUDOKU2: Sudoku<NumberSet> = Sudoku {
        //number not in row/block
        arr: [
            [N5, N6, N3, N2, N1, N7, N9, N8, N4],
            [N9, N8|N9, N8|N3|N4, N3|N9, N4|N8|N9, N6, N7, N5, N2],
            [N2, N4, N7, N9, N8, N5, N1, N3, N6],
            [N7, N1, N8, N5, N6, N9, N2, N4, N3],
            [N4, N9, N6, N1, N2, N3, N5, N7, N8],
            [N3, N2, N5, N4, N7, N8, N6, N9, N1],
            [N1, N3, N4, N7, N5, N2, N8, N6, N9],
            [N6, N7, N9, N8, N3, N1, N4, N2, N5],
            [N8, N5, N2, N6, N9, N4, N3, N1, N7],
        ],
        };
    }
    lazy_static! {
        static ref CONTRADICTION_SUDOKU3: Sudoku<NumberSet> = Sudoku {
        // Number 6 occurs 2 times in the lower right block
        arr: [
            [N5, NALL, N3, N2, N1, NALL, NALL, N8, N4],
            [N9, N8, N1, NALL, N4, NALL, NALL, N5, N2],
            [N2, N4, N7, NALL, N8, NALL, NALL, N3, N6],
            [N7, N1, N8, NALL, N6, NALL, NALL, N4, N3],
            [N4, N9, N6, NALL, N2, NALL, NALL, N7, N8],
            [N3, N2, N5, NALL, N7, NALL, NALL, N9, N1],
            [N1, N3, N4, NALL, N5, NALL, NALL, N6, N9],
            [N6, N7, N9, NALL, N3, NALL, NALL, N2, N5],
            [NALL, NALL, NALL, NALL, NALL, NALL, N6, N1, N7],
        ],
        };
    }
    #[test]
    fn test_is_solved() {
        assert_eq!(VALID_SUDOKU.is_solved(), true);
        assert_eq!(CONTRADICTION_SUDOKU1.is_solved(), false);
        assert_eq!(CONTRADICTION_SUDOKU2.is_solved(), false);
        assert_eq!(CONTRADICTION_SUDOKU3.is_solved(), false);
    }
    #[test]
    fn test_is_invalid() {
        assert_eq!(VALID_SUDOKU.is_invalid(), false);
        assert_eq!(CONTRADICTION_SUDOKU1.is_invalid(), true);
        assert_eq!(CONTRADICTION_SUDOKU2.is_invalid(), false);
        assert_eq!(CONTRADICTION_SUDOKU3.is_invalid(), true);
    }
    #[test]
    fn test_has_no_contradiction() {
        assert_eq!(VALID_SUDOKU.has_no_contradiction(), true);
        assert_eq!(CONTRADICTION_SUDOKU1.has_no_contradiction(), false);
        assert_eq!(CONTRADICTION_SUDOKU2.has_no_contradiction(), false);
        assert_eq!(CONTRADICTION_SUDOKU3.has_no_contradiction(), false);
    }
    #[test]
    fn test_all_numbers_possible() {
        assert_eq!(VALID_SUDOKU.all_numbers_possible(), true);
        assert_eq!(CONTRADICTION_SUDOKU1.all_numbers_possible(), false);
        assert_eq!(CONTRADICTION_SUDOKU2.all_numbers_possible(), false);
        assert_eq!(CONTRADICTION_SUDOKU3.all_numbers_possible(), true);
    }
}
