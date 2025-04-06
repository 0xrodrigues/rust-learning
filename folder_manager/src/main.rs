use std::io::{self};

fn main() {
    let mut folders: Vec<String> = vec![
        "Downloads".into(),
        "Documents".into()
    ];

    loop {
        println!("\nCommand list:");
        println!("1. List folders");
        println!("2. Create folder");
        println!("3. Delete folder");
        println!("0. Exit");

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Failed to read input");

        let command: i32 = match input_line.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid command");
                continue;
            }
        };

        match command {
            0 => break,
            1 => listing(&folders),
            2 => create_folder(&mut folders),
            3 => exclude_folder(&mut folders),
            _ => println!("Invalid command"),
        }
    }
}

fn create_folder(folders: &mut Vec<String>) {
    let mut folder_name = String::new();

    println!("Enter folder name: ");
    io::stdin()
        .read_line(&mut folder_name)
        .expect("Failed to read input");

    folder_name = folder_name.trim().to_string();

    if folders.contains(&folder_name) {
        println!("Folder already exists");
        return;
    }

    folders.push(folder_name.clone());

    println!("Folder {} created successfully!", folder_name);
}

fn exclude_folder(folders: &mut Vec<String>) {
    if folders.is_empty() {
        println!("No folders to remove.");
        return;
    }

    let mut folder_name = String::new();

    println!("Enter the name of the folder you want to remove: ");
    io::stdin()
        .read_line(&mut folder_name)
        .expect("Failed to read input");

    folder_name = folder_name.trim().to_string();

    let original_len = folders.len();
    folders.retain(|folder| folder != &folder_name);

    if folders.len() < original_len {
        println!("Folder {} successfully deleted!", folder_name);
    } else {
        println!("Folder {} not found.", folder_name);
    }
}

fn listing(folders: &Vec<String>) {
    for folder in folders.iter() {
        println!("Folder: {}", folder);
    }
}
