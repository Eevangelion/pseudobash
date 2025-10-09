use pseudobash::cli::{CLI, DefaultCLI};

fn main() {
    println!("Welcome to Pseudobash v{}!\n", env!("CARGO_PKG_VERSION"));
    let mut cli: DefaultCLI = CLI::default();
    cli.start();
}
