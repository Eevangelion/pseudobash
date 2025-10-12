fn main() {
    let read_dir = std::fs::read_dir(
        std::env::current_dir()
            .map(|path| {
                let mut args = std::env::args();
                match args.len() {
                    1 => path,
                    2 => {
                        args.next();
                        path.join(args.next().unwrap())
                            .canonicalize()
                            .unwrap_or_else(|e| {
                                eprintln!("{e}");
                                std::process::exit(-1);
                            })
                    }
                    _ => {
                        eprintln!(
                            "Unexpected number of arguments in: '{}'",
                            args.map(|mut arg| {
                                arg.push(' ');
                                arg
                            })
                            .collect::<String>()
                        );
                        std::process::exit(-1);
                    }
                }
            })
            .unwrap_or_else(|e| {
                eprintln!("{e}");
                std::process::exit(-1);
            }),
    )
    .unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(-1);
    });

    let mut entries: Vec<String> = read_dir
        .filter_map(|val| {
            val.map_err(|e| eprintln!("{e}"))
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .ok()
        })
        .collect();
    entries.sort_by_key(|val| val.to_lowercase());

    for entry in entries {
        if entry.chars().next().is_some_and(|chr| chr == '.') {
            continue;
        }
        println!("{entry}")
    }
}
