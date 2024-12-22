use warp::{Rejection, reply, http::StatusCode};
use mysql::Pool;
use crate::models::auth::Auth;
use crate::db::query::{register_user_query, login_user_query};

pub async fn register_handler(auth: Auth, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match register_user_query(auth, mysql_pool).await {
        Ok(msg) => Ok(reply::with_status(reply::json(&msg), StatusCode::OK)), // Sende erfolgreiche Statusmeldung mit Message als Json an den Client.
        Err(err) =>  { // fange alle errors hier und sende den Client den Error in einer bestimmten Formatierung
            let response = reply::with_status(reply::json(&format!("{:?}", err)), StatusCode::INTERNAL_SERVER_ERROR);
            Ok(response)
        },
    }
}

pub async fn login_handler(auth: Auth, mysql_pool: &Pool) -> Result<impl warp::Reply, Rejection> {
    match login_user_query(auth, mysql_pool).await {
        Ok(token) => Ok(reply::with_status(reply::json(&token), StatusCode::OK)), // Sende erfolgreiche Statusmeldung mit Token als Json an den Client
        Err(err) =>  {   // fange alle errors hier und sende den Client den Error in einer bestimmten Formatierung
            let response = reply::with_status(reply::json(&format!("{:?}", err)), StatusCode::UNAUTHORIZED);
            Ok(response)
        },
    }
}