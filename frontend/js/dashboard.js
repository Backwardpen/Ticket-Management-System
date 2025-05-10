document.addEventListener('DOMContentLoaded', () => {
    const email = localStorage.getItem('userEmail');
    const greetingElement = document.getElementById('greeting');
    greetingElement.textContent = `Willkommen, ${email || 'Gast'}!`; // Erstellt eine Willkommensnachricht mit der Email des Benutzers

    loadTickets();

    // Suchfunktion auf der Seite
    const searchInput = document.getElementById('search');
    searchInput.addEventListener('input', () => {
        const term = searchInput.value.toLowerCase();
        const tickets = document.querySelectorAll('#activeTickets .ticket');

        // Filtert die Tickets basierend auf dem Suchbegriff
        // Der Suchbegriff kann in den Ticket-Titeln, Namen, IDs und Räumen vorkommen
        tickets.forEach(ticket => {
            const title = ticket.dataset.title;
            const name = ticket.dataset.name;
            const id = ticket.dataset.id;
            const raum = ticket.dataset.raum;

            const visible =
                title.includes(term) ||
                name.includes(term) ||
                id.includes(term) ||
                raum.includes(term);

            // Setzt die Sichtbarkeit der Tickets basierend auf dem Suchbegriff
            ticket.style.display = visible ? 'block' : 'none';
        });
    });
});

// Dropdown-Funktion
function toggleDropdown() {
    document.getElementById("dropdown-content").classList.toggle("show");
}

// Dropdown schließen beim Klick außerhalb
window.onclick = function (event) {
    if (!event.target.matches('.dropbtn')) {
        document.querySelectorAll(".dropdown-content").forEach(dropdown => {
            if (dropdown.classList.contains('show')) {
                dropdown.classList.remove('show');
            }
        });
    }
};

// Funktion zum Laden der Tickets im Dashboard
async function loadTickets() {
    const activeTicketsDiv = document.getElementById('activeTickets');
    const assignedTicketsDiv = document.getElementById('assignedTickets');
    const userEmail = localStorage.getItem('userEmail');

    // Schickt den Benutzer zurück zur Login-Seite, wenn keine Email vorhanden ist bzw jemand nicht eingeloggt ist
    if (!userEmail) {
        window.location.href = 'index.html';
        return;
    }

    try {
        const response = await fetch('http://10.10.10.194:5555/tickets/all_tickets');

        // Überprüft, ob die Antwort erfolgreich war und wenn nicht, wird ein Fehler geworfen
        if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

        const tickets = await response.json();

        // Überprüft, ob die Tickets ein Array sind und ob es leer ist
        // Wenn ja, wird eine Nachricht angezeigt, dass keine aktiven Tickets vorhanden sind
        if (!Array.isArray(tickets) || tickets.length === 0) {
            activeTicketsDiv.innerHTML = "<p>Keine aktiven Tickets vorhanden.</p>";
            assignedTicketsDiv.innerHTML = "<p>Keine zugewiesenen Tickets vorhanden.</p>";
            return;
        }

        // Aktive Tickets (status !== 'closed')
        const activeTickets = tickets.filter(ticket => ticket.status !== 'closed');

        // Zugeordnete Tickets (optional, falls verfügbar)
        const assignedTickets = tickets.filter(ticket => ticket.assigned_to === userEmail);

        // HTML generieren für aktive Tickets
        activeTicketsDiv.innerHTML = activeTickets.map(ticket => `
            <div class="ticket" 
                data-title="${ticket.ticket_title.toLowerCase()}" 
                data-name="${ticket.name.toLowerCase()}" 
                data-id="${ticket.id}" 
                data-raum="${ticket.raum.toLowerCase()}">
                
                <div class="ticket-header">
                    <h3>${ticket.ticket_title}</h3>
                    <div class="ticket-actions-top">
                        <button class="close-button" onclick="handleClose(${ticket.id})">✖</button>
                    </div>
                </div>

                <p><strong>Ticket-ID:</strong> ${ticket.id}</p>
                <p><strong>Name:</strong> ${ticket.name}</p>
                <p><strong>Email:</strong> ${ticket.email}</p>
                <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                <p><strong>Raum:</strong> ${ticket.raum}</p>
                <div class="ticket-actions-bottom">
                    <button class="edit-button" onclick="window.location.href='editTicket.html?id=${ticket.id}'">Bearbeiten</button>
                </div>
            </div>
        `).join('') || "<p>Keine aktiven Tickets vorhanden.</p>";

        // Platzhalter für zugewiesene Tickets
        assignedTicketsDiv.innerHTML = assignedTickets.map(ticket => `
            <div class="ticket-card">
                <h3>${ticket.ticket_title}</h3>
                <p><strong>Raum:</strong> ${ticket.raum}</p>
                <p><strong>Status:</strong> ${ticket.status || 'unbekannt'}</p>
            </div>
        `).join('') || "<p>Keine zugewiesenen Tickets vorhanden.</p>";

    } catch (error) {
        console.error("Fehler beim Laden der Tickets:", error);
        activeTicketsDiv.innerHTML = '<p>Fehler beim Laden der Tickets.</p>';
        assignedTicketsDiv.innerHTML = '<p>Fehler beim Laden der Tickets.</p>';
    }
}

// Ticket schließen Funktion
async function handleClose(ticketId) {
    // Erwartet eine Bestätigung vom Benutzer, damit keine Tickets versehentlich geschlossen werden
    const confirmed = confirm("Möchtest du dieses Ticket wirklich schließen?");
    if (!confirmed) return;

    try {
        const response = await fetch(`http://10.10.10.194:5555/tickets/close/${ticketId}`, {
            method: 'POST',
        });

        if (response.ok) {
            alert("Ticket wurde erfolgreich geschlossen.");
            loadTickets(); // Liste neu laden
        } else {
            const error = await response.json();
            alert("Fehler beim Schließen:\n" + (error.message || "Unbekannter Fehler"));
        }
    } catch (err) {
        console.error("Fehler beim Schließen:", err);
        alert("Netzwerkfehler beim Schließen des Tickets.");
    }
}