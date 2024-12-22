use serde::{Deserialize, Serialize};

// Hier wird das Ticket-Modell definiert
// Für beim Erstellen eines Tickets werden die folgenden Felder als String benötigt:
#[derive(Debug, Deserialize, Serialize)]
pub struct Ticket {
    pub ticket_title: String,
    pub email: String,
    pub name: String,
    pub ticket_description: String,
    pub raum: String,
}