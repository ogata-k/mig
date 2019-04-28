use mig::app::{action_controller, get_matches_safe, mig_app};

fn main() {
    let matches_rs = get_matches_safe(mig_app());
    match matches_rs {
        Err(e) => {
            print!("{}", e);
        }
        Ok(matches) => {
            match action_controller(matches) {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("\nError:\n{}", e),
            };
        }
    }
}
