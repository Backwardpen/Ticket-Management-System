const loginStatusDiv = document.getElementById('loginStatus');

// Funktion zum Erstellen eines Tickets
// Diese Funktion wird aufgerufen, wenn der Benutzer auf den Button klickt und es wird eine neue Seite geöffnet
function createTicket() {
    window.location.href = "createTicket.html";
}

document.addEventListener('DOMContentLoaded', () => {
    const loginForm = document.getElementById('loginForm');

    loginForm.addEventListener('submit', async function (event) {
        event.preventDefault();

        const emailInput = document.getElementById('loginEmail');
        const passwordInput = document.getElementById('loginPassword');
        const email = emailInput.value;
        const password = passwordInput.value;

        loginStatusDiv.style.display = "none";
        loginStatusDiv.textContent = "";

        const loginData = {
            email: email,
            password: password,
        };

        // console.log('Login Data:', loginData); // Debugging

        try {
            const response = await fetch(loginForm.action, {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(loginData)
            });

            console.log('Response:', response);

            if (response.ok) {
                try {
                    const token = await response.json();
                    console.log('Response JSON', token);
                    let userEmail;

                    // Hier wird der Token decodiert und die Email extrahiert
                    // Wenn der Token ein JWT ist, wird die Email aus dem Payload extrahiert
                    // Hier passiert große Magie
                    /**
                     * Decodes a base64-encoded string into a JSON payload.
                     * The function first decodes the base64 string, then decodes any percent-encoded UTF-8 characters,
                     * resulting in a readable JSON string.
                     *
                     * @param {string} base64 - The base64-encoded string to decode.
                     * @returns {string} The decoded JSON payload as a string.
                    */
                    if (typeof token === 'string') {
                        try {
                            const base64Url = token.split('.')[1];
                            const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
                            const jsonPayload = decodeURIComponent(atob(base64).split('').map(function (c) {
                                return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
                            }).join(''));

                            const decodedPayload = JSON.parse(jsonPayload);
                            
                            userEmail = decodedPayload.sub || token; // Nutze sub für Email oder fallback auf Token wenn sub nicht vorhanden ist 
                        } catch (e) {
                            console.error("Token konnte nicht korrekt oder gar nicht decodiert werden: ", e);
                            userEmail = token
                        }
                    } else if (token && token.email) {
                        userEmail = token.email;
                    }
                    else {
                        userEmail = 'default';
                    }

                    localStorage.setItem('userEmail', userEmail);
                    window.location.href = "dashboard.html";


                } catch (jsonError) {
                    console.error("Fehler beim Lesen der JSON Daten nach login : ", jsonError);
                    loginStatusDiv.textContent = 'Fehler bei Login-Auswertung:\n' + (jsonError.message || 'Unbekannter Fehler');
                    
                    loginStatusDiv.className = 'form-status error';
                    loginStatusDiv.style.display = "block";
                }
            } else {
                try {
                    const errorText = await response.text();
                    console.error("Fehler beim Anmelden: ", errorText);
                    loginStatusDiv.textContent = 'Fehler beim Anmelden:\n' + errorText;

                    loginStatusDiv.className = 'form-status error';
                    loginStatusDiv.style.display = "block";
                } catch (error) {
                    console.error("Fehler beim Auslesen der Daten mit der Antwort:", error);
                    loginStatusDiv.textContent = 'Fehler bei der Antwort\n';

                    loginStatusDiv.className = 'form-status error';
                    loginStatusDiv.style.display = "block";
                }
            }
        } catch (error) {
            console.error("Fehler beim Anmelden mit dem Fehler: ", error)
            loginStatusDiv.textContent = 'Ein Fehler ist beim Login aufgetreten:\n' + (error.message || 'Unbekannter Fehler');
            loginStatusDiv.className = 'form-status error';
            loginStatusDiv.style.display = "block";
        }
    });
});