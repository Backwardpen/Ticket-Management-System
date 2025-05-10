// Importieren der benötigten Module
use crate::db::query::{create_ticket_query, get_all_tickets_query, get_ticket_by_id_query, get_tickets_by_email_query, get_archived_tickets_query};
use crate::models::ticket::Ticket;
use crate::error::CustomError;
use mysql::Pool;
use warp::{http::StatusCode, reply, Rejection};
use urlencoding::decode;

// create_ticket_handler nimmt ein Ticket-Objekt und eine MySQL-Pool-Verbindung entgegen und gibt ein Result zurück, das entweder eine Antwort oder einen Fehler enthält.
// Wenn die Abfrage erfolgreich ist, wird ein Statuscode 200 (OK) zurückgegeben, andernfalls ein Statuscode 500 (Internal Server Error).
// Dieses Prinzip nutzt get_tickets_by_email_handler und get_ticket_by_id_handler ebenfalls.
pub async fn create_ticket_handler(ticket: Ticket, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match create_ticket_query(ticket, mysql_pool).await {
        Ok(msg) => Ok(reply::with_status(reply::json(&msg), StatusCode::OK)),
        Err(err) => {
            //Error code aus db-abfrage (nicht connecten usw).
            let response = reply::with_status(
                reply::json(&format!("{:?}", err)),
                StatusCode::INTERNAL_SERVER_ERROR,
            ); //response code setten ! (500)
            Ok(response)
        }
    }
}

// Dasselbe System wie bei create_ticket_handler wird hier verwendet, um die Tickets nach E-Mail-Adresse abzurufen.
pub async fn get_tickets_by_email_handler(email: String, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    let decoded_email = match decode(&email) {
        Ok(decoded) => decoded.into_owned(),
        Err(_) => {
            return Err(warp::reject::custom(CustomError {
                message: "Ungültige E-Mail-Kodierung".into(),
            }));
        }
    };

    match get_tickets_by_email_query(decoded_email, mysql_pool).await {
        Ok(tickets) => Ok(warp::reply::json(&tickets)),
        Err(err) => Err(err),
    }
}


// Hier wird ein Ticket anhand der ID geholt
pub async fn get_ticket_by_id_handler(id: u32, mysql_pool: &Pool,) -> Result<impl warp::Reply, Rejection> {
    match get_ticket_by_id_query(id, mysql_pool).await {
        Ok(ticket) => match ticket {
            Some(ticket) => Ok(reply::with_status(reply::json(&ticket), StatusCode::OK)),
            None => Ok(reply::with_status(
                reply::json(&"Ticket nicht gefunden"),
                StatusCode::NOT_FOUND,
            )),
        },
        Err(err) => Ok(reply::with_status(
            reply::json(&format!("{:?}", err)),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

// Hier wird die Funktion get_all_tickets_handler definiert, die alle Tickets abruft und zurückgibt.
pub async fn get_all_tickets_handler(mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match get_all_tickets_query(mysql_pool).await {
        Ok(tickets) => Ok(reply::with_status(reply::json(&tickets), StatusCode::OK)),
        Err(err) => Ok(reply::with_status(
            reply::json(&format!("{:?}", err)),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

// Hier wird die Funktion archived_ticket_handler definiert, der die Tickets archiviered und in archived.html verschiebt, sowie eine Antwort zurückgibt.
pub async fn get_archived_tickets_handler(mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match get_archived_tickets_query(mysql_pool).await {
        Ok(tickets) => Ok(warp::reply::json(&tickets)),
        Err(err) => Err(err),
    }
}