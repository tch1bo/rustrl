pub mod environment;
pub mod tictactoe;
use crate::environment::Environment;
use crate::environment::State;
use rand::seq::SliceRandom;

fn main() {
    let mut ttt = tictactoe::TicTacToeEnvironment::new();

    while !ttt.get_state().is_terminal() {
        println!("{:#}", ttt.get_state());

        let actions = ttt.get_actions();
        let action = &actions.choose(&mut rand::thread_rng()).expect("asd");
        let reward = ttt.apply_action(action);
        println!("got reward {reward} for {action:?}");
        println!("--------------------------------------------------------");
    }

    println!("\n\nfinal state:\n\n{:#}", ttt.get_state());
}
