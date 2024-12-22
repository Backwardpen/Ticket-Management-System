use mysql::Pool;
use crate::models::ticket::Ticket;
use crate::error::CustomError;
use mysql::prelude::*;
use bcrypt::{hash, verify};
use crate::models::auth::Auth;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

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
pub async fn create_ticket_query(ticket: Ticket, mysql_pool: &Pool) -> Result<String, warp::Rejection> {
    println!("Eingehendes Ticket: {:?}", ticket);
    let result = mysql_pool.get_conn().and_then(|mut conn| {
        println!("SQL Query wird ausgeführt mit Ticket-Daten: {:?}", ticket);
        conn.exec_drop( // exec_drop wird verwendet, wenn keine Daten zurückgegeben werden
            "INSERT INTO tickets (title, email, name, description, raum) VALUES (?, ?, ?, ?, ?)", // SQL Query mit Platzhaltern
            ( // Werte für die Platzhalter
                &ticket.ticket_title, 
                &ticket.email,
                &ticket.name,
                &ticket.ticket_description,
                &ticket.raum,
            ),
        ).map(move |_| Ok::<_, mysql::Error>(conn.last_insert_id())) // Gibt die ID des eingefügten Datensatzes zurück
    });

    match result { // Ergebnis der Datenbankabfrage verarbeiten
        Ok(Ok(id)) => { // Wenn die Datenbankabfrage erfolgreich war
            let msg = format!("Ticket erfolgreich erstellt mit ID: {:?}", id);
            println!("{}", msg);
            Ok(msg)
        }
        Ok(Err(err)) => { // Wenn ein Fehler bei der Datenbankabfrage/übertragung aufgetreten ist
            eprintln!("Fehler beim Erstellen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
        Err(err) => { // Wenn ein allgemeiner Fehler aufgetreten ist
            eprintln!("Fehler beim Erstellen des Tickets: {:?}", err);
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
    }
}

// Funktion zum Abrufen von Tickets aus der Datenbank anhand der E-Mail-Adresse
pub async fn get_tickets_by_email_query(email: String, mysql_pool: &Pool) -> Result<Vec<Ticket>, warp::Rejection> {
    println!("Email die abgerufen werden soll: {}", email); // E-Mail-Adresse, die versucht wird abzurufen, in der Konsole ausgeben
    let result = mysql_pool.get_conn().and_then(move |mut conn| { // Datenbankverbindung herstellen
        conn.exec_map( // exec_map wird verwendet, wenn Daten zurückgegeben werden
            "SELECT title, email, name, description, raum FROM tickets WHERE lower(email) = lower(?)", // SQL Query mit Platzhaltern
            (email,), // Werte für die Platzhalter
            |(ticket_title, email, name, ticket_description, raum)| Ticket { 
                ticket_title,
                email,
                name,
                ticket_description,
                raum,
            },
        )
    });

    match result { // Ergebnis der Datenbankabfrage verarbeiten
        Ok(tickets) => Ok(tickets),
        Err(err) => { 
            eprintln!("Fehler beim Abrufen der Tickets: {:?}", err); // Fehler in der Konsole ausgeben
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
    }
}

// Funktion zum Registrieren eines Nutzers in der Datenbank
pub async fn register_user_query(auth: Auth, mysql_pool: &Pool) -> Result<String, warp::Rejection> {
    let email_clone = auth.email.clone(); // E-Mail-Adresse klonen, um sie in der Closure zu verwenden
    let existing_user = mysql_pool.get_conn().and_then(move |mut conn| { // Datenbankverbindung herstellen
        conn.exec_first::<(String,), _, _>( // exec_first wird verwendet, wenn nur ein Datensatz zurückgegeben wird
            "SELECT email FROM users WHERE lower(email) = lower(?)", // SQL Query mit Platzhaltern
            (email_clone,),
        )
    });

    match existing_user { // Ergebnis der Datenbankabfrage verarbeiten
        Ok(Some(_)) => {
            eprintln!("Ein Benutzer mit dieser Email existiert bereits: {}", auth.email);
            return  Err(warp::reject::custom(CustomError { message: format!("Ein Benutzer mit dieser Email existiert bereits: {:?}", auth.email) }))
        },
        Ok(None) => {  
            println!("Kein User mit Email gefunden: {}", auth.email);
            let password_hash_result = hash(auth.password, bcrypt::DEFAULT_COST); // Passwort hashen

            let password_hash = match password_hash_result { // Ergebnis des Hashens verarbeiten
                Ok(hash) => hash, // Wenn das Hashen erfolgreich war
                Err(err) => { // Wenn ein Fehler beim Hashen aufgetreten ist
                    eprintln!("Fehler beim Hashen des Passworts: {:?}", err); // Fehler in der Konsole ausgeben
                    return Err(warp::reject::custom(CustomError { message: format!("Fehler beim Hashen des Passworts: {:?}", err) })); // Fehler an den Client zurückgeben und Funktion beenden
                }
            };

            let result = mysql_pool.get_conn().and_then(|mut conn| { // Datenbankverbindung herstellen
                conn.exec_drop( // exec_drop wird verwendet, wenn keine Daten zurückgegeben werden
                    "INSERT INTO users (email, password_hash) VALUES (?, ?)", // SQL Query mit Platzhaltern
                    (&auth.email, &password_hash), // Werte für die Platzhalter
                ).map(move |_| Ok::<_, mysql::Error>(conn.last_insert_id())) // Gibt die ID des eingefügten Datensatzes zurück
            });

            match result { // Ergebnis der Datenbankabfrage verarbeiten
                Ok(Ok(id)) => { // Wenn die Datenbankabfrage erfolgreich war
                    let msg = format!("User erfolgreich erstellt mit ID: {:?}", id);
                    println!("{}", msg);
                    Ok(msg)
                }
                Ok(Err(err)) => { // Wenn ein Fehler bei der Datenbankabfrage/übertragung aufgetreten ist
                    eprintln!("Fehler beim Erstellen des User: {:?}", err);
                    Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
                }
                Err(err) => { // Wenn ein allgemeiner Fehler aufgetreten ist
                eprintln!("Fehler beim Erstellen des User: {:?}", err);
                Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
                }
            }
        }
        Err(err) => { // Wenn ein allgemeiner Fehler bei der Suche aufgetreten ist
            eprintln!("Fehler beim Finden des Users: {:?}", err);
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
    }
}

// Funktion zum Einloggen eines Nutzers in die Anwendung
pub async fn login_user_query(auth: Auth, mysql_pool: &Pool) -> Result<String, warp::Rejection> { 
    let email_clone = auth.email.clone(); // E-Mail-Adresse klonen, um sie in der Closure zu verwenden
    let result = mysql_pool.get_conn().and_then(move |mut conn| { // Datenbankverbindung herstellen
        conn.exec_first::<(String, String), _, _>( // exec_first wird verwendet, wenn nur ein Datensatz zurückgegeben wird
            "SELECT password_hash, role FROM users WHERE lower(email) = lower(?)", // SQL Query mit Platzhaltern
            (email_clone,), // Werte für die Platzhalter
        )
    });

    // Ergebnis der Datenbankabfrage verarbeiten
    let (password_hash, role) = match result {
         Ok(Some(tuple)) => tuple, // Wenn ein Datensatz zurückgegeben wurde
         Ok(None) => { // Wenn kein Datensatz zurückgegeben wurde
            return Err(warp::reject::custom(CustomError { message: "User nicht gefunden".to_string() }));
         }
         Err(err) => { // Wenn ein Fehler bei der Datenbankabfrage/übertragung aufgetreten ist
             eprintln!("Fehler beim Abrufen des Users: {:?}", err);
            return Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }));
         }
    };

    // Überprüfen, ob das eingegebene Passwort mit dem gehashten Passwort übereinstimmt
    match verify(auth.password, &password_hash) {
        Ok(true) => { // Wenn das Passwort korrekt ist
            let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET muss gesetzt sein"); // JWT Secret aus der Umgebungsvariable laden
            // Gültigkeit des Tokens: Aktuelle Zeit + 1 Stunde (via SystemTime und UNIX_EPOCH)
            let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + 3600;
            let claims = Claims { // Claims-Objekt erstellen
                sub: auth.email, // E-Mail-Adresse des Nutzers
                role, // Rolle des Nutzers
                exp, // Gültigkeit des Tokens
            };

            // Token erstellen
            let token_result = encode( 
                &Header::default(), // Standard-Header für den Token
                &claims, // Claims-Objekt für den Token 
                &EncodingKey::from_secret(jwt_secret.as_ref()) // Secret für die Signatur des Tokens
            );

           match token_result { // Ergebnis des Token-Erstellens verarbeiten
                Ok(token) => { // Wenn das Token erfolgreich erstellt wurde
                    return Ok(token)
                },
                Err(err) => { // Wenn ein Fehler beim Erstellen des Tokens aufgetreten ist
                   eprintln!("Fehler beim Erstellen des Tokens: {:?}", err);
                   return Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }));
                }
            }
        }
        Ok(false) => { // Wenn das Passwort falsch ist
            eprintln!("Falsches Passwort eingegeben");
            Err(warp::reject::custom(CustomError { message: "Falsches Passwort eingegeben".to_string() }))
        }
        Err(err) => { // Wenn ein Fehler beim Überprüfen des Passworts aufgetreten ist
            eprintln!("Fehler beim Überprüfen des Passworts: {:?}", err);
            Err(warp::reject::custom(CustomError { message: format!("{:?}", err) }))
        }
    }
}