use cde_rust_sudoku::base::*;
use cde_rust_sudoku::solve::*;
use std::str::FromStr;

const SOLVED_SUDOKU: &'static str = "\
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

const SUDOKU1: &'static str = "\
┌─┬─┬─┬─┬─┬─┬─┬─┬─┐
│ │8│ │9│ │ │4│ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│5│ │6│ │8│ │7│9│ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│9│4│3│ │6│ │ │8│ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │9│ │ │ │ │1│5│8│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│6│ │ │5│2│8│ │ │4│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│8│7│5│ │ │ │ │6│ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │5│ │ │4│ │6│3│9│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │6│9│ │7│ │8│ │1│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │8│ │ │9│ │4│ │
└─┴─┴─┴─┴─┴─┴─┴─┴─┘";

const SUDOKU2: &'static str = "\
┌─┬─┬─┬─┬─┬─┬─┬─┬─┐
│4│ │ │ │ │ │8│ │5│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │3│ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │7│ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │2│ │ │ │ │ │6│ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │8│ │4│ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │1│ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │6│ │3│ │7│ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│5│ │ │2│ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│1│ │4│ │ │ │ │ │ │
└─┴─┴─┴─┴─┴─┴─┴─┴─┘";

const SUDOKU3: &'static str = "\
┌─┬─┬─┬─┬─┬─┬─┬─┬─┐
│8│ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │3│6│ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │7│ │ │9│ │2│ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │5│ │ │ │7│ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │4│5│7│ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │1│ │ │ │3│ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │1│ │ │ │ │6│8│
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │8│5│ │ │ │1│ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │9│ │ │ │ │4│ │ │
└─┴─┴─┴─┴─┴─┴─┴─┴─┘";


const EMPTY_SUDOKU: &'static str = "\
┌─┬─┬─┬─┬─┬─┬─┬─┬─┐
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
├─┼─┼─┼─┼─┼─┼─┼─┼─┤
│ │ │ │ │ │ │ │ │ │
└─┴─┴─┴─┴─┴─┴─┴─┴─┘";

use std::io::Read;

fn python_solve() {
    //parse_test1();
    //solve_test1();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let game_state = parse_game_state(&input).unwrap();
    println!("{}", format_game_state(&game_state));
    let mut solver_state = game_to_solver_state(&game_state);
    //let (_won, _lost) = compute_solve1(&mut solver_state, true);
    //let (_won, _lost) = compute_solve_tree(&mut solver_state, true); // verbose=true
    let (_won, _lost) = compute_solve_tree(&mut solver_state, false); // verbose=false
}

fn main() {
    let sudoku_input = Sudoku::from_str(SUDOKU2).unwrap();


    let timer = std::time::Instant::now();
    let mut sudoku = sudoku_input.clone();
    let sudoku = match brute_force_with_exclude(sudoku) {
        SolverResult::Solved(s) => {
            println!("Solved!");
            s
        }
        SolverResult::Contradiction(s) => {
            println!("Contradiction!");
            s
        }
    };
    println!("{}", sudoku);
    println!("Computed in {} seconds", timer.elapsed().as_secs());


    let timer = std::time::Instant::now();
    let mut sudoku = sudoku_input.clone();
    let sudoku = match brute_force(sudoku) {
        SolverResult::Solved(s) => {
            println!("Solved!");
            s
        }
        SolverResult::Contradiction(s) => {
            println!("Contradiction!");
            s
        }
    };
    println!("{}", sudoku);
    println!("Computed in {} seconds", timer.elapsed().as_secs());
}
