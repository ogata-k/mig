extern crate clap;

use mig::app::{action_controller, mig_app};

fn main() {
    let matches_rs = mig_app().get_matches_safe();
    match matches_rs {
        Err(e) => {
            print!("{}", e);
        }
        Ok(matches) => {
            action_controller(matches);
        }
    }
}

