mod db;
mod error;
mod handlers;
mod models;

use crate::models::auth::Auth;
use db::connection::get_db_pool;
use handlers::auth_handler::{login_handler, register_handler};
use handlers::ticket_handler::{
    create_ticket_handler, get_all_tickets_handler, get_ticket_by_id_handler,
    get_tickets_by_email_handler,
};
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Datenbankpool erstellen (wird für alle Handler wiederverwendet)
    let mysql_pool = get_db_pool().await;
    // Datenbankverbindungspool klonen, damit er in den Handler verwendet werden kann
    let pool_clone = mysql_pool.clone();

    // CORS Konfiguration: Erlaubt Anfragen von http://127.0.0.1:5501 mit bestimmten Methoden und Headern
    let cors = warp::cors()
        .allow_origin("http://127.0.0.1:5501")
        .allow_origin("http://127.0.0.1:5555")
        .allow_methods(vec!["POST", "GET", "OPTIONS"])
        .allow_headers(vec!["Content-Type", "*"])
        .build();

    // -------------------- Routen Definitionen --------------------
    // Route für die Registrierung
    let register = {
        let pool = pool_clone.clone(); // Datenbankpool klonen
        warp::post()
            .and(warp::path("register")) // Pfad: /register
            .and(warp::body::json()) // Erwartet einen JSON-Body
            .and_then(move |auth: Auth| {
                // Übergibt die Auth-Daten an den Handler
                let pool = pool.clone(); // Datenbankpool klonen (innerhalb des Handlers)
                async move { register_handler(auth, &pool).await }
            })
            .with(cors.clone()) // CORS-Middleware hinzufügen
    };

    // Route für den Login
    let login = {
        let pool = pool_clone.clone(); // Datenbankpool klonen
        warp::post()
            .and(warp::path("login")) // Pfad: /login
            .and(warp::body::json()) // Erwartet einen JSON-Body
            .and_then(move |auth: Auth| {
                println!("login route gestartet"); // Debugging-Ausgabe

                // Übergibt die Auth-Daten an den Handler
                let pool = pool.clone(); // Datenbankpool klonen (innerhalb des Handlers)
                async move { login_handler(auth, &pool).await }
            })
            .with(cors.clone()) // CORS-Middleware hinzufügen
    };

    // Route für das Erstellen von Tickets
    let tickets = {
        let pool = pool_clone.clone(); // Datenbankpool klonen
        warp::post()
            .and(warp::path("tickets")) // Pfad: /tickets
            .and(warp::body::json()) // Erwartet einen JSON-Body
            .and_then(move |ticket| {
                let pool = pool.clone(); // Datenbankpool klonen (innerhalb des Handlers)
                async move { create_ticket_handler(ticket, &pool).await }
            })
            .with(cors.clone()) // CORS-Middleware hinzufügen
    };

    // Route für das Abrufen von Tickets nach E-Mail
    let tickets_by_email = {
        let pool = pool_clone.clone(); // Datenbankpool klonen
        warp::get()
            .and(warp::path("tickets")) // Pfad: /tickets
            .and(warp::path("by_email"))
            .and(warp::path::param::<String>())
            .and_then(move |email: String| {
                // Übergibt die E-Mail an den Handler
                let pool = pool.clone(); // Datenbankpool klonen (innerhalb des Handlers)
                async move { get_tickets_by_email_handler(email, &pool).await }
            })
            .with(cors.clone()) // CORS-Middleware hinzufügen
    };

    // Route für das Abrufen eines Tickets nach ID
    let ticket_by_id = {
        let pool = pool_clone.clone(); // Datenbankpool klonen
        warp::get()
            .and(warp::path("tickets"))
            .and(warp::path::param::<u32>()) // Nimmt die ID als Pfadparameter entgegen
            .and_then(move |id: u32| {
                // Übergibt die ID an den Handler
                let pool = pool.clone(); // Datenbankpool klonen (innerhalb des Handlers)
                async move { get_ticket_by_id_handler(id, &pool).await }
            })
            .with(cors.clone()) // CORS-Middleware hinzufügen
    };

    let all_tickets = {
        let pool = pool_clone.clone(); // Datenbankpool klonen
        warp::get()
            .and(warp::path("tickets")) // Pfad: /tickets
            .and(warp::path("all_tickets")) // Pfad: /tickets/all
            .and_then(move || {
                // Übergibt die E-Mail an den Handler
                let pool = pool.clone(); // Datenbankpool klonen (innerhalb des Handlers)
                async move { get_all_tickets_handler(&pool).await }
            })
            .with(cors.clone()) // CORS-Middleware hinzufügen
    };

    // -------------------- Routen zusammenführen --------------------
    // Alle Routen mit 'or' kombinieren
    let routes = register
        .or(login)
        .or(tickets)
        .or(tickets_by_email)
        .or(ticket_by_id)
        .or(all_tickets);

    let routes = routes.with(cors.clone()); // CORS-Middleware hinzufügen
                                            // Logging-Middleware hinzufügen
    let log = warp::log("ticketsystem");

    // Server wird gestartet mit dem Port 5555
    warp::serve(routes.with(log))
        .run(([127, 0, 0, 1], 5555))
        .await;
}
