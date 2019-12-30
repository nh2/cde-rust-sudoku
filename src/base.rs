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

//TODO: write from trait with the new Iterator
//
//impl From<Sudoku<T>> for Sudoku<U> {
//    fn from (item: Sudoku<T> -> Sudoku<U> {





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
