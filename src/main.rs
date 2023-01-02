use std::env;
use fork::{daemon, Fork};

fn main() {

    let args: Vec<String> = env::args().collect();
    if let Ok(Fork::Child) = daemon(false, false) {
        let _ = exec::execvp(&args[1], &args[2..]);
    }

}
