document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('createTicketForm');
    const formStatusDiv = document.getElementById('formStatus');
    const roomInput = document.getElementById('raum');
    const roomErrorDiv = document.getElementById('roomError');
    const roomOptions = Array.from(document.querySelectorAll('#rooms option')).map(option => option.value);

    const viewTicketForm = document.getElementById("viewTicketForm");
    const formStatusViewDiv = document.getElementById('formStatusView');

    // -------- Ticket-Erstellung --------
    form.addEventListener('submit', async (event) => {
        event.preventDefault();

        // Hier wird überprüft, ob der Raum korrekt ausgewählt wurde oder ob überhaupt ein Raum ausgewählt wurde
        const selectedRoom = roomInput.value.trim();
        if (selectedRoom === "" || !roomOptions.includes(selectedRoom)) {
            roomErrorDiv.style.display = "block";
            roomInput.focus();
            return;
        } else {
            roomErrorDiv.style.display = "none";
        }

        formStatusDiv.style.display = "none";
        formStatusDiv.textContent = "";

        // Hier wird das Formular in ein JSON-Objekt umgewandelt
        // Das mit JSON ist notwendig, weil wir die Daten als JSON an den Server senden und es eben sonst nicht funktioniert
        const formData = new FormData(form);
        const formDataJson = {};
        formData.forEach((value, key) => {
            formDataJson[key] = value;
        });

        try {
            // Erwartet eine Antwort im JSON-Format vom Server
            const response = await fetch(form.action, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formDataJson),
            });

            if (response.ok) {
                const result = await response.json();
                formStatusDiv.textContent = 'Ticket erstellt!';
                formStatusDiv.className = 'form-status success';
                formStatusDiv.style.display = "block";
                form.reset();
            } else {
                const result = await response.json();
                formStatusDiv.textContent = 'Fehler beim Erstellen des Tickets!\n' + (result.message || 'Unbekannter Fehler');
                formStatusDiv.className = 'form-status error';
                formStatusDiv.style.display = "block";
            }
        } catch (error) {
            console.error('Fehler beim Senden des Tickets:', error);
            formStatusDiv.textContent = 'Fehler beim Senden des Tickets!\n' + (error.message || 'Unbekannter Fehler');
            formStatusDiv.className = 'form-status error';
            formStatusDiv.style.display = "block";
        }
    });

    roomInput.addEventListener('input', () => {
        roomErrorDiv.style.display = 'none';
    });

    // -------- Ticket-Anzeige (per E-Mail) --------
    viewTicketForm.addEventListener('submit', (event) => {
        event.preventDefault();
        const emailInput = document.getElementById('emailView');
        const email = emailInput.value;

        // Wenn die E-Mail-Adresse nicht angegeben wurde, wird eine Fehlermeldung angezeigt
        if (!email) {
            formStatusViewDiv.textContent = 'Bitte geben Sie eine gültige Email-Adresse ein';
            formStatusViewDiv.style.display = "block";
            formStatusViewDiv.className = 'form-status error';
            return;
        }

        // Weiterleiten mit Email als URL-Parameter auf die viewTicket.html
        const encodedEmail = encodeURIComponent(email);
        window.location.href = `viewTicket.html?email=${encodedEmail}`;
    });
});
