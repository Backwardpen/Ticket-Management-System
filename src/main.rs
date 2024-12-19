// Importieren der benötigten Bibliotheken
use warp::Filter;
use warp::reject::Reject;
use mysql;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};

// Struktur des Tickets
// Hier definiere ich die Daten, welche ich empfange von dem Frontend. Ich stelle sicher, dass alle Inputs des Users in der richtigen Form vorliegen (hier alle als String).
#[derive(Debug, Deserialize, Serialize)]
struct Ticket {
    ticket_title: String,
    email: String,
    name: String,
    ticket_description: String,
    raum: String,
}

// Definition einer eigenen Fehlerstruktur als String
#[derive(Debug)]
struct CustomError {
    message: String,
}

// Implementierung der Fehlerstruktur
// Hier stelle ich das Format der Fehlermeldung dar, welche ich an den User zurückgebe, wenn ein Fehler auftritt.
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Fehlermeldung wird in der Konsole ausgegeben
        write!(f, "{}", self.message)
    }
}

// Implementierung der Fehlerstruktur in das Error-Handling, welche ich an späterer Stelle wieder nutze
impl std::error::Error for CustomError {}
impl Reject for CustomError {}

// Eine async Funktion, welche ein Ticket erstellt und in die Datenbank speichert
async fn create_ticket(ticket: Ticket, mysql_pool: &mysql::Pool) -> Result<String, warp::Rejection> {
    println!("Eingehendes Ticket: {:?}", ticket);
    let result = mysql_pool.get_conn().and_then(|mut conn| {
        println!("SQL Query wird ausgeführt mit Ticket-Daten: {:?}", ticket);
        // SQL Query, um ein Ticket in die Datenbank zu speichern
        // Wird genutzt nach Vorgabe von MySQL
        conn.exec_drop(
            "INSERT INTO tickets (title, email, name, description, raum) VALUES (?, ?, ?, ?, ?)",
            (
                &ticket.ticket_title,
                &ticket.email,
                &ticket.name,
                &ticket.ticket_description,
                &ticket.raum,
            ),
        ).map(move |_| Ok::<_, mysql::Error>(conn.last_insert_id()))
    });
    // Kontrolle der Eingabe und Fehlerbehandlung
    match result {
        // Wenn das Ticket erfolgreich erstellt wurde, wird die ID des Tickets zurückgegeben
        Ok(Ok(id)) => {
            let msg = format!("Ticket erfolgreich erstellt mit ID: {:?}", id);
            println!("{}", msg);
            Ok(msg)
        }
        // Wenn das Ticket nicht erstellt werden konnte, wird ein CustomError zurückgegeben
        Ok(Err(err)) => {
            // eprintln gibt die Fehlermeldung in der Konsole aus
            eprintln!("Fehler beim Erstellen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
        // Wenn die Verbindung zur Datenbank nicht hergestellt werden konnte, wird ein CustomError zurückgegeben
        Err(err) => {
            eprintln!("Fehler beim Erstellen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
    }
}


// Diese Funktion wird genutzt, um Tickets anhand der Email-Adresse abzurufen
async fn get_tickets_by_email(email: String, mysql_pool: &mysql::Pool) -> Result<Vec<Ticket>, warp::Rejection> {
        println!("Email die abgerufen werden soll: {}", email);
        let result = mysql_pool.get_conn().and_then(move |mut conn| {
            conn.exec_map(
                "SELECT title, email, name, description, raum FROM tickets WHERE lower(email) = lower(?)",
                (email,),
                |(ticket_title, email, name, ticket_description, raum)| Ticket { 
                    ticket_title,
                    email,
                    name,
                    ticket_description,
                    raum,
                },
            )
        });

        match result {
            Ok(tickets) => Ok(tickets),
            Err(err) => {
                eprintln!("Fehler beim Abrufen der Tickets: {:?}", err);
                Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
            }
        }
}

// Hier wird die Konfiguration der Datenbank definiert
#[derive(Debug)]
struct DBConfig {
    user: String,
    password: String,
    host: String,
    port: u16,
    db_name: String,
}

// Meine async Main-Funktion
#[tokio::main]
async fn main() {
    // Lädt die Umgebungsvariablen aus .env
    dotenv().ok();

    // Hier werden die Umgebungsvariablen für die Datenbankverbindung definiert
    // Wenn dies nicht der Fall ist, gebe ich eine Fehlermeldung aus
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

    // MySQL-Verbindungs-URL generieren
    // MySQL-Verbindungs-URL ist ein String, welcher für die Datenbankverbindung notwendig ist
    let mysql_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.db_name
    );

    // Versuch sich mit der Datenbank zu verbinden
    let opts = mysql::Opts::from_url(&mysql_url).expect("Invalid MySQL URL");
    let mysql_pool = match mysql::Pool::new(opts) {
        // Ok = Erfolgreiche Verbindung und erstellt den Pool (Pool ist eine Sammlung von Verbindungen)
        Ok(pool) => pool,
        // Err = Fehlermeldung, wenn die Verbindung nicht hergestellt werden konnte
        Err(err) => {
            eprintln!("Fehler beim Verbinden mit der Datenbank: {:?}", err);
            return;
        }
    };

    // Überprüfen, ob die Verbindung zur Datenbank funktioniert
    let test_conn = mysql_pool.get_conn();
    match test_conn {
        // Wenn die Verbindung erfolgreich war, wird eine Erfolgsmeldung ausgegeben
        Ok(_) => println!("Verbindung zu einer möglichen Datenbank war erfolgreich!"),
        // Wenn die Verbindung nicht erfolgreich war, wird eine Fehlermeldung ausgegeben
        Err(err) => {
            eprintln!("Fehler beim Verbinden mit der Datenbank: {:?}", err);
            return;
        }
    }

    // Datenbank und Tabelle erstellen via MySQL-Query Script
    // Dies funktinioniert nach einem Standard MySQL-Query Script, welches für alle Datenbanken ähnlich ist, nur auf die spezifische Datenbank angepasst
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
    // Hier überprüfe ich, ob meine Datenbank den benötigten Pool hat und ob die Datenbank und richtigen Tabellen erstellt wurden
    match mysql_pool.get_conn().and_then(|mut conn| conn.query_drop(db_creation_query)) {
        // Bei Erfolg wird eine Erfolgsmeldung ausgegeben
        Ok(_) => println!("Database und Tabelle vorhanden oder neu initialisiert."),
        // Bei einem Fehler wird eine Fehlermeldung ausgegeben
        Err(err) => {
            eprintln!("Fehler beim Initialisieren oder Überprüfen der Datenbank: {:?}", err);
            return;
        }
    }

    // CORS Konfiguration
    let cors = warp::cors()
    .allow_origin("http://127.0.0.1:5501") // erlaube Anfragen von dieser URL
    .allow_methods(vec!["POST", "GET", "OPTIONS"]) // erlaube POST, GET und OPTIONS
    .allow_headers(vec!["Content-Type","*"]) // erlaube alle Header
    .build(); // baue die CORS-Konfiguration

    // Hier wird der Endpunkt für das Erstellen eines Tickets definiert
    let tickets = warp::post() 
    .and(warp::path("tickets")) // Endpunkt /tickets
    .and(warp::body::json()) // JSON-Body wird erwartet
    .and_then({
        let pool = mysql_pool.clone();
        move |ticket: Ticket| {
            let pool = pool.clone();
            async move {
                // Matched die Rückgabe der Funktion create_ticket
                match create_ticket(ticket, &pool).await {
                    Ok(msg) => Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&msg), warp::http::StatusCode::OK)),
                    Err(err) =>  {
                        // sehr spezifischer Fehler, der nur in der Konsole ausgegeben wird
                        // eigentlich nur zum Debuggen wichtig
                        let response = warp::reply::with_status(warp::reply::json(&format!("{:?}", err)), warp::http::StatusCode::INTERNAL_SERVER_ERROR);
                        Ok::<_, warp::Rejection>(response)
                    },
                }   
            }
        }
    }).with(cors.clone()); // Cors nun im Endpoint einfügen

    // Hier wird der Endpunkt für das Abrufen von Tickets anhand der Email-Adresse definiert
    let tickets_by_email = warp::get()
    .and(warp::path("tickets")) // Endpunkt /tickets
    .and(warp::path("by_email")) // Endpunkt /by_email
    .and(warp::path::param::<String>()) // Email-Adresse wird erwartet
    .and_then({ // alles wie oben
        let pool = mysql_pool.clone();
        move |email: String| {
            let pool = pool.clone();
            async move {
                match get_tickets_by_email(email, &pool).await {
                    Ok(tickets) => Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&tickets), warp::http::StatusCode::OK)),
                    Err(err) => Ok::<_, warp::Rejection>(warp::reply::with_status(warp::reply::json(&format!("{:?}", err)), warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
                }
            }
        }
    }).with(cors.clone());

    let routes = tickets.or(tickets_by_email);
    // Server starten und auf Port 5555 laufen lassen
    let log = warp::log("ticketsystem");
    warp::serve(routes.with(log)).run(([127, 0, 0, 1], 5555)).await;
}