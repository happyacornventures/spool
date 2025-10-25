fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if --dir flag is present and capture its value
    let dir_arg = args
        .iter()
        .position(|arg| arg == "--dir")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());

    let Some(dir) = &dir_arg else {
        println!("Please provide a directory path with the --dir flag.");
        return;
    };

    for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let path = entry.path();
                    println!("Found subdirectory: {}", path.display());
                    let docker_compose_file = path.join("docker-compose.yml");
                    if docker_compose_file.exists() {
                        println!(
                            "Docker-compose file found in: {}",
                            docker_compose_file.display()
                        );
                    } else {
                        println!("No docker-compose.yml file found in: {}", path.display());
                    }
                }
            }
        }
    }

    println!("Hello, world!");
}
