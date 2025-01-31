use crate::error::CustomError;
use crate::models::auth::Auth;
use crate::models::ticket::Ticket;
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use mysql::prelude::*;
use mysql::Pool;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

// Defintion eines Claims-Structs, das die Daten für das JWT-Token enthält
// JWT Token sind Base64-kodierte JSON-Objekte, die signiert sind und Informationen über den Nutzer enthalten
// JWT Token werden verwendet, um den Nutzer zu authentifizieren und zu autorisieren

// JWT Token besteht aus Header, Claims und Signatur
// Header: Enthält Informationen über den Algorithmus und den Typ des Tokens
// Claims: Enthält die Nutzdaten des Tokens (z.B. Sub, Role, Exp)
// Signatur: Wird mit dem Secret und den Daten aus Header und Claims erstellt
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

// Funktion zum Erstellen eines Tickets in der Datenbank
pub async fn create_ticket_query(
    ticket: Ticket,
    mysql_pool: &Pool,
) -> Result<String, warp::Rejection> {
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
        .map(move |_| Ok::<_, mysql::Error>(conn.last_insert_id()))
    });

    match result {
        Ok(Ok(id)) => {
            let msg = format!("Ticket erfolgreich erstellt mit ID: {:?}", id);
            println!("{}", msg);
            Ok(msg)
        }
        Ok(Err(err)) => {
            eprintln!("Fehler beim Erstellen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError {
                message: format!("{:?}", err),
            }))
        }
        Err(err) => {
            eprintln!("Fehler beim Erstellen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError {
                message: format!("{:?}", err),
            }))
        }
    }
}

// Funktion zum Abrufen eines einzelnen Tickets über die ID
pub async fn get_ticket_by_id_query(
    id: u32,
    mysql_pool: &Pool,
) -> Result<Option<Ticket>, warp::Rejection> {
    let result = mysql_pool.get_conn().and_then(move |mut conn| {
        conn.exec_first::<(String, String, String, String, String), _, _>(
            "SELECT title, email, name, description, raum FROM tickets WHERE id = ?",
            (id,),
        )
    });

    match result {
        Ok(Some((ticket_title, email, name, ticket_description, raum))) => Ok(Some(Ticket {
            ticket_title,
            email,
            name,
            ticket_description,
            raum,
        })),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Fehler beim Abrufen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError {
                message: format!("{:?}", err),
            }))
        }
    }
}

// Funktion zum Abrufen von Tickets aus der Datenbank anhand der E-Mail-Adresse
pub async fn get_tickets_by_email_query(
    email: String,
    mysql_pool: &Pool,
) -> Result<Vec<Ticket>, warp::Rejection> {
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
            Err(warp::reject::custom(CustomError {
                message: format!("{:?}", err),
            }))
        }
    }
}

// Registrierung eines neuen Nutzers in der Anwendung
pub async fn register_user_query(auth: Auth, mysql_pool: &Pool) -> Result<Auth, warp::Rejection> {
    let email_clone = auth.email.clone();
    let existing_user = mysql_pool.get_conn().and_then(move |mut conn| {
        conn.exec_first::<(String,), _, _>(
            "SELECT email FROM users WHERE lower(email) = lower(?)",
            (email_clone,),
        )
    });

    match existing_user {
        Ok(Some(_)) => {
            eprintln!(
                "Ein Benutzer mit dieser Email existiert bereits: {}",
                auth.email
            );
            return Err(warp::reject::custom(CustomError {
                message: format!(
                    "Ein Benutzer mit dieser Email existiert bereits: {:?}",
                    auth.email
                ),
            }));
        }

        Ok(None) => {
            println!("Kein User mit Email gefunden: {}", auth.email);

            let password_hash_result = hash(auth.password.clone(), bcrypt::DEFAULT_COST);

            let password_hash = match password_hash_result {
                Ok(hash) => hash,
                Err(err) => {
                    eprintln!("Fehler beim Hashen des Passworts: {:?}", err);
                    return Err(warp::reject::custom(CustomError {
                        message: format!("Fehler beim Hashen des Passworts: {:?}", err),
                    }));
                }
            };
            let result = mysql_pool.get_conn().and_then(|mut conn| {
                conn.exec_drop(
                    "INSERT INTO users (email, username, password_hash) VALUES (?, ?, ?)",
                    (&auth.email, &auth.username, &password_hash),
                )
                .map(move |_| Ok::<_, mysql::Error>(conn.last_insert_id()))
            });

            match result {
                Ok(Ok(id)) => {
                    let msg = format!(
                        "User erfolgreich erstellt mit ID: {:?}; Username: {:?}; Email: {:?}",
                        id, auth.username, auth.email
                    );
                    println!("{}", msg);
                    Ok(auth)
                }
                Ok(Err(err)) => {
                    eprintln!("Fehler beim Erstellen des User: {:?}", err);
                    Err(warp::reject::custom(CustomError {
                        message: format!("{:?}", err),
                    }))
                }
                Err(err) => {
                    eprintln!("Fehler beim Erstellen des User: {:?}", err);
                    Err(warp::reject::custom(CustomError {
                        message: format!("{:?}", err),
                    }))
                }
            }
        }

        Err(err) => {
            eprintln!("Fehler beim Finden des Users: {:?}", err);
            Err(warp::reject::custom(CustomError {
                message: format!("{:?}", err),
            }))
        }
    }
}

// Funktion zum Einloggen eines Nutzers in die Anwendung
pub async fn login_user_query(auth: Auth, mysql_pool: &Pool) -> Result<String, warp::Rejection> {
    let email_clone = auth.email.clone();
    let result = mysql_pool.get_conn().and_then(move |mut conn| {
        conn.exec_first::<(String, String), _, _>(
            "SELECT password_hash, role FROM users WHERE lower(email) = lower(?)",
            (email_clone,),
        )
    });
    let (password_hash, role) = match result {
        Ok(Some(tuple)) => tuple,
        Ok(None) => {
            return Err(warp::reject::custom(CustomError {
                message: "User nicht gefunden".to_string(),
            }));
        }
        Err(err) => {
            eprintln!("Fehler beim Abrufen des Users: {:?}", err);
            return Err(warp::reject::custom(CustomError {
                message: format!("{:?}", err),
            }));
        }
    };

    // Überprüfen des Passworts
    // Falls das Passwort korrekt ist, wird ein JWT-Token erstellt und zurückgegeben
    // 3600 für 1 Stunde Gültigkeit
    // Die Gültigkeit des Tokens wird in der Claims-Struktur festgelegt
    match verify(auth.password, &password_hash) {
        Ok(true) => {
            let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET muss gesetzt sein");
            let exp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
                + 3600;
            let claims = Claims {
                sub: auth.email,
                role,
                exp,
            };
            let token_result = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_ref()),
            );
            match token_result {
                Ok(token) => return Ok(token),
                Err(err) => {
                    eprintln!("Fehler beim Erstellen des Tokens: {:?}", err);
                    return Err(warp::reject::custom(CustomError {
                        message: format!("{:?}", err),
                    }));
                }
            }
        }
        Ok(false) => {
            eprintln!("Falsches Passwort eingegeben");
            Err(warp::reject::custom(CustomError {
                message: "Falsches Passwort eingegeben".to_string(),
            }))
        }
        Err(err) => {
            eprintln!("Fehler beim Überprüfen des Passworts: {:?}", err);
            Err(warp::reject::custom(CustomError {
                message: format!("{:?}", err),
            }))
        }
    }
}