/* Bibliotheken, die ich benutzen würde:
 * - tokio (für Asynchrone Prozesse)
 * - mysql (SQL interface)
 * - warp? oder rocket als alternative(rest API)
 * - serde (umformung der Daten für den SQL Server und für den input, könnte vermutlich umgangen werden)
 */

use warp::Filter;
use warp::reject::Reject;
use mysql;
use dotenv::dotenv;
use mysql::prelude::*;
use std::env;
use serde::Deserialize;

// Struktur des Tickets
#[derive(Debug, Deserialize)]
struct Ticket {
    ticket_title: String,
    email: String,
    name: String,
    ticket_description: String,
    raum: String,
}

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for CustomError {}

impl Reject for CustomError {}

async fn create_ticket(ticket: Ticket, mysql_pool: &mysql::Pool) -> Result<&'static str, warp::Rejection> {
    println!("Eingehendes Ticket: {:?}", ticket); // Log incoming data
    let result = mysql_pool.get_conn().and_then(|mut conn| {
        println!("SQL Query wird ausgeführt mit Ticket-Daten: {:?}", ticket);
        conn.exec_drop(
            "INSERT INTO tickets (title, email, name, description, raum) VALUES (?, ?, ?, ?, ?)",
            (
                &ticket.ticket_title,
                &ticket.email,
                &ticket.name,
                &ticket.ticket_description,
                &ticket.raum,
            ),
        )
    });

    match result {
        Ok(_) => {
            println!("Ticket erstellt: {:?}", ticket);
            Ok("Ticket erfolgreich erstellt")
        }
        Err(err) => {
            eprintln!("Fehler beim Erstellen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
    }
}

#[derive(Debug)]
struct DBConfig {
    user: String,
    password: String,
    host: String,
    port: String,
    db_name: String,
}

#[tokio::main]
async fn main() {
    // Lädt die Umgebungsvariablen aus .env
    dotenv().ok();

    // Konfiguration der Datenbank aus Umgebungsvariablen laden
    let db_config = DBConfig {
        user: env::var("DB_USER").expect("DB_USER muss gesetzt sein"),
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD muss gesetzt sein"),
        host: env::var("DB_HOST").expect("DB_HOST muss gesetzt sein"),
        port: env::var("DB_PORT").expect("DB_PORT muss gesetzt sein"),
        db_name: env::var("DB_NAME").expect("DB_NAME muss gesetzt sein"),
    };

    // MySQL-Verbindungs-URL generieren
    let mysql_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.db_name
    );

    // Verbindung zur Datenbank herstellen
    let opts = mysql::Opts::from_url(&mysql_url).expect("Invalid MySQL URL");
    let mysql_pool = match mysql::Pool::new(opts) {
        Ok(pool) => pool, // Verbindung erfolgreich
        Err(err) => {
            eprintln!("Fehler beim Verbinden mit der Datenbank: {:?}", err);
            return; // Beendet die Anwendung bei Fehlern
        }
    };

    // Überprüfen, ob die Verbindung zur Datenbank funktioniert
    let test_conn = mysql_pool.get_conn();
    match test_conn {
        Ok(_) => println!("Datenbankverbindung erfolgreich!"),
        Err(err) => {
            eprintln!("Fehler beim Verbinden mit der Datenbank: {:?}", err);
            return; // Beendet die Anwendung bei Verbindungsfehler
        }
    }

    // Datenbank und Tabelle erstellen via MySQL-Query Script
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
    ";
    match mysql_pool.get_conn().and_then(|mut conn| conn.query_drop(db_creation_query)) {
        Ok(_) => println!("Database und Tabelle vorhanden oder neu initialisiert."),
        Err(err) => {
            eprintln!("Fehler beim Initialisieren oder Überprüfen der Datenbank: {:?}", err);
            return; // Beendet die Anwendung bei Fehlern
        }
    }

    // Warp-Routen erstellen
    let tickets = warp::post()
        .and(warp::path("tickets"))
        .and(warp::body::json()) // JSON-Daten erwarten
        .and_then(move |ticket: Ticket| {
            let pool = mysql_pool.clone(); // Pool klonen, um Threadsicherheit zu gewährleisten
            async move {
                match create_ticket(ticket, &pool).await {
                    Ok(msg) => Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&msg), warp::http::StatusCode::OK)), // Erfolgreich
                    Err(_) => Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&"Fehler beim Erstellen des Tickets"), warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
                }
            }
        });

    // Server starten
    let log = warp::log("ticketsystem");
    warp::serve(tickets.with(log)).run(([127, 0, 0, 1], 5555)).await;
}