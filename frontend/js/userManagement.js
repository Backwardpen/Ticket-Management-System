const loginStatusDiv = document.getElementById('loginStatus');

document.addEventListener('DOMContentLoaded', () => {
    const createUserForm = document.getElementById('createUserForm');

    // Benutzerliste beim Laden der Seite direkt abrufen
    benutzerDatenAnzeige();

    // Event Listener für das Formular
    createUserForm.addEventListener('submit', async function (event) {
        event.preventDefault();

        const email = document.getElementById('loginEmail').value;
        const password = document.getElementById('loginPassword').value;

        loginStatusDiv.style.display = "none";
        loginStatusDiv.textContent = "";

        const registerData = {
            email: email,
            password: password,
        };

        try {
            /**
             * Das Response-Objekt, das von der fetch-API beim Aufruf des User-Registrierungsendpunkts zurückgegeben wird.
             * 
             * @type {Response}
             * @siehe {@link https://developer.mozilla.org/de/docs/Web/API/Response}
             */
            const response = await fetch("http://10.10.10.194:5555/register", {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(registerData)
            });

            if (response.ok) {
                loginStatusDiv.textContent = 'Registrierung war erfolgreich!';
                loginStatusDiv.className = 'form-status success';
                loginStatusDiv.style.display = "block";
                createUserForm.reset();

                // Benutzerliste nach erfolgreicher Registrierung neu laden
                benutzerDatenAnzeige();
            } else {
                const result = await response.json();
                console.error("Fehler bei der Registrierung: ", result);
                loginStatusDiv.textContent = 'Fehler bei der Registrierung:\n' + (result.message || 'Unbekannter Fehler');
                loginStatusDiv.className = 'form-status error';
                loginStatusDiv.style.display = "block";
            }
        } catch (error) {
            console.error("Fehler: ", error);
            loginStatusDiv.textContent = 'Ein Fehler beim Registrieren ist aufgetreten:\n' + (error.message || 'Unbekannter Fehler');
            loginStatusDiv.className = 'form-status error';
            loginStatusDiv.style.display = "block";
        }
    });
});

/**
 * Lädt die Benutzerliste vom Server und zeigt sie im HTML-Element mit der ID 'userContainer' an.
 * 
 * Für jeden Benutzer wird eine E-Mail-Adresse sowie ein Lösch-Button angezeigt.
 * Bei Klick auf den Lösch-Button wird die Funktion `deleteUser` mit der E-Mail des Benutzers aufgerufen.
 * 
 * Fehler beim Laden oder Abrufen der Benutzerliste werden im Container angezeigt und in der Konsole protokolliert.
 * 
 * @async
 * @function benutzerDatenAnzeige
 * @returns {Promise<void>} Gibt ein Promise zurück, das abgeschlossen ist, wenn die Benutzer angezeigt wurden.
 */
async function benutzerDatenAnzeige() {
    const userContainer = document.getElementById('userContainer');
    userContainer.innerHTML = '';

    try {
        const response = await fetch("http://10.10.10.194:5555/users");
        if (response.ok) {
            const users = await response.json();

            if (users.length === 0) {
                userContainer.innerHTML = '<p>Keine Benutzer gefunden.</p>';
                return;
            }

            users.forEach(user => {
                const userDiv = document.createElement('div');
                userDiv.classList.add('user-entry');

                // Email
                const userText = document.createElement('span');
                userText.textContent = `Email: ${user.email}`;
                userDiv.appendChild(userText);

                // Delete Button
                const deleteBtn = document.createElement('button');
                deleteBtn.textContent = 'Löschen';
                deleteBtn.classList.add('delete-user-button');
                deleteBtn.onclick = () => deleteUser(user.email);

                userDiv.appendChild(deleteBtn);
                userContainer.appendChild(userDiv);
            });
        } else {
            userContainer.innerHTML = '<p>Fehler beim Laden der Benutzerliste.</p>';
        }
    } catch (error) {
        console.error("Fehler beim Abrufen der Benutzerliste:", error);
        userContainer.innerHTML = '<p>Fehler beim Abrufen der Benutzerliste.</p>';
    }
}

/**
 * Löscht einen Benutzer anhand seiner E-Mail-Adresse nach Bestätigung durch den Nutzer.
 *
 * Zeigt eine Bestätigungsabfrage an und sendet anschließend eine DELETE-Anfrage an den Server,
 * um den Benutzer zu löschen. Bei Erfolg wird eine Benachrichtigung angezeigt und die Benutzerliste aktualisiert.
 * Bei Fehlern werden entsprechende Fehlermeldungen angezeigt.
 *
 * @async
 * @function
 * @param {string} email - Die E-Mail-Adresse des zu löschenden Benutzers.
 * @returns {Promise<void>} 
 */
async function deleteUser(email) {
    const confirmed = confirm(`Möchtest du den Benutzer "${email}" wirklich löschen?`);
    if (!confirmed) return;

    try {
        const response = await fetch(`http://10.10.10.194:5555/users/delete/${encodeURIComponent(email)}`, {
            method: 'DELETE',
        });

        if (response.ok) {
            alert("Benutzer wurde erfolgreich gelöscht.");
            benutzerDatenAnzeige();
        } else {
            const result = await response.json();
            alert("Fehler beim Löschen:\n" + (result.message || 'Unbekannter Fehler'));
        }
    } catch (err) {
        console.error("Fehler beim Löschen des Benutzers:", err);
        alert("Netzwerkfehler beim Löschen des Benutzers.");
    }
}