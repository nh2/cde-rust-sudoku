use cde_rust_sudoku::base::*;
use cde_rust_sudoku::solve::*;
use cde_rust_sudoku::examples::*;
use std::str::FromStr;

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
