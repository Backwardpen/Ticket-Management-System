document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('createTicketForm');
    const formStatusDiv = document.getElementById('formStatus');
    const roomInput = document.getElementById('raum');
    const roomErrorDiv = document.getElementById('roomError');
    const roomOptions = Array.from(document.querySelectorAll('#rooms option')).map(option => option.value);

    const viewTicketForm = document.getElementById("viewTicketForm");
    const formStatusViewDiv = document.getElementById('formStatusView');

    // CreateTicket Form soll die Daten an den Server senden und dann eine Erfolgsmeldung oder Fehlermeldung anzeigen
    form.addEventListener('submit', async (event) => {
        event.preventDefault();

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

        const formData = new FormData(form);
        const formDataJson = {};
        formData.forEach((value, key) => {
            formDataJson[key] = value;
        });
        try {
            const response = await fetch(form.action, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formDataJson),
            });

            console.log('Fetch Response', response);

            if (response.ok) {
                const result = await response.json();
                console.log('Response JSON', result);
                formStatusDiv.textContent = 'Ticket erstellt!\n';
                formStatusDiv.className = 'form-status success';
                formStatusDiv.style.display = "block";
                form.reset();
            } else {
                const result = await response.json();
                formStatusDiv.textContent = 'Fehler beim Erstellen des Tickets!\n' + (result.message || 'Unbekannter Fehler');
                console.log('Fehler beim Fetch (NOT OK)', result)
                formStatusDiv.className = 'form-status error';
                formStatusDiv.style.display = "block";
            }
        } catch (error) {
            console.error('Fehler beim Senden des Tickets (ERROR CATCH):', error);
            formStatusDiv.textContent = 'Fehler beim Senden des Tickets!\n' + (error.message || 'Unbekannter Fehler');
            formStatusDiv.className = 'form-status error';
            formStatusDiv.style.display = "block";
        }
    });
    
    roomInput.addEventListener('input', () => {
        roomErrorDiv.style.display = 'none';
    });

    // ViewTicket Form soll die Email abrufen und dann weiterleiten auf die viewTicket.html Seite mit der eingetragenen Email
    viewTicketForm.addEventListener('submit', (event) => {
        event.preventDefault();
        function viewTickets() {
            const emailInput = document.getElementById('emailView');
            const email = emailInput.value;
            if (email) {
                fetch(`http://127.0.0.1:5555/tickets/by_email/${email}`)
                    .then(response => response.json())
                    .then(data => {
                        if (data && data.length > 0) { // Hier wird geprüft ob Daten vorhanden sind
                            sessionStorage.setItem('tickets', JSON.stringify(data));
                            window.location.href = `viewTicket.html`;
                        } else {
                            formStatusViewDiv.textContent = 'Keine Tickets für diese E-Mail-Adresse gefunden.';
                            formStatusViewDiv.style.display = "block";
                            formStatusViewDiv.className = 'form-status error';
                        }

                    }).catch(error => {
                        console.error('There has been a problem with your fetch operation:', error);
                        formStatusViewDiv.textContent = 'Fehler beim abrufen der Daten!\n' + (error.message || 'Unbekannter Fehler');
                        formStatusViewDiv.style.display = "block";
                        formStatusViewDiv.className = 'form-status error';
                    });
            } else {
                formStatusViewDiv.textContent = 'Bitte geben Sie eine gültige Email-Adresse ein';
                formStatusViewDiv.style.display = "block";
                formStatusViewDiv.className = 'form-status error';
            }
        }
        viewTickets();
    });
});