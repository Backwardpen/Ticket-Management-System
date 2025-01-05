## Frontend
### Pages

## Index
    ✅ Login
    - Registrieren muss noch verschoben werden aufs Dashboard, da der Zugang nur für Admins ist
✅ createTicket

## ViewTickets
<<<<<<< HEAD
    ✅ Tickets anzeigen
    - Tickets bearbeiten
=======
    - Tickets anzeigen
    - Tickets bearbeiten
    - Tickets löschen
>>>>>>> 1a68b1f3a3def8f8d440aa150609c392b4d668ac
    - Tickets schließen/archivieren

## createTicket
    - wenn eine Emailadresse keine Tickets hat, dann sollte man auch nicht weitergeleitet werden auf viewTickets
<<<<<<< HEAD

## editTickets
    - Tickets bearbeiten
        - direkt reinpasten von viewTickets mit allen Angaben
    - Tickets schließen/archivieren
=======
>>>>>>> 1a68b1f3a3def8f8d440aa150609c392b4d668ac

## Grundlegendes
✅ Footer mit Credentials
✅ Header auf alle Seiten mit Buttons für zurück und Logout 

## Dashboard
- Anzeige aller Tickets (aktive Tickets)
<<<<<<< HEAD
✅ Willkommensnachricht ist noch nicht korrekt (JSON-String statt Name/Email)
✅ Profil-Button muss noch angepasst werden
    ✅ Falsche Position
- Wenn man eingeloggt ist, dann sollte man wenn man ein neues Ticket erstellt nicht ausgeloggt werden, sondern im Dashboard bleiben.
- Des Weiteren sollten direkt die Email und der Name eingetragen sein, da man ja noch eingeloggt ist.
    - Eventuell muss hierfür eine neue Seite erstellt werden, die gleich aussieht wie das normale createTicket, aber den Funktionen des Dashboards entspricht.
- Wenn man nicht eingeloggt ist, sollte man gar keinen Zugriff auf das Dashboard haben. Auch nicht als Gast, da das den Sinn eines Logins untergräbt.
=======
- Willkommensnachricht ist noch nicht korrekt (JSON-String statt Name/Email)
- Profil-Button muss noch angepasst werden
    - Falsche Position
>>>>>>> 1a68b1f3a3def8f8d440aa150609c392b4d668ac

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

### Tickets
- Kategorien
✅ Datum
✅ Inhalt
    ✅ Überschrift
    ✅ Text
- Status
    - Offen
    - In Bearbeitung
    - Abgeschlossen
✅ Email des Ticketerstellers
- Tags
    - Status
    - Kategorisierung
- Tickets schließen
- Tickets bearbeiten

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
✅ Übersicht offener Tickets des Users
- Statistiken

### Dashboard(viewTickets) Admin
- Übersicht aller Tickets
- Statistiken
- Filteroptionen
- Suche nach Ticketname oder ID --> Funktion mittels Stichwortsuche?

### Benachrichtigungen
- Email-Benachrichtigungen

### Dokumentation
- Benutzerhandbuch
- Entwicklerdokumentation

**potentieller Raumplan mit Tickets verbinden und dann bei Klick auf den Raum alle Tickets anzeigen lassen.**
**Die Farben verändern sich wenn ein oder mehrere Tickets auf einem Raum liegen**
**Anhänge (optional) --> ungeeignet**
- Datei-Uploads
    - Fotos
    - Files
    - **Dateigrößen Beschränkung**: Am server als auch frontend per html file "select" attribute, kann eine limitierte Dateiauswahl oder ein File größen  limit festgelegt werden


## Ziggys Ansatz

### Server
#### Rust Backend
- Kontinuierlich laufender Server
- Leitet Anfragen vom Frontend an die Datenbank weiter nachdem auf Validität geprüft wird saniert wird
- Sorgt dafür, dass Anmeldungen möglich sind

#### Web Server
- Sendet über Js Anfragen an den Backend Server