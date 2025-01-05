document.addEventListener('DOMContentLoaded', () => {
    const ticketsContainer = document.getElementById('tickets-container');
    const storedTickets = sessionStorage.getItem('tickets');

    if (storedTickets) {
        const tickets = JSON.parse(storedTickets);
        tickets.forEach(ticket => {
            const ticketElement = document.createElement('div');
            ticketElement.classList.add('ticket');
            ticketElement.innerHTML = `
                <h3>${ticket.ticket_title}</h3>
                <p><strong>Name:</strong> ${ticket.name}</p>
                <p><strong>Email:</strong> ${ticket.email}</p>
                <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                <p><strong>Raum:</strong> ${ticket.raum}</p>
                <div class = "button-container">
                    <button class="edit-button"  data-ticket-id="${ticket.id}">Ticket bearbeiten</button>  
                    <button class="archive-button" data-ticket-id="${ticket.id}">Ticket schließen</button>
                </div>
            `
            ticketsContainer.appendChild(ticketElement);
        });
        // Event listener für alle "Bearbeiten"-Buttons
        document.querySelectorAll('.edit-button').forEach(button => {
            button.addEventListener('click', function () {
                const ticketId = this.getAttribute('data-ticket-id');
                window.location.href = `editTicket.html?ticketId=${ticketId}`;
            });
        });
        // Event listener für alle "Archivieren/Schließen"-Buttons
        document.querySelectorAll('.archive-button').forEach(button => {
            button.addEventListener('click', function () {
                const ticketId = this.getAttribute('data-ticket-id');
                archiveTicket(ticketId);
            });
        });
    }
    else {
        ticketsContainer.innerHTML = '<p>Keine Tickets vorhanden</p>';
    }
});