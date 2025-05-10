document.addEventListener('DOMContentLoaded', () => {
    const ticketsContainer = document.getElementById('tickets-container');
    const params = new URLSearchParams(window.location.search);
    const email = params.get('email');

    // Wenn keine E-Mail-Adresse vorhanden ist, zeige eine Fehlermeldung an
    if (!email) {
        ticketsContainer.innerHTML = '<p>Keine E-Mail-Adresse übergeben.</p>';
        return;
    }

    // Fetch-Tickets von der API
    fetch(`http://10.10.10.194:5555/tickets/by_email/${encodeURIComponent(email)}`)
        .then(response => {
            if (!response.ok) {
                throw new Error("Fehler beim Abrufen der Tickets");
            }
            return response.json();
        })
        .then(tickets => {
            if (!tickets || tickets.length === 0) {
                ticketsContainer.innerHTML = '<p>Es sind keine Tickets für diese Email vorhanden</p>';
                return;
            }

            // Tickets anzeigen (für jedes Ticket ein neues Element erstellen)
            tickets.forEach(ticket => {
                const ticketElement = document.createElement('div');
                ticketElement.classList.add('ticket');
                ticketElement.setAttribute('data-id', ticket.id);

                // Ticket-Details
                // Nicht hardcodiert, sondern aus dem Ticket-Objekt, damit es leichter ist
                ticketElement.innerHTML = `
                    <h3>${ticket.ticket_title}</h3>
                    <p><strong>Ticket-ID:</strong> ${ticket.id}</p>
                    <p><strong>Name:</strong> ${ticket.name}</p>
                    <p><strong>Email:</strong> ${ticket.email}</p>
                    <p><strong>Beschreibung:</strong> ${ticket.ticket_description}</p>
                    <p><strong>Raum:</strong> ${ticket.raum}</p>
                    <div class="ticket-actions">
                        <button class="edit-button" data-id="${ticket.id}">Bearbeiten</button>
                        <button class="close-button" data-id="${ticket.id}">WIP (Schließen)</button>
                    </div>
                `;

                ticketsContainer.appendChild(ticketElement);
            });

            // Bearbeiten-Button
            document.querySelectorAll('.edit-button').forEach(button => {
                button.addEventListener('click', () => {
                    const ticketId = button.getAttribute('data-id');
                    window.location.href = `editTicket.html?id=${ticketId}`;
                });
            });

            // Schließen-Button
            document.querySelectorAll('.close-button').forEach(button => {
                button.addEventListener('click', async () => {
                    const ticketId = button.getAttribute('data-id');

                    // Bestätigungsdialog
                    const confirmClose = confirm("Möchtest du dieses Ticket wirklich schließen?");
                    if (!confirmClose) return;

                    // Ticket schließen, falls der Benutzer bestätigt
                    try {
                        const response = await fetch(`http://10.10.10.194:5555/tickets/close/${ticketId}`, {
                            method: 'POST',
                        });

                        if (response.ok) {
                            const ticketDiv = document.querySelector(`.ticket[data-id="${ticketId}"]`);
                            if (ticketDiv) ticketDiv.remove();
                                // Prüfen, ob noch Tickets übrig sind
                                const remainingTickets = document.querySelectorAll('.ticket');

                                //console.log("Ticket schließen:", ticketId); // Debugging-Info

                                if (remainingTickets.length === 0) {
                                    ticketsContainer.innerHTML = '<p>Es sind keine Tickets für diese Email vorhanden</p>';
                                }

                                alert("Ticket wurde erfolgreich geschlossen.");
                        } else {
                            const err = await response.json();
                            alert("Fehler beim Schließen des Tickets:\n" + (err.message || "Unbekannter Fehler"));
                        }
                    } catch (err) {
                        console.error("Netzwerkfehler:", err);
                        alert("Netzwerkfehler beim Schließen des Tickets.");
                    }
                });
            });
        })
        .catch(error => {
            console.error("Fehler beim Abrufen der Tickets:", error);
            ticketsContainer.innerHTML = '<p>Fehler beim Laden der Tickets.</p>';
        });
});