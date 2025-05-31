use std::io::{self};
// use std::process::Command;


struct Folder {
    name: String,
    path: String,
    private: bool
}

fn main() {
    let mut folders: Vec<Folder> = get_initial_folder();
    let mut is_folders_checked: bool = false;

    loop {
        println!("\nCommand list:");
        println!("1. List folders");
        println!("2. Create folder");
        println!("3. Delete folder");
        println!("4. Check folders");
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
            2 => { create_folder(&mut folders); is_folders_checked = false },
            3 => exclude_folder(&mut folders),
            4 => { 
                if is_folders_checked == true {
                    println!("Folders already checked");
                    continue;
                }

                check_folders(&folders); 
                is_folders_checked = true;
            },
            _ => println!("Invalid command"),
        }
    }
}

fn create_folder(folders: &mut Vec<Folder>) {
    let mut folder_name = String::new();
    let mut folder_path = String::new();

    println!("Enter folder name: ");
    io::stdin()
        .read_line(&mut folder_name)
        .expect("Failed to read input");
    println!("Enter folder path: ");
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read input");

    folder_name = folder_name.trim().to_string();

    let new_folder = Folder {
        name: folder_name.trim().to_string(),
        path: folder_path.trim().to_string(),
        private: false
    };

    for folder in folders.iter_mut() {
         if folder.name == folder_name {
            println!("Folder already exists");
            return;
        }
    }

    folders.push(new_folder);

    println!("Folder {} created successfully!", folder_name);
}

fn exclude_folder(folders: &mut Vec<Folder>) {
    if folders.is_empty() {
        println!("No folders to remove.");
        return;
    }

    let original_len = folders.len();
    let mut folder_name = String::new();

    println!("Enter the name of the folder you want to remove: ");
    io::stdin()
        .read_line(&mut folder_name)
        .expect("Failed to read input");

    folders.retain(|folder| folder.name != folder_name.trim().to_string());

    if folders.len() < original_len {
        println!("Folder {} successfully deleted!", folder_name);
    } else {
        println!("Folder {} not found.", folder_name);
    }
}

fn listing(folders: &Vec<Folder>) {
    for folder in folders {
        println!("Folder: {} | Path: {} | isPrivate: {}", folder.name, folder.path, folder.private);
    }
}

fn check_folders(folders: &Vec<Folder>) {

    for folder in folders.iter() {
        println!("Checking folder: {}", folder.name);
    }
    println!("####### All folders checked #######");
}

fn get_initial_folder() -> Vec<Folder> {
        let initial_folder = Folder {
        name: String::from("Downloads"),
        path: String::from("batman/home"),
        private: false
    };

    let second_folder = Folder {
        name: String::from("Documents"),
        path: String::from("batman/home"),
        private: false
    };

    return vec![
        initial_folder.into(),
        second_folder.into()
    ];
}

// fn list_dir() {
//     let output = Command::new("ls")
//         .output()
//         .expect("Erro ao executar comando");

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     println!("Pastas:\n{}", stdout);
// }
