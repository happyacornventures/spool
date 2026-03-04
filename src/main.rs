fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if --dir flag is present and capture its value
    let dir_arg = args
        .iter()
        .position(|arg| arg == "--dir")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());

    // Check if --project flag is present and capture its value
    let project_arg = args
        .iter()
        .position(|arg| arg == "--project")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());

    let Some(dir) = &dir_arg else {
        println!("Please provide a directory path with the --dir flag.");
        return;
    };

    // check if project exists
    // if let Some(project) = &project_arg {
    //     let project_check_command = format!("docker ps -a --filter 'name={}'", project);
    //     let project_check_status = std::process::Command::new("sh")
    //         .arg("-c")
    //         .arg(project_check_command)
    //         .status();

    //     if !project_check_status.unwrap().success() {
    //         // println!("Project '{}' does not exist. Please create it before running the program.", project);
    //         std::process::Command::new("sh")
    //             .arg("-c")
    //             .arg(format!("docker network create {}", network))
    //             .status();
    //         return;
    //     }
    // }

    // Add project name to docker compose commands if a project is specified
    let project_name_flag = project_arg
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
                            "cd {} && docker compose{} up -d",
                            path.file_name().unwrap().to_str().unwrap(),
                            project_name_flag
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
