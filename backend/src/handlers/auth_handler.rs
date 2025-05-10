// Importieren der benötigten Module
use crate::db::query::{login_user_query, create_user_query, get_all_users_query, delete_user_query};
use crate::models::auth::Auth;
use mysql::Pool;
use warp::{http::StatusCode, reply, Rejection};
use urlencoding::decode;

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
    //println!("login gestartet (login_handler), mit Auth: {:?}", auth); // Debugging-Ausgabe
    
    match login_user_query(auth, mysql_pool).await {
        Ok(token) => {
            Ok(reply::with_status(reply::json(&token), StatusCode::OK)) // Sende die erfolgreiche Statusmeldung mit dem Token als Json an den Client zurück
        }
        Err(err) => {
            let response = reply::with_status(
                reply::json(&format!("{:?}", err)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Ok(response)
        }
    }
}

// Hier wird der Handler für das Abrufen aller Benutzer definiert
// get_all_users_handler nimmt eine MySQL-Pool-Verbindung entgegen und gibt ein Result zurück, das entweder eine Antwort oder einen Fehler enthält
// Dieser Code soll dafür sorgen, dass der Handler bei einem erfolgreichen Durchlauf eine Antwort mit dem Statuscode 200 und einer Liste von Benutzern als JSON zurückgibt
pub async fn get_all_users_handler(mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match get_all_users_query(mysql_pool).await {
        Ok(users) => Ok(reply::with_status(reply::json(&users), StatusCode::OK)), // Sende die erfolgreiche Statusmeldung mit der Liste von Benutzern als Json an den Client zurück
        Err(err) => {
            let response = reply::with_status(
                reply::json(&format!("{:?}", err)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Ok(response)
        }
    }
}

// Hier wird der Handler für das Löschen eines Benutzers definiert
// delete_user_handler nimmt eine MySQL-Pool-Verbindung und eine Benutzer-ID entgegen und gibt ein Result zurück, das entweder eine Antwort oder einen Fehler enthält
// Dieser Code soll dafür sorgen, dass der Handler bei einem erfolgreichen Durchlauf eine Antwort mit dem Statuscode 200 und einer Bestätigung als JSON zurückgibt
pub async fn delete_user_handler(email: String, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    // println!("delete_user_handler gestartet mit email: {:?}", email); // Debugging-Ausgabe

    let decoded_email = match decode(&email) {
        Ok(decoded) => decoded.into_owned(),
        Err(_) => {
            let response = reply::with_status(
                reply::json(&format!("Fehler beim Dekodieren der Email: {}", email)),
                StatusCode::BAD_REQUEST,
            );
            return Ok(response);
        }
    };

    // println!("Dekodierte Email: {}", decoded_email); // Debugging-Ausgabe

    match delete_user_query(decoded_email, mysql_pool).await {
        Ok(msg) => Ok(reply::with_status(warp::reply::json(&msg), StatusCode::OK)),
        Err(err) => Err(err),
    }
}