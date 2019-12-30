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

    pub fn has_no_contradiciton(&self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use lazy_static::lazy_static;

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

    const N1:NumberSet = NumberSet::N1;
    const N2:NumberSet = NumberSet::N2;
    const N3:NumberSet = NumberSet::N3;
    const N4:NumberSet = NumberSet::N4;
    const N5:NumberSet = NumberSet::N5;
    const N6:NumberSet = NumberSet::N6;
    const N7:NumberSet = NumberSet::N7;
    const N8:NumberSet = NumberSet::N8;
    const N9:NumberSet = NumberSet::N9;
    const NALL:NumberSet = NumberSet::all();
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
        //one number 2 times in one block
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
    fn test_has_no_contradiction() {
        assert_eq!(VALID_SUDOKU.has_no_contradiciton(), true);
        assert_eq!(CONTRADICTION_SUDOKU1.has_no_contradiciton(), false);
        assert_eq!(CONTRADICTION_SUDOKU2.has_no_contradiciton(), false);
        assert_eq!(CONTRADICTION_SUDOKU3.has_no_contradiciton(), false);
    }
    #[test]
    fn test_all_numbers_possible() {
        assert_eq!(VALID_SUDOKU.all_numbers_possible(), true);
        assert_eq!(CONTRADICTION_SUDOKU1.all_numbers_possible(), false);
        assert_eq!(CONTRADICTION_SUDOKU2.all_numbers_possible(), false);
        assert_eq!(CONTRADICTION_SUDOKU3.all_numbers_possible(), true);
    }
}
