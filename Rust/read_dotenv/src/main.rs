use dotenvy;
use std::env;
fn main() {
    let current_directory = env::current_dir();
    let dotenv_directory = current_directory
        .expect("no se pudo leer el directorio de trabajo")
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("config.env");
    print!("{:?}", dotenv_directory);
    let _ = dotenvy::from_path(dotenv_directory);
    let test = env::var("TEST_DOTENV");
    println!("{:?}", test);
}
