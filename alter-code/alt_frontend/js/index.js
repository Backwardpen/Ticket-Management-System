const loginStatusDiv = document.getElementById('loginStatus');

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
            password: password
        };

        console.log('Login Data:', loginData);

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

                    if (typeof token === 'string') {
                        try {
                            const base64Url = token.split('.')[1];
                            const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
                            const jsonPayload = decodeURIComponent(atob(base64).split('').map(function (c) {
                                return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
                            }).join(''));

                            const decodedPayload = JSON.parse(jsonPayload);
                            
                            userEmail = decodedPayload.sub || token; // Nutze sub für email oder fallback auf token wenn sub nicht vorhanden ist 
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
                    const result = await response.json();
                    console.error("Fehler beim Anmelden: ", result);
                    loginStatusDiv.textContent = 'Fehler beim Anmelden:\n' + (result.message || 'Unbekannter Fehler');
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