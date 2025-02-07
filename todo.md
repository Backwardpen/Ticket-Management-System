## Frontend
Aussehen noch ein wenig abändern, damit es etwas moderner aussieht

## Index
    ✅ Login
        - FIX LOGIN OR BAD
            - Username sollte nicht abgefragt werden, wird aber versucht abzufragen, da ich es für die Registrierung brauche
                --> Lösung: GET GOOD --> Backend Problem
    ✅ Registrieren muss noch verschoben werden aufs Dashboard, da der Zugang nur für Admins ist
    - Passwort vergessen?
    - Passwort falsch eingegeben --> Fehlermeldung oder Fehlermeldung

## ViewTickets
    ✅ Tickets anzeigen
    - Tickets bearbeiten
    - Tickets schließen/archivieren

## createTicket
    ✅ wenn eine Emailadresse keine Tickets hat, dann sollte man auch nicht weitergeleitet werden auf viewTickets

## editTickets
    - Tickets bearbeiten
        - direkt reinpasten von viewTickets mit allen Angaben
    - Tickets schließen/archivieren

## Grundlegendes
✅ Footer mit Credentials
✅ Header auf alle Seiten mit Buttons für zurück und Logout 

## Dashboard
- Anzeige aller Tickets (aktive Tickets)
✅ Willkommensnachricht ist noch nicht korrekt (JSON-String statt Name/Email)
✅ Profil-Button muss noch angepasst werden
    ✅ Falsche Position
- Des Weiteren sollten direkt die Email und der Name eingetragen sein, da man ja eingeloggt ist.
    - Eventuell muss hierfür eine neue Seite erstellt werden, die gleich aussieht wie das normale createTicket, aber den Funktionen des Dashboards entspricht.
- Ohne Login sollte man gar keinen Zugriff auf das Dashboard haben. 
    - Auch nicht als "Gast", da das den Sinn eines Logins untergräbt und deshalb das Konzept von "Gast" nur ein Fallback für einen Fehler sein soll
    - Wenn man auf eine Seite geht sucht man nach Login Daten und wenn diese nicht vorhanden sind und vom Server als valide zurückgegeben werden, dann wird man einfach wieder auf die Index.html geleitet

## User Management
- Löschen von Usern
- Einstellen der User Berechtigungen
- Benutzerliste richtig anzeigen lassen
    - Benutzerliste aus der Datenbank statt aus dem sessionStorage abfragen

## Archivierte Tickets
- Anzeigen aller archivierten Tickets


## Backend
### User
- Berechtigungen
    - Admin
    ✅ Standard User
✅ Passwörter
    ✅ Hashen
✅ Username
    - Passwort ändern

### Tickets
✅ Datum
✅ Inhalt
    ✅ Überschrift
    ✅ Text
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
        - Email hashing? --> Vorschlag von CS Students
    ✅ Passwort
    ✅ Berechtigungen (Rollen)
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