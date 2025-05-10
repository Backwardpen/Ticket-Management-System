use serde::{Deserialize, Serialize};

// Hier wird das Ticket-Modell definiert
// Für beim Erstellen eines Tickets werden die folgenden Felder als String benötigt:
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ticket {
    pub id: Option<i32>,
    pub ticket_title: String,
    pub email: String,
    pub name: String,
    pub ticket_description: String,
    pub raum: String,
    pub status: Option<String>,
}