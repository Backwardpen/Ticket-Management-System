// Wird ausgeführt, wenn das Dokument vollständig geladen ist
document.addEventListener('DOMContentLoaded', () => {
    // Holt sich die Tickets aus dem sessionStorage des Browsers
    const ticketsContainer = document.getElementById('tickets-container');
    const storedTickets = sessionStorage.getItem('tickets');
    // Falls Tickets vorhanden sind, wird dieser Code aufgerufen
    if (storedTickets) {
        const tickets = JSON.parse(storedTickets);
        tickets.forEach(ticket => {

            const ticketElement = document.createElement('div');
            ticketElement.classList.add('ticket');
            // Fügt die Ticket-Informationen in das HTML ein pro Ticket
            ticketElement.innerHTML = `
            <h3>${ticket.ticket_title}</h3>
                <p><strong>Name:</strong> ${ticket.name}</p>
                <p><strong>Email:</strong> ${ticket.email}</p>
                <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                <p><strong>Raum:</strong> ${ticket.raum}</p>
                <div class="ticket-actions">
                    <button class="edit-button">Bearbeiten</button>
                    <button class="edit-button">Schließen</button>
                </div>
            `;
            // Fügt das Ticket-Element in das ticketsContainer ein
            ticketsContainer.appendChild(ticketElement);
        });

        const editButtons = document.querySelectorAll('.edit-button');

        editButtons.forEach(button => {
            button.addEventListener('click', () => {
                const ticketId = button.getAttribute('data-id');
                window.location.href = `editTicket.html?id=${ticketId}`;
            });
        });
    } else {
        // Falls keine Tickets vorhanden sind, wird dieser Code aufgerufen
        ticketsContainer.innerHTML = '<p>Es sind keine Tickets für diese Email vorhanden</p>'
    }
});