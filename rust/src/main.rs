fn main() {
    //TODO If ping() is down, don't call echo().
    ping();
    //TODO Handle and log errors from echo() inside main() and continue.
    echo("Hello, world!");
    //TODO If there are jobs, print them, else print nothing.
    job_count();
    //TODO Lastly, move these functions outside of main.rs so we can reuse and test them.
}

//TODO println! or eprintln! should happen in main()
fn ping() {
    let is_ok = is_successful();
    if is_ok {
        println!("UP");
    } else {
        let err_msg = "DOWN";
        eprintln!("ERROR: {}", err_msg);
    }
}

//TODO println! or eprintln! should happen in main()
fn echo(m: &str) {
    let is_ok = is_successful();
    if is_ok {
        println!("{}", m);
    } else {
        //TODO This kills program execution, which we don't want.
        panic!("ERROR: We encountered error code {}", rand::random::<u8>());
    }
}

//TODO println! or eprintln! should happen in main()
fn job_count() {
    let exists = is_successful();
    if exists {
        println!("{}", rand::random::<u8>());
    } else {
        println!("none");
    }
}

/// Just a dummy function to simulate unexpected behavior.
/// `cargo run -- --skip` to always run happy path.
fn is_successful() -> bool {
    if std::env::args().any(|x| x == *"--skip") {
        return true;
    }
    rand::random::<bool>()
}
