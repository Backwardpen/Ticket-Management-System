document.addEventListener('DOMContentLoaded', async () => {
    const urlParams = new URLSearchParams(window.location.search);
    const ticketId = urlParams.get('id');

    const titleInput = document.getElementById('ticketTitle');
    const nameInput = document.getElementById('name');
    const emailInput = document.getElementById('email');
    const descriptionInput = document.getElementById('ticketDescription');
    const roomInput = document.getElementById('raum');
    const formStatusDiv = document.getElementById('formStatus');

    // Wenn Ticket-ID fehlt oder ungültig ist, dann Fehler anzeigen
    if (!ticketId || isNaN(ticketId)) {
        console.error('Ungültige oder fehlende Ticket-ID');
        formStatusDiv.textContent = 'Ungültige oder fehlende Ticket-ID.';
        formStatusDiv.classList.add('form-status', 'error');
        formStatusDiv.style.display = 'block';
        return;
    }

    try {
        const response = await fetch(`http://10.10.10.194:5555/tickets/${ticketId}`);
        if (!response.ok) throw new Error(`Fehler beim Abrufen: Status ${response.status}`);

        const data = await response.json();

        // Felder befüllen
        titleInput.value = data.ticket_title || '';
        nameInput.value = data.name || '';
        emailInput.value = data.email || '';
        descriptionInput.value = data.ticket_description || '';
        roomInput.value = data.raum || '';

    } catch (error) {
        console.error('Fehler beim Laden der Ticketdaten:', error);
        formStatusDiv.textContent = 'Fehler beim Laden der Ticketdaten.\n' + (error.message || 'Unbekannter Fehler');
        formStatusDiv.classList.add('form-status', 'error');
        formStatusDiv.style.display = 'block';
    }
});