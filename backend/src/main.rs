// Importieren der benötigten Module 
// Zuerst die Module, die in den anderen Dateien definiert sind
// Dann die externen Module
mod db;
mod error;
mod handlers;
mod models;

use models::auth::Auth;
use db::connection::get_db_pool;
use handlers::auth_handler::{login_handler, register_handler, get_all_users_handler, delete_user_handler};
use handlers::ticket_handler::{create_ticket_handler, get_all_tickets_handler, get_ticket_by_id_handler, get_tickets_by_email_handler, get_archived_tickets_handler};
use warp::Filter;


// Hier fängt die Hauptfunktion des Backends an
// Diese Funktion wird asynchron ausgeführt, um die Warp-Serverbibliothek zu verwenden
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Datenbankpool erstellen (wird für alle Handler wiederverwendet)
    let mysql_pool = get_db_pool().await;
    // Datenbankverbindungspool klonen, damit er in den Handler verwendet werden kann
    let pool_clone = mysql_pool.clone();

    // CORS Konfiguration: Erlaubt Anfragen von http://127.0.0.1:5501 mit bestimmten Methoden und Headern
    // Diese Konfiguration ist wichtig, um Cross-Origin Resource Sharing zu ermöglichen und um sicherzustellen, dass der Server Anfragen von anderen Ursprüngen akzeptiert
    let cors = warp::cors()
    .allow_origin("http://10.10.10.194:8080")
    .allow_origin("http://localhost:8080")
    .allow_methods(vec!["POST", "GET", "OPTIONS", "DELETE"])
    .allow_headers(vec!["Content-Type"])
    .build();

    // -------------------- Routen Definitionen --------------------
    // Route für die Registrierung
    let register = {
        let pool = pool_clone.clone(); 
        warp::post()
            .and(warp::path("register"))
            .and(warp::body::json()) 
            .and_then(move |auth: Auth| {
                // Übergibt die Auth-Daten an den Handler
                let pool = pool.clone(); 
                async move{register_handler(auth, &pool).await}
            })
        .with(cors.clone())
    };

    // Route für den Login
    let login = {
        let pool = pool_clone.clone(); 
        warp::post()
            .and(warp::path("login"))
            .and(warp::body::json()) 
            .and_then(move |auth: Auth| {
                //println!("login route gestartet"); // Debugging-Ausgabe

                // Übergibt die Auth-Daten an den Handler
                let pool = pool.clone(); 
                async move{login_handler(auth, &pool).await }
            })
        .with(cors.clone())
    };

    // Route für das Erstellen von Tickets
    let tickets = {
        let pool = pool_clone.clone(); 
        warp::post()
            .and(warp::path("tickets"))
            .and(warp::body::json()) 
            .and_then(move |ticket| {
                let pool = pool.clone(); 
                async move{create_ticket_handler(ticket, &pool).await}
            })
        .with(cors.clone())
    };

    // Route für das Abrufen von Tickets nach E-Mail
    let tickets_by_email = {
        let pool = pool_clone.clone(); 
        warp::get()
            .and(warp::path("tickets"))
            .and(warp::path("by_email"))
            .and(warp::path::param::<String>())
            .and_then(move |email: String| {
                // Übergibt die E-Mail an den Handler
                let pool = pool.clone(); 
                async move{get_tickets_by_email_handler(email, &pool).await}
            })
        .with(cors.clone())
    };

    // Route für das Abrufen eines Tickets nach ID
    let ticket_by_id = {
        let pool = pool_clone.clone(); 
        warp::get()
            .and(warp::path("tickets"))
            .and(warp::path::param::<u32>()) // Nimmt die ID als Pfadparameter entgegen -- Wird genutzt, weil die Id in der URL steht und wir somit den Pfad leicht anpassen können
            .and_then(move |id: u32| {
                // Übergibt die ID an den Handler
                let pool = pool.clone(); 
                async move{get_ticket_by_id_handler(id, &pool).await}
            })
        .with(cors.clone())
    };

    // Route für das Abrufen aller Tickets
    let all_tickets = {
        let pool = pool_clone.clone(); 
        warp::get()
            .and(warp::path("tickets"))
            .and(warp::path("all_tickets"))
            .and_then(move || {
                // Übergibt die E-Mail an den Handler
                let pool = pool.clone(); 
                async move { get_all_tickets_handler(&pool).await }
            })
        .with(cors.clone()) 
    };

    // Route für das Abrufen aller Benutzer
    let get_all_users = {
        let pool = pool_clone.clone(); 
        warp::get()
            .and(warp::path("users"))
            .and_then(move || {
                // Übergibt die E-Mail an den Handler
                let pool = pool.clone(); 
                async move { get_all_users_handler(&pool).await }
            })
        .with(cors.clone()) 
    };

    // Route für das Abrufen aller archivierten Tickets
    // Noch nicht implementiert und vollständig
    let archived_tickets = {
        let pool = pool_clone.clone();
        warp::get()
            .and(warp::path("tickets"))
            .and(warp::path("archived"))
            .and_then(move || {
                let pool = pool.clone();
                async move {get_archived_tickets_handler(&pool).await }
            })
        .with(cors.clone())
    };

    // Route für das Löschen eines Benutzers
    let delete_user = {
        let pool = pool_clone.clone();
        warp::delete()
            .and(warp::path("users"))
            .and(warp::path("delete"))
            .and(warp::path::param::<String>()) // Nimmt die E-Mail als Pfadparameter entgegen zur Identifikation des Benutzers
            .and_then(move | email: String| {
                let pool = pool.clone();
                async move{delete_user_handler(email, &pool).await }
            })
        .with(cors.clone())
    };

    // -------------------- Routen zusammenführen --------------------
    // Alle Routen mit 'or' kombinieren
    // Dies bedeutet, dass die erste Route, die mit der Anfrage übereinstimmt, verwendet wird
    // Wenn keine Route übereinstimmt, wird eine 404-Fehlerseite zurückgegeben aka "Not Found" oder index.html, da dies meine Fallbackseite ist
    let routes = register
        .or(login)
        .or(tickets)
        .or(tickets_by_email)
        .or(ticket_by_id)
        .or(all_tickets)
        .or(archived_tickets)
        .or(get_all_users)
        .or(delete_user);

    // CORS-Filter auf alle Routen anwenden
    let routes = routes.with(cors.clone()); 
    // Logging für alle Routen aktivieren
    let log = warp::log("ticketsystem");

    // Server wird gestartet mit dem Port 5555
    // Der Server lauscht auf allen IP-Adressen mittels des 0.0.0.0-Adressraums
    warp::serve(routes.with(log))
        .run(([0, 0, 0, 0], 5555))
        .await;
}