use std::fs;
use std::env;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // Получаем путь к текущей директории
    let current_dir = env::current_dir()?;
    
    println!("Содержимое директории {:?}:", current_dir);

    // Читаем содержимое директории
    let entries = fs::read_dir(current_dir)?;

    // Итерируемся по всем элементам в директории
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Выводим имя файла или папки
        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                println!("{}", name_str);

                // Если файл называется Cargo.toml, читаем его содержимое
                if name_str == "Cargo.toml" {
                    print_cargo_toml_contents(&path)?;
                }
            }
        }
    }

    Ok(())
}

// Функция для чтения и вывода содержимого файла Cargo.toml
fn print_cargo_toml_contents(path: &Path) -> io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Содержимое файла Cargo.toml:");
    println!("{}", contents);
    Ok(())
}
