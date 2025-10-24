fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if --dir flag is present and capture its value
    let dir_arg = args.iter()
      .position(|arg| arg == "--dir")
      .and_then(|i| args.get(i + 1))
      .map(|s| s.to_string());

    let Some(dir) = &dir_arg else {
        println!("Please provide a directory path with the --dir flag.");
        return;
    };

    println!("Hello, world!");
}
