/* Bibliotheken, die ich benutzen würde:
 * - tokio (für Asynchrone Prozesse)
 * - mysql (SQL interface)
 * - warp? oder rocket als alternative(rest API)
 * - serde (umformung der Daten für den SQL Server und für den input, könnte vermutlich umgangen werden)
 */

use std::collections::HashMap;
use warp::Filter;
use mysql;
use dotenv::dotenv;
use std::env;

/*
struct Ticket {
    ticketTitle: String,
    email: String,
    name: String,
    ticketDescription: String,
    raum: String,
}
*/

async fn das_teil_was_aufgerufen_wird_wenn_ein_ticket_erstellt_wird(_ticket: HashMap<String, String>, _mysql_pool: &mysql::Pool) -> &str {
    // Hier wird das Ticket in die Datenbank geschrieben
    // Muss überprüfen, dass alle Informationen tatsächlich richtig eingegeben wurden
    // Inputs sanieren aka MySQL Injections blocken 
    // Übergabe der Daten an den MySQL-Server (asynchron) <-- zusammen packen mit der Sanierung der Inputs
    // Rückgabe des Anfragestatus
    println!("Ticket definitiv erstellt");
    return "Ticket definitiv erstellt";
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
// Erklärung was Async heißt
async fn main() {
    // Lädt die .env Datei, die die Zugänge für die Datenbank enthält
    dotenv().ok();

    // Standarddefinition der Zugänge (durch .env Auslesen oder Argumente ersetzten)
    let db_config = DBConfig {
        user: env::var("DB_USER").expect("DB_USER muss gesetzt sein"),
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD muss gesetzt sein"),
        host: env::var("DB_HOST").expect("DB_HOST muss gesetzt sein"),
        port: env::var("DB_PORT").expect("DB_PORT muss gesetzt sein"),
        db_name: env::var("DB_NAME").expect("DB_NAME muss gesetzt sein"),
    };
    println!("Database Config: {:?}", db_config);

    // Die URL für den SQL Server mit hilfe der Konfiguration generieren  
    let mysql_url = format!("mysql://{}:{}@{}:{}/{}", db_config.user, db_config.password, db_config.host, db_config.port, db_config.db_name);
    // Lädt die Datenbank als "Pool" (Arc (Atomic Reference Counter)), der defür sorgt, dass auch wenn mehrmals auf einmal zugegriffen wird, nichts kaputt geht
    let mysql_pool = mysql::Pool::new(mysql_url).unwrap();

    // Erstellt eine Konfiguration für warp, um Tickets entgegenzunehmen
    let tickets = warp::post()
        .and(warp::path("tickets")) // Setzt den Pfad, auf dem der Server zuhört
        .and(warp::body::form()) // Erwartet HTML Form Daten (gleiches format wie PHP oder rest parameter)
        .and_then(move |ticket: HashMap<String, String>| { // Was bei der Anfrage ausgeführt werden soll
            let pool = mysql_pool.clone(); // "Klont" den pool (erstellt einen Pointer mit dem darauf zugegriffen werden kann der automatisch rechtzeitig gelöscht wird)
            async move {
                // Antwortet auf den Benutzer
                let response = das_teil_was_aufgerufen_wird_wenn_ein_ticket_erstellt_wird(ticket, &pool).await;
                Ok::<_, warp::Rejection>(warp::reply::json(&response))
            }
        });

    warp::serve(tickets).run(([127, 0, 0, 1], 5555)).await;
}