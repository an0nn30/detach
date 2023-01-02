use std::{env, process::Command};
use fork::{daemon, Fork};

fn main() {

    let args: Vec<String> = env::args().collect();
    // dbg!("Running: {}", &args[1]);
    // dbg!("With args: {:?}", &args[2..]);

    if let Ok(Fork::Child) = daemon(false, false) {
        let _ = Command::new(&args[1])
        .args(&args[2..])
        .output()
        .expect("Error running command");
    }

}
