use cde_rust_sudoku::base::*;
use cde_rust_sudoku::solve::*;
use std::str::FromStr;

const SUDOKU: &'static str = "\
┌─┬─┬─┬─┬─┬─┬─┬─┬─┐
│7│8│2│9│5│3│4│1│6│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│5│1│6│2│8│4│7│9│3│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│9│4│3│1│6│7│2│8│5│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│2│9│4│7│3│6│1│5│8│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│6│3│1│5│2│8│9│7│4│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│8│7│5│4│9│1│3│6│2│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│1│5│7│8│4│2│6│3│9│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│4│6│9│3│7│5│8│2│1│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│3│2│8│6│1│9│5│4│7│
└─┴─┴─┴─┴─┴─┴─┴─┴─┘";

const SUDOKU_2: &'static str = "
 8 9  4
5 6 8 79
943 6  8
 9    158
6  528  4
875    6
 5  4 639
 69 7 8 1
  8  9 4 ";

fn main() {
    let mut sudoku = Sudoku::from_str(SUDOKU).unwrap();
    let sudoku = match brute_force(sudoku) {
        SolverResult::Solved(s) => {
            println!("solved!");
            s
        }
        SolverResult::Contradiction(s) => {
            println!("Contradiction!");
            s
        }
    };
    println!("{}", sudoku);
}
