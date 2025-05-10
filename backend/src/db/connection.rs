use dotenv::dotenv;
use mysql::prelude::*;
use mysql::{Pool, PooledConn};
use std::env;

// Hier wird die Struktur definiert, die für die Konfiguration für die Datenbank enthält
// user = Benutzername
// password = Passwort
// host = Wird genutzt als Ip Adresse oder Hostname
// port = Port der Datenbank, standardmäßig 3306
// db_name = Name der Datenbank, die genutzt werden soll
// Diese Struktur wird genutzt um die Umgebungsvariablen zu speichern und um die Verbindung zur Datenbank zu erstellen
#[derive(Debug)]
struct DBConfig {
    user: String,
    password: String,
    host: String,
    port: u16,
    db_name: String,
}

// Hier wird die Funktion get_db_pool definiert, die die Verbindung zur Datenbank erstellt
// Diese Funktion gibt ein "Pool" zurück, welches genutzt werden kann um die Verbindung zur Datenbank zu erstellen
pub async fn get_db_pool() -> Pool {
    // Hier wird die .env Datei geladen, die die Umgebungsvariablen enthält
    // Diese Datei wird nicht mit hochgeladen, da sie sensible Daten enthält und alle nötigen 
    dotenv().ok();

    // Hier werden die Umgebungsvariablen für die Datenbankverbindung abgerufen
    // Jede dieser Variablen muss gesetzt sein, da sie für die Verbindung zur Datenbank benötigt werden
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
    // Eine Datenbank für die Tickts und eine für die User
    let db_creation_query = "
        SET SESSION sql_mode=(SELECT REPLACE(@@SESSION.sql_mode, 'STRICT_TRANS_TABLES', '')); -- komische Lösung aus dem Internet, nicht hinterfragen
        CREATE DATABASE IF NOT EXISTS ticketsystem;
        USE ticketsystem;
        CREATE TABLE IF NOT EXISTS tickets (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            raum VARCHAR(50) NOT NULL,
            status ENUM('open', 'in_progress', 'closed') NOT NULL DEFAULT 'open',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            -- role VARCHAR(50) NOT NULL DEFAULT 'user', -- Hier würde ich die Rolle des Users speichern, jedoch habe ich mich entschieden keine Rollen zu haben, da es keinen Nutzen gab bis jetzt.
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";

    // Hier wird die Abfrage ausgeführt und das Ergebnis überprüft
    match mysql_pool
        .get_conn()
        .and_then(|mut conn: PooledConn| conn.query_drop(db_creation_query))
    {
        Ok(_) => println!("Database und Tabelle vorhanden oder neu initialisiert."),
        Err(err) => {
            eprintln!("Fehler beim Initialisieren oder Überprüfen der Datenbank: {:?}",err);
            std::process::exit(1);
        }
    }

    // Überprüfe ob ein User mit ID 1 existiert
    // Wird genutzt, damit ich nicht jedes Mal den Standard User neu erstellen muss
    // Der Standard User hat die ID 1 und ist ein Admin
    // Der Standard User ist in der .env festgelegt, welche aber nicht mit hochgeladen wird aus Sicherheitsgründen
    let check_user_query = "SELECT id FROM users WHERE id = 1";
    let existing_user = mysql_pool
        .get_conn()
        .and_then(move |mut conn| conn.exec_first::<(u32,), _, _>(check_user_query, ()));
    match existing_user {
        Ok(Some(_)) => println!("Standard User (id=1) existiert bereits!"),
        Ok(None) => {
            let default_email = env::var("DEFAULT_USER_EMAIL").expect("DEFAULT_USER_EMAIL muss gesetzt sein in der .env Datei");
            let default_password = env::var("DEFAULT_USER_PASSWORD").expect("DEFAULT_USER_PASSWORD muss gesetzt sein in der .env Datei");
            let default_password_hash = bcrypt::hash(default_password, bcrypt::DEFAULT_COST).unwrap();
            let insert_default_user = "INSERT INTO users (id, email, password_hash) VALUES (1, ?, ?)";

            match mysql_pool.get_conn().and_then(|mut conn: PooledConn| {
                conn.exec_drop(insert_default_user, (&default_email, &default_password_hash,))
            }) 
            {
                Ok(_) => println!("Standard User (id=1) erfolgreich erstellt!"),
                Err(err) => eprintln!("Fehler beim erstellen des Standard Users: {:?}", err),
            };
        }
        Err(err) => eprintln!("Fehler beim Überprüfen des Standard Users: {:?}", err),
    }
    mysql_pool
}