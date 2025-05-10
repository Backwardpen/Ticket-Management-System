document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('createTicketForm');
    const formStatusDiv = document.getElementById('formStatus');

    form.addEventListener('submit', async (event) => {
        event.preventDefault();
        formStatusDiv.style.display = "none";

        const formData = new FormData(form);
        const data = Object.fromEntries(formData.entries());

        try {
            const response = await fetch('http://10.10.10.194:5555/tickets', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(data)
            });

            const result = await response.json();

            if (response.ok) {
                formStatusDiv.textContent = 'Ticket erfolgreich erstellt!';
                formStatusDiv.className = 'form-status success';
                formStatusDiv.style.display = "block";
                form.reset();
            } else {
                formStatusDiv.textContent = result.message || 'Fehler beim Erstellen des Tickets';
                formStatusDiv.className = 'form-status error';
                formStatusDiv.style.display = "block";
            }
        } catch (error) {
            console.error('Netzwerkfehler:', error);
            formStatusDiv.textContent = 'Netzwerkfehler: ' + error.message;
            formStatusDiv.className = 'form-status error';
            formStatusDiv.style.display = "block";
        }
    });
});
