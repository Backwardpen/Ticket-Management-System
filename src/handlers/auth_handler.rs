use crate::db::query::{login_user_query, create_user_query};
use crate::models::auth::Auth;
use mysql::Pool;
use warp::{http::StatusCode, reply, Rejection};

// Hier wird der Handler für die Registrierung eines Benutzers definiert
// register_handler nimmt ein Auth-Objekt und eine MySQL-Pool-Verbindung entgegen und gibt ein Result zurück, das entweder eine Antwort oder einen Fehler enthält
// Dieser Code soll dafür sorgen, dass der Handler bei einem erfolgreichen Durchlauf eine Antwort mit dem Statuscode 200 und einer Message als JSON zurückgibt
// Statuscode 200 bedeutet, dass die Anfrage erfolgreich war
pub async fn register_handler(auth: Auth, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match create_user_query(auth, mysql_pool).await {
        Ok(auth) => Ok(reply::with_status(reply::json(&auth), StatusCode::OK)), // Sende erfolgreiche Statusmeldung mit Message als Json an den Client.
        Err(err) => {
            let response = reply::with_status(
                reply::json(&format!("{:?}", err)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Ok(response)
        }
    }
}

// Hier wird der Handler für das Einloggen eines Benutzers definiert
// login_handler nimmt ein Auth-Objekt und eine MySQL-Pool-Verbindung entgegen und gibt ein Result zurück, das entweder eine Antwort oder einen Fehler enthält
// Dieser Code soll dafür sorgen, dass der Handler bei einem erfolgreichen Durchlauf eine Antwort mit dem Statuscode 200 und einem Token als JSON zurückgibt
// Statuscode 200 bedeutet, dass die Anfrage erfolgreich war
pub async fn login_handler(auth: Auth, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    println!("login gestartet (login_handler), mit Auth: {:?}", auth); // Debugging-Ausgabe
    match login_user_query(auth, mysql_pool).await {
        Ok(token) => {
            Ok(reply::with_status(reply::json(&token), StatusCode::OK)) // Sende die erfolgreiche Statusmeldung mit dem Token als Json an den Client zurück
        }
        Err(err) => {
            println!("login_user_query fehlgeschlagen mit Error: {:?}", err);
            let response =
                reply::with_status(reply::json(&format!("{:?}", err)), StatusCode::UNAUTHORIZED);
            Ok(response)
        }
    }
}