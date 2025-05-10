document.addEventListener('DOMContentLoaded', () => {
    const email = localStorage.getItem('userEmail');
    const greetingElement = document.getElementById('greeting');
    greetingElement.textContent = `Willkommen, ${email}!`;
});

function toggleDropdown() {
    document.getElementById("dropdown-content").classList.toggle("show");
}

// Close the dropdown if the user clicks outside of it
window.onclick = function (event) {
    if (!event.target.matches('.dropbtn')) {
        var dropdowns = document.getElementsByClassName("dropdown-content");
        for (let i = 0; i < dropdowns.length; i++) {
            var openDropdown = dropdowns[i];
            if (openDropdown.classList.contains('show')) {
                openDropdown.classList.remove('show');
            }
        }
    }
}

async function loadTickets() {
    const activeTicketsDiv = document.getElementById('activeTickets');
    const assignedTicketsDiv = document.getElementById('assignedTickets');
    const userEmail = localStorage.getItem('userEmail');

    if (!userEmail) {
        activeTicketsDiv.innerHTML = '<p>Bitte melde dich an.</p>';
        assignedTicketsDiv.innerHTML = '<p>Bitte melde dich an.</p>';
        return;
    }

    try {
        // Neue API-Route für alle Tickets
        const response = await fetch('http://127.0.0.1:5555/tickets/all_tickets',);

        if (!response.ok) {
            activeTicketsDiv.innerHTML = '<p>Es konnten keine Tickets geladen werden.</p>';
            assignedTicketsDiv.innerHTML = '<p>Es konnten keine Tickets geladen werden.</p>';
            console.error(`HTTP error! status: ${response.status}`);
            return;
        }

        const tickets = await response.json();
        console.log(tickets);

        if (!Array.isArray(tickets) || tickets.length === 0) {
            activeTicketsDiv.innerHTML = "<p>Keine aktiven Tickets vorhanden.</p>";
            assignedTicketsDiv.innerHTML = "<p>Keine zugewiesenen Tickets vorhanden.</p>";
            return;
        }

        // Eigene Tickets (erstellt vom Nutzer)
        let activeTicketsHtml = '';
        // Zugewiesene Tickets (dem Nutzer zugewiesen)
        let assignedTicketsHtml = '';

        tickets.forEach(ticket => {ticket.email === userEmail})
                activeTicketsHtml += `
                    <div class="ticket-card">
                        <h3>${ticket.ticket_title}</h3>
                        <p><strong>Name:</strong> ${ticket.name}</p>
                        <p><strong>Raum:</strong> ${ticket.raum}</p>
                        <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                    </div>
                `;
        ;

        activeTicketsDiv.innerHTML = activeTicketsHtml || "<p>Keine aktiven Tickets vorhanden.</p>";
        assignedTicketsDiv.innerHTML = assignedTicketsHtml || "<p>Keine zugewiesenen Tickets vorhanden.</p>";

    } catch (error) {
        console.error("Fehler beim Laden der Tickets:", error);
        activeTicketsDiv.innerHTML = '<p>Fehler beim Laden der Tickets.</p>';
        assignedTicketsDiv.innerHTML = '<p>Fehler beim Laden der Tickets.</p>';
    }
}

// Tickets beim Laden der Seite laden
document.addEventListener('DOMContentLoaded', loadTickets);