use crate::base::{NumberSet, Sudoku, compute_exclude};

pub enum SolverResult {
    Solved(Sudoku<NumberSet>),
    Contradiction(Sudoku<NumberSet>),
}


pub fn brute_force(mut sudoku: Sudoku<NumberSet>) -> SolverResult {
    if sudoku.is_solved() {
        return SolverResult::Solved(sudoku);
    }
    if sudoku.is_invalid() {
        return SolverResult::Contradiction(sudoku);
    }
    let first_empty_cell = sudoku.iter_with_index().find(|elem| !elem.2.is_singleton());
    let (r, c, cell) = match first_empty_cell {
        None => {
            return SolverResult::Contradiction(sudoku);
        }
        Some(x) => x,
    };
    let cell = *cell;
    for val in NumberSet::VALUES.iter() {
        if cell.intersects(*val) {
            *sudoku.get_mut(r, c) = *val;
            sudoku = match brute_force(sudoku) {
                SolverResult::Solved(s) => {
                    return SolverResult::Solved(s);
                }
                SolverResult::Contradiction(s) => s,
            };
            *sudoku.get_mut(r, c) = cell;
        }
    }
    // If all of the previous attempts returned Contradiction, that's what we return too
    SolverResult::Contradiction(sudoku)
}

pub fn brute_force_with_exclude(mut sudoku: Sudoku<NumberSet>) -> SolverResult {
    compute_exclude(&mut sudoku);
    if sudoku.is_solved() {
        return SolverResult::Solved(sudoku);
    }
    if sudoku.is_invalid() {
        return SolverResult::Contradiction(sudoku);
    }
    let first_empty_cell = sudoku.iter_with_index().find(|elem| !elem.2.is_singleton());
    let (r, c, cell) = match first_empty_cell {
        None => {
            return SolverResult::Contradiction(sudoku);
        }
        Some(x) => x,
    };
    let cell = *cell;
    for val in NumberSet::VALUES.iter() {
        if cell.intersects(*val) {
            let mut sudoku_down = sudoku.clone();
            *sudoku_down.get_mut(r, c) = *val;
            match brute_force_with_exclude(sudoku_down) {
                SolverResult::Solved(s) => {
                    return SolverResult::Solved(s);
                }
                SolverResult::Contradiction(s) => (),
            };
        }
    }
    // If all of the previous attempts returned Contradiction, that's what we return too
    SolverResult::Contradiction(sudoku)
}

