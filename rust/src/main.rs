//TODO Adhere to writing clean, idiomatic Rust code.
fn main() {
    //TODO If ping() is down, log an error and don't call echo().
    ping();
    //TODO Handle and log errors from echo() inside main() and continue.
    echo("Hello, world!");
    //TODO If there are jobs, print them, else print nothing.
    job_count();
    //TODO Lastly, move these functions outside of main.rs so we can reuse and test them.
}

//TODO println! or eprintln! should happen in main()
fn ping() {
    let is_ok = random_bool();
    if is_ok {
        println!("UP");
    } else {
        let err_msg = "DOWN";
        eprintln!("ERROR: {}", err_msg);
    }
}

//TODO println! or eprintln! should happen in main()
fn echo(m: &str) {
    let is_ok = random_bool();
    if is_ok {
        println!("{}", m);
    } else {
        //TODO This kills program execution, which we don't want.
        panic!("ERROR: We encountered error code {}", rand::random::<u8>());
    }
}

//TODO println! or eprintln! should happen in main()
fn job_count() {
    let jobs_exist = random_bool();
    if jobs_exist {
        println!("{}", rand::random::<u8>());
    } else {
        println!("None");
    }
}

/// Just a dummy function to simulate unexpected behavior.
/// `cargo run -- --skip` to always run happy path.
fn random_bool() -> bool {
    if std::env::args().any(|x| x == *"--skip") {
        return true;
    }
    rand::random::<bool>()
}
