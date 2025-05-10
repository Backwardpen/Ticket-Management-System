const loginStatusDiv = document.getElementById('loginStatus');
const formStatusDiv = document.getElementById('formStatus');
document.addEventListener('DOMContentLoaded', () => {

    const createUserForm = document.getElementById('createUserForm');

    // Event Listener für Register Button
    createUserForm.addEventListener('submit', async function (event) {
        event.preventDefault();

        const emailInput = document.getElementById('loginEmail');
        const passwordInput = document.getElementById('loginPassword');

        const email = emailInput.value;
        const password = passwordInput.value;

        loginStatusDiv.style.display = "none";
        loginStatusDiv.textContent = "";

        const registerData = {
            email: email,
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

                let existingUsers = sessionStorage.getItem('users');
                if (existingUsers) {
                    let parsedUsers = JSON.parse(existingUsers);
                    if (!Array.isArray(parsedUsers)) {
                        parsedUsers = [parsedUsers];
                    }

                    if (Array.isArray(result)) {
                        sessionStorage.setItem('users', JSON.stringify([...parsedUsers, ...result]))
                    } else {
                        sessionStorage.setItem('users', JSON.stringify([...parsedUsers, result]))
                    }
                }
                else {
                    sessionStorage.setItem('users', JSON.stringify(result));
                }

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
});