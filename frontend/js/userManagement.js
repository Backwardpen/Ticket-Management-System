const loginStatusDiv = document.getElementById('loginStatus');
const formStatusDiv = document.getElementById('formStatus');
document.addEventListener('DOMContentLoaded', () => {

    const createUserForm = document.getElementById('createUserForm');

    // Event Listener für Register Button
    createUserForm.addEventListener('submit', async function (event) {
        event.preventDefault();

        const emailInput = document.getElementById('loginEmail');
        const passwordInput = document.getElementById('loginPassword');
        const usernameInput = document.getElementById('username');

        const email = emailInput.value;
        const password = passwordInput.value;
        const username = usernameInput.value;

        loginStatusDiv.style.display = "none";
        loginStatusDiv.textContent = "";

        const registerData = {
            email: email,
            username: username,
            password: password,
        }

        try {
            const response = await fetch("http://127.0.0.1:5555/register", {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(registerData)
            });

            if (response.ok) {
                const result = await response.json();

                sessionStorage.setItem('users', JSON.stringify(result));

                loginStatusDiv.textContent = 'Registrierung war erfolgreich!\n'
                loginStatusDiv.className = 'form-status success';
                loginStatusDiv.style.display = "block";

                createUserForm.reset();
            } else {
                const result = await response.json();

                console.error("Fehler bei der Registrierung: ", result)

                loginStatusDiv.textContent = 'Fehler bei der Registrierung:\n' + (result.message || 'Unbekannter Fehler');
                loginStatusDiv.className = 'form-status error';
                loginStatusDiv.style.display = "block";
            }
        } catch (error) {
            console.error("Fehler: ", error)

            loginStatusDiv.textContent = 'Ein Fehler beim Registrieren ist aufgetreten: \n' + (error.message || 'Unbekannter Fehler');
            loginStatusDiv.className = 'form-status error';
            loginStatusDiv.style.display = "block";
        }
    });
    benutzerDatenAnzeige();
});

function benutzerDatenAnzeige() {
    // Holt sich die Tickets aus dem sessionStorage des Browsers
    const userContainer = document.getElementById('userContainer');
    const storedUsers = sessionStorage.getItem('users');
    // Falls Tickets vorhanden sind, wird dieser Code aufgerufen
    if (storedUsers) {
        let users = JSON.parse(storedUsers);

        // Bisher angezeigte Daten Löschen
        userContainer.innerHTML = "";

        console.log(storedUsers);

        if (Array.isArray(users)) {
            users.forEach(user => {
                const userElement = document.createElement('div');
                userElement.classList.add('user');
                // Fügt die User-Informationen in das HTML ein pro User
                userElement.innerHTML = `
                <h3>${user.username}</h3>
                <p><strong>Email:</strong> ${user.email}</p>
                <p><strong>ID:</strong> ${user.id}</p>
            `;
                // Fügt das User-Element in das userContainer ein
                userContainer.appendChild(userElement);
            });
        } else {
            // Wenn es kein Array ist, gehe davon aus, dass es ein einzelnes User-Objekt ist und erstelle direkt die div.
            const userElement = document.createElement('div');
            userElement.classList.add('user');
            userElement.innerHTML = `
                <h3>${users.username}</h3>
                <p><strong>Email:</strong> ${users.email}</p>
                <p><strong>ID:</strong> ${users.id}</p>
              `;
            userContainer.appendChild(userElement);
        }
    } else {
        // Falls keine User vorhanden sind, wird dieser Code aufgerufen --> sollte nie vorkommen, da es immer einen Standard-User (Id: 1) geben muss
        userContainer.innerHTML = '<p>Es sind keine User vorhanden</p>';
    }
}

document.addEventListener('DOMContentLoaded', benutzerDatenAnzeige);