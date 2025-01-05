use dotenv::dotenv;
use mysql::prelude::*;
use mysql::{Pool, PooledConn};
use std::env;

// Hier wird eine Struktur definiert, die die Konfiguration für die Datenbank enthält
#[derive(Debug)]
struct DBConfig {
    user: String,
    password: String,
    host: String,
    port: u16,
    db_name: String,
}

pub async fn get_db_pool() -> Pool {
    dotenv().ok();

    // Hier werden die Umgebungsvariablen für die Datenbankverbindung abgerufen
    let db_config = DBConfig {
        user: env::var("DB_USER").expect("DB_USER muss gesetzt sein"),
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD muss gesetzt sein"),
        host: env::var("DB_HOST").expect("DB_HOST muss gesetzt sein"),
        port: env::var("DB_PORT")
        .expect("DB_PORT muss gesetzt sein")
        .parse::<u16>()
        .expect("DB_PORT muss eine Zahl sein"),
        db_name: env::var("DB_NAME").expect("DB_NAME muss gesetzt sein"),
    };

    // Hier wird die MySQL-Verbindungs-URL erstellt
    // Diese verläuft immer nach dem selben Schema: mysql://<user>:<password>@<host>:<port>/<db_name> und muss auch so aufgebaut sein
    let mysql_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.db_name
    );

    let opts = mysql::Opts::from_url(&mysql_url).expect("Invalid MySQL URL");
    let mysql_pool = match mysql::Pool::new(opts) {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Fehler beim Verbinden mit der Datenbank: {:?}", err);
            std::process::exit(1);
        }
    };

    // Hier wird überprüft, ob die Verbindung zur Datenbank erfolgreich war mittels einer Testabfrage
    let test_conn = mysql_pool.get_conn();
    match test_conn {
        Ok(_) => println!("Verbindung zur Datenbank war erfolgreich!"),
        Err(err) => {
            eprintln!("Fehler beim Verbinden mit der Datenbank: {:?}", err);
            std::process::exit(1);
        }
    }

    // Hier wird überprüft, ob die Datenbank und die Tabelle existieren und ggf. neu erstellt
    let db_creation_query = "
        CREATE DATABASE IF NOT EXISTS ticketsystem;
        USE ticketsystem;
        CREATE TABLE IF NOT EXISTS tickets (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            raum VARCHAR(50) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            role VARCHAR(50) NOT NULL DEFAULT 'user',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";

    // Hier wird die Abfrage ausgeführt und das Ergebnis überprüft
    match mysql_pool
    .get_conn()
    .and_then(|mut conn: PooledConn| conn.query_drop(db_creation_query)) {
        Ok(_) => println!("Database und Tabelle vorhanden oder neu initialisiert."),
        Err(err) => {
            eprintln!(
                "Fehler beim Initialisieren oder Überprüfen der Datenbank: {:?}",
                err
            );
            std::process::exit(1);
        }
    }
    mysql_pool // Hier wird der Pool zurückgegeben
}