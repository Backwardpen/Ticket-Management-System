## Frontend
### Pages

## Index
    ✅ Login
    - Registrieren muss noch verschoben werden aufs Dashboard, da der Zugang nur für Admins ist
✅ createTicket

## ViewTickets
    - Tickets anzeigen
    - Tickets bearbeiten
    - Tickets löschen
    - Tickets schließen/archivieren

## createTicket
    - wenn eine Emailadresse keine Tickets hat, dann sollte man auch nicht weitergeleitet werden auf viewTickets

## Grundlegendes
✅ Footer mit Credentials
✅ Header auf alle Seiten mit Buttons für zurück und Logout 

## Dashboard
- Anzeige aller Tickets (aktive Tickets)
- Willkommensnachricht ist noch nicht korrekt (JSON-String statt Name/Email)
- Profil-Button muss noch angepasst werden
    - Falsche Position

## User Management
- Anzeigen aller vorhandenen User
- Löschen von Usern
- User umbenennen
    - Username selber erstellen
- Einstellen der User Berechtigungen

## Archivierte Tickets
- Anzeigen aller archivierten Tickets

## Backend

### User
- Berechtigungen
    - Admin/Techniker
    - Standard User
✅ Passwörter
    ✅ Hashen
- Username
- Einstellungen
    - Passwort ändern

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
✅ Email des Ticketerstellers
- Tags
    - Status
    - Kategorisierung
- Tickets schließen
- Tickets bearbeiten
- Tickets löschen

### User Funktionalität
## Datenbank (MySQL)
- Tickets
    ✅ Id
    ✅ Name
    ✅ Datum
    ✅ Inhalt
    ✅ Email
    - Tags
    - Kategorie
    - Status
- User
    ✅ Id
    ✅ Email
    ✅ Passwort
    - Berechtigungen (Rollen)
        - Berechtigungen soll im Adminbereich angepasst werden können

### Dashboard(viewTickets) User
✅ Übersicht offener Tickets
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
- Leitet Anfragen vom Frontend an die Datenbank weiter nachdem auf Validität geprüft wird saniert wird
- Sorgt dafür, dass Anmeldungen möglich sind

#### Web Server
- Sendet über Js Anfragen an den Backend Server