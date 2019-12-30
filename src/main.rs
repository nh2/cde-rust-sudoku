use cde_rust_sudoku::base::*;

fn main() {
    let input = "
 8 9  4
5 6 8 79
943 6  8
 9    158
6  528  4
875    6
 5  4 639
 69 7 8 1
  8  9 4 ";

    let game_state = parse_game_state(&input).unwrap();
    println!("{}", format_game_state(&game_state));
    let mut solver_state = game_to_solver_state(&game_state);
    compute_exclude(&mut solver_state);
    let game_state = solver_to_game_state(&solver_state);
    println!("{}", format_game_state(&game_state));
}
