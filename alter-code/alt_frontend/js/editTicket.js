document.addEventListener('DOMContentLoaded', () => {
    // URL Parameter auslesen
    const urlParams = new URLSearchParams(window.location.search);
    const ticketId = urlParams.get('id');

    const titleInput = document.getElementById('ticketTitle');
    const nameInput = document.getElementById('name');
    const emailInput = document.getElementById('email');
    const descriptionInput = document.getElementById('ticketDescription');
    const roomInput = document.getElementById('raum');
    const formStatusDiv = document.getElementById('formStatus');
    if (ticketId) {
        fetch(`http://127.0.0.1:5555/tickets/${ticketId}`)
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                // Befüllen der Eingabefelder
                titleInput.value = data.ticket_title;
                nameInput.value = data.name;
                emailInput.value = data.email;
                descriptionInput.value = data.ticket_description;
                roomInput.value = data.raum;
            })
            .catch(error => {
                console.error('Fehler beim Laden der Ticket Daten: ', error);
                formStatusDiv.textContent = 'Fehler beim Laden der Ticket Daten!\n' + (error.message || 'Unbekannter Fehler');
                formStatusDiv.style.display = "block";
            });
    } else {
        formStatusDiv.textContent = 'Keine Ticket ID im URL gefunden!\n';
        formStatusDiv.style.display = "block";
        console.error('Keine Ticket ID im URL gefunden');
    }
});