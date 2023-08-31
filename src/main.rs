pub mod environment;
pub mod rl;
pub mod tictactoe;

use crate::environment::{Environment, State};
use crate::rl::trainer::{DPTrainer, DPTrainerConfig};
use crate::tictactoe::environment::TicTacToeEnvironment;
use rand::seq::SliceRandom;
use std::env;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("please provide one argument")
    }

    if args[1] == "play" {
        let mut ttt = TicTacToeEnvironment::new();
        while !ttt.state().is_terminal() {
            println!("{:#}", ttt.state());

            let actions = ttt.actions();
            let action = &actions.choose(&mut rand::thread_rng()).expect("asd");
            let reward = ttt.apply_action(action);
            println!("got reward {reward:?} for {action:?}");
            println!("--------------------------------------------------------");
        }

        println!("\n\nfinal state:\n\n{:#}", ttt.state());
    } else if args[1] == "train_dp" {
        let env = TicTacToeEnvironment::new();
        let config = DPTrainerConfig::new(10000);
        let mut trainer = DPTrainer::init_with_uniform_policies(env, config);
        trainer.train();
    } else {
        panic!("unexpected arguments provided: {args:?}")
    }
}
