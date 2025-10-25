pub fn main() {
    match std::env::current_dir() {
        Ok(cur_dir) => {
            let paths = std::fs::read_dir(&cur_dir).unwrap();
            for path in paths {
                println!("{}", path.unwrap().file_name().display());
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-1)
        }
    }
}