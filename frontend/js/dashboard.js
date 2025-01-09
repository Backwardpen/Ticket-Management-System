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
        const response = await fetch(`http://127.0.0.1:5555/tickets/by_email/${userEmail}`);

        if (!response.ok) {
            activeTicketsDiv.innerHTML = '<p>Es konnten keine Tickets geladen werden.</p>';
            assignedTicketsDiv.innerHTML = '<p>Es konnten keine Tickets geladen werden.</p>';

            console.error(`HTTP error! status: ${response.status}`);
            return;
        }

        const tickets = await response.json();
        console.log(tickets);

        if (tickets.length === 0) {
            activeTicketsDiv.innerHTML = "<p>Keine aktiven Tickets vorhanden.</p>";
            assignedTicketsDiv.innerHTML = "<p>Keine zugewiesenen Tickets vorhanden.</p>";
            return;
        }

        // Aufteilung der Tickets
        let activeTicketsHtml = '';
        let assignedTicketsHtml = '';

        tickets.forEach(ticket => {
            activeTicketsHtml += `
                <div class="ticket-card">
                    <h3>${ticket.ticket_title}</h3>
                    <p><strong>Name:</strong> ${ticket.name}</p>
                    <p><strong>Raum:</strong> ${ticket.raum}</p>
                    <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                </div>
           `;
            assignedTicketsHtml = activeTicketsHtml;
        });
        activeTicketsDiv.innerHTML = activeTicketsHtml;
        assignedTicketsDiv.innerHTML = assignedTicketsHtml;

    } catch (error) {
        console.error("Fehler beim Laden der Tickets:", error);

        activeTicketsDiv.innerHTML = '<p>Fehler beim Laden der Tickets.</p>';
        assignedTicketsDiv.innerHTML = '<p>Fehler beim Laden der Tickets.</p>';
    }
}