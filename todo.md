## Frontend

### Homepage
- Login
✅ createTicket
- ViewTicket

## Grundlegendes
✅ Footer mit Credentials
✅ Header auf alle Seiten mit Buttons für zurück und Logout 


## Backend

### User
- Berechtigungen
    - Admin/Supporter
    - Standard User
- Passwortörter
    - Hashen

#### Rollen und Berechtigungen
- Unterschiedliche Rollen und Rechte
- Anpassbare Berechtigungen

### Tickets
- Kategorien
- Datum
- Inhalt
    - Überschrift
    - Bilder
    - Text
- Email des Ticketerstellers
- Tags
    - Status
    - Kategorisierung

### User Funktionalität
## Datenbank (MySQL)
- Tickets
- User
    - Id
    - Name
    - Email
    - Passwort
    - Berechtigungen (Rollen)


### Dashboard(viewTickets) User
- Übersicht offener Tickets
- Statistiken

### Dashboard(viewTickets) Admin
- Übersicht aller Tickets
- Statistiken
- Filteroptionen
- Suche nach Ticketname oder ID

### Benachrichtigungen
- Email-Benachrichtigungen

### Anhänge
- Datei-Uploads
    - Fotos
    - Files

### Dokumentation
- Benutzerhandbuch
- Entwicklerdokumentation

**potentieller Raumplan mit Tickets verbinden und dann bei Klick auf den Raum alle Tickets anzeigen lassen.**
**Die Farben verändern sich wenn ein oder mehrere Tickets auf einem Raum liegen**



## Ziggys Ansatz

### Server
#### Rust Backend
- Kontinuierlich laufender Server
- Leitet Anfragen vom Frontend an die Datenbank weiter nachdem auf validität geprüft wird saniert wird
- Sorgt dafür, dass anmeldungen möglich sind

#### Web Server
- Sendet über js anfragen an den Backend Server