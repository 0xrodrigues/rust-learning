use std::io::{self};

fn main() {
    let mut folders: Vec<String> = vec![
        "Downloads".into(),
        "Documents".into()
    ];

    loop {
        println!("\nLista de comandos:");
        println!("1. Listar pastas");
        println!("2. Criar pasta");
        println!("3. Excluir pasta");
        println!("0. Sair");

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Erro ao ler input");

        let command: i32 = match input_line.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Comando inválido");
                continue;
            }
        };

        match command {
            0 => break,
            1 => listing(&folders),
            2 => create_folder(&mut folders),
            3 => exclude_folder(&mut folders),
            _ => println!("Comando inválido"),
        }
    }
}

fn create_folder(folders: &mut Vec<String>) {
    let mut folder_name = String::new();

    println!("Digite o nome da pasta: ");
    io::stdin()
        .read_line(&mut folder_name)
        .expect("Falha ao ler entrada");

    folder_name = folder_name.trim().to_string();

    folders.push(folder_name.clone());

    println!("Pasta {} criada com sucesso!",folder_name );
}

fn exclude_folder(folders: &mut Vec<String>) {
    if folders.is_empty() {
        println!("Nenhuma pasta para remover.");
        return;
    }

    let mut folder_name = String::new();

    println!("Digite o nome da pasta que deseja remover: ");
    io::stdin()
        .read_line(&mut folder_name)
        .expect("Falha ao ler entrada");

    folder_name = folder_name.trim().to_string();

    let original_len = folders.len();
    folders.retain(|folder| folder != &folder_name);

    if folders.len() < original_len {
        println!("Pasta {} excluída com sucesso!", folder_name);
    } else {
        println!("Pasta {} não encontrada.", folder_name);
    }
}

fn listing(folders: &Vec<String>) {
    for folder in folders.iter() {
        println!("Pasta: {}", folder);
    }
}
