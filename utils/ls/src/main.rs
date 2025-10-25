pub fn main() {
    match std::env::current_dir() {
        Ok(cur_dir) => {
            let paths = std::fs::read_dir(&cur_dir).unwrap();
            for path in paths {
                println!("{}", path.unwrap().path().display());
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
