use mysql::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read dotenv 
    let current_directory = env::current_dir();
    let dotenv_directory = current_directory
        .expect("no se pudo leer el directorio de trabajo")
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("config.env");
    let _ = dotenvy::from_path(dotenv_directory);
    
    // Connect to MySQL
    let user = env::var("MYSQL_ADMIN_USER").unwrap();
    let password = env::var("MYSQL_ADMIN_PASSWORD").unwrap();
    let host = env::var("MYSQL_UA_HOST").unwrap();
    let database = env::var("MYSQL_PSDA_DATABASE").unwrap();
    let url = format!("mysql://{user}:{password}@{host}:3306/{database}");
    let _pool = Pool::new(Opts::from_url(&url).unwrap());

    // MySQL Connection
    let url = format!("mysql://{user}:{password}@{host}/{database}");
    let pool = Pool::new(Opts::from_url(&url)?)?;
    match pool.get_conn() {
        Ok(_conn) => {
            println!("Conexión exitosa a la base de datos.");
            // Puedes realizar operaciones con `conn` aquí
        },
        Err(e) => eprintln!("Error al conectar a la base de datos: {}", e),
    }

    Ok(())
}
