<<<<<<< HEAD
document.addEventListener('DOMContentLoaded', () => {
    const ticketsContainer = document.getElementById('tickets-container');
    const storedTickets = sessionStorage.getItem('tickets');

=======
// Wird ausgeführt, wenn das Dokument vollständig geladen ist
document.addEventListener('DOMContentLoaded', () => {
    // Holt sich die Tickets aus dem sessionStorage des Browsers
    const ticketsContainer = document.getElementById('tickets-container');
    const storedTickets = sessionStorage.getItem('tickets');

    // Falls Tickets vorhanden sind, wird dieser Code aufgerufen
>>>>>>> 1a68b1f3a3def8f8d440aa150609c392b4d668ac
    if (storedTickets) {
        const tickets = JSON.parse(storedTickets);
        tickets.forEach(ticket => {
            const ticketElement = document.createElement('div');
            ticketElement.classList.add('ticket');
<<<<<<< HEAD
            ticketElement.innerHTML = `
                <h3>${ticket.ticket_title}</h3>
=======

            // Fügt die Ticket-Informationen in das HTML ein
            ticketElement.innerHTML = `
            <h3>${ticket.ticket_title}</h3>
>>>>>>> 1a68b1f3a3def8f8d440aa150609c392b4d668ac
                <p><strong>Name:</strong> ${ticket.name}</p>
                <p><strong>Email:</strong> ${ticket.email}</p>
                <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                <p><strong>Raum:</strong> ${ticket.raum}</p>
<<<<<<< HEAD
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
=======
            `;
            // Fügt das Ticket-Element in das ticketsContainer ein
            ticketsContainer.appendChild(ticketElement);
        });

    } else {
        // Falls keine Tickets vorhanden sind, wird dieser Code aufgerufen
        ticketsContainer.innerHTML = '<p>Keine Tickets vorhanden</p>'
>>>>>>> 1a68b1f3a3def8f8d440aa150609c392b4d668ac
    }
});