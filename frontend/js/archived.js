// Work in progress


document.addEventListener('DOMContentLoaded', () => {
    const container = document.getElementById('archived-tickets-container');

    fetch('http://10.10.10.194:5555/tickets/archived')
        .then(response => {
            if (!response.ok) {
                throw new Error('Fehler beim Abrufen der archivierten Tickets');
            }
            return response.json();
        })
        .then(tickets => {
            if (tickets.length === 0) {
                container.innerHTML = '<p>Keine archivierten Tickets vorhanden.</p>';
                return;
            }

            tickets.forEach(ticket => {
                const ticketDiv = document.createElement('div');
                ticketDiv.className = 'ticket';
                ticketDiv.innerHTML = `
                    <h3>${ticket.ticket_title}</h3>
                    <p><strong>ID:</strong> ${ticket.id}</p>
                    <p><strong>Name:</strong> ${ticket.name}</p>
                    <p><strong>Email:</strong> ${ticket.email}</p>
                    <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                    <p><strong>Raum:</strong> ${ticket.raum}</p>
                `;
                container.appendChild(ticketDiv);
            });
        })
        .catch(error => {
            console.error('Fehler:', error);
            container.innerHTML = '<p>Fehler beim Laden der archivierten Tickets.</p>';
        });
});
