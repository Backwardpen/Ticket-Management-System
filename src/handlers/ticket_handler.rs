use warp::{Rejection, reply, http::StatusCode};
use mysql::Pool;
use crate::models::ticket::Ticket;
use crate::db::query::{create_ticket_query, get_tickets_by_email_query};

// create_ticket_handler nimmt ein Ticket-Objekt und eine MySQL-Pool-Verbindung entgegen und gibt ein Result zurück, das entweder eine Antwort oder einen Fehler enthält.
// Wenn die Abfrage erfolgreich ist, wird ein Statuscode 200 (OK) zurückgegeben, andernfalls ein Statuscode 500 (Internal Server Error).
// Dieses Prinzip nutzt get_tickets_by_email_handler ebenfalls.
pub async fn create_ticket_handler(ticket: Ticket, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
     match create_ticket_query(ticket, mysql_pool).await {
        Ok(msg) => Ok(reply::with_status(reply::json(&msg), StatusCode::OK)),
        Err(err) =>  {
            let response = reply::with_status(reply::json(&format!("{:?}", err)), StatusCode::INTERNAL_SERVER_ERROR);
            Ok(response)
        },
    }   
}

// Dasselbe System wie bei create_ticket_handler wird hier verwendet, um die Tickets nach E-Mail-Adresse abzurufen.
pub async fn get_tickets_by_email_handler(email: String, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match get_tickets_by_email_query(email, mysql_pool).await {
        Ok(tickets) => Ok(reply::with_status(reply::json(&tickets), StatusCode::OK)),
        Err(err) => Ok(reply::with_status(reply::json(&format!("{:?}", err)), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}