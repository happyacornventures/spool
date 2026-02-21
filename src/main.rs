fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if --dir flag is present and capture its value
    let dir_arg = args
        .iter()
        .position(|arg| arg == "--dir")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());

    // Check if --network flag is present and capture its value
    let network_arg = args
        .iter()
        .position(|arg| arg == "--network")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());

    let Some(dir) = &dir_arg else {
        println!("Please provide a directory path with the --dir flag.");
        return;
    };

    // check if network exists
    if let Some(network) = &network_arg {
        let network_check_command = format!("docker network ls | grep {}", network);
        let network_check_status = std::process::Command::new("sh")
            .arg("-c")
            .arg(network_check_command)
            .status();

        if !network_check_status.unwrap().success() {
            // println!("Network '{}' does not exist. Please create it before running the program.", network);
            std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("docker network create {}", network))
                .status();
            return;
        }
    }

    // Add project name to docker compose commands if a network is specified
    let project_name_flag = network_arg
        .as_ref()
        .map(|n| format!(" -p {}", n))
        .unwrap_or_default();

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

                        // Pull latest images
                        println!("Pulling latest images for: {}", path.display());
                        let docker_compose_pull_command = format!(
                            "cd {} && docker compose{} pull",
                            path.file_name().unwrap().to_str().unwrap(),
                            project_name_flag
                        );
                        let pull_cmd_status = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&docker_compose_pull_command)
                            .current_dir(dir)
                            .status();

                        if !pull_cmd_status.unwrap().success() {
                            println!("Failed to pull images for: {}", path.display());
                            continue; // Skip to next directory if pull fails
                        }

                        println!("Starting services in: {}", path.display());
                        let docker_compose_up_command = format!(
                            "cd {} && docker compose up -d",
                            path.file_name().unwrap().to_str().unwrap()
                        );
                        let compose_cmd_status = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(docker_compose_up_command)
                            .current_dir(dir)
                            .status();
                        if compose_cmd_status.unwrap().success() {
                            println!("Successfully started services in: {}", path.display());
                        } else {
                            println!("Failed to start services in: {}", path.display());
                        }
                    } else {
                        println!("No docker-compose.yml file found in: {}", path.display());
                    }
                }
            }
        }
    }
}
