// Importieren der benötigten Module
use warp::reject::Reject;

// Hier wird ein eigener Fehler definiert, der die Trait Reject implementiert
// Hier wird die Struktur CustomError definiert, die eine Nachricht enthält
// Dies mache ich, um benutzerdefinierte Fehler zu erstellen, die in der Anwendung verwendet werden können
// Grund: Deutlichere Erleichterung in Abläufen, die mit Fehlern zu tun haben
#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

// Hier wird die Implementierung der Trait Display für CustomError definiert
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { // Mit fmt wird die Nachricht des CustomError-Objekts formatiert1
        write!(f, "{}", self.message) // Hier wird die Nachricht des CustomError-Objekts geschrieben und zurückgegeben
    }
}

impl std::error::Error for CustomError {} // Hier wird die Implementierung der Trait Error für CustomError definiert
impl Reject for CustomError {} // Hier wird die Implementierung der Trait Reject für CustomError definiert