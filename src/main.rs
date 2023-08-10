pub mod environment;
pub mod tictactoe;

use crate::environment::{DPEnvironment, Environment, State, StateId};
use crate::tictactoe::environment::TicTacToeEnvironment;
use rand::seq::SliceRandom;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("please provide one argument")
    }

    let mut ttt = TicTacToeEnvironment::new();
    if args[1] == "play" {
        while !ttt.state().is_terminal() {
            println!("{:#}", ttt.state());

            let actions = ttt.actions();
            let action = &actions.choose(&mut rand::thread_rng()).expect("asd");
            let reward = ttt.apply_action(action);
            println!("got reward {reward:?} for {action:?}");
            println!("--------------------------------------------------------");
        }

        println!("\n\nfinal state:\n\n{:#}", ttt.state());
    } else if args[1] == "dp" {
        let transition_table = ttt.state_transitions();
        println!("{:#?}", transition_table[&StateId(0)]);
    } else {
        panic!("unexpected arguments provided: {args:?}")
    }
}
