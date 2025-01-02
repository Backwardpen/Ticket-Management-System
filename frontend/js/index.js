const loginStatusDiv = document.getElementById('loginStatus');

        function createTicket() {
            window.location.href = "createTicket.html";
        }

        // Event Listener für Register Button
        document.getElementById('registerButton').addEventListener('click', async function () {
            const emailInput = document.getElementById('loginEmail');
            const passwordInput = document.getElementById('loginPassword');
            const email = emailInput.value;
            const password = passwordInput.value;

            loginStatusDiv.style.display = "none";
            loginStatusDiv.textContent = "";

            const registerData = {
                email: email,
                password: password
            }

            try {
                const response =  await fetch("http://127.0.0.1:5555/register", {
                    method: "POST",
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(registerData)
                });
                if (response.ok) {
                    const result =  await response.json();
                    console.log('Response Json:', result);
                    loginStatusDiv.textContent = 'Registrierung war erfolgreich!\n'
                    loginStatusDiv.style.display = "block";
                } else {
                    const result = await response.json();
                    console.error("Fehler bei der Registrierung: ", result)
                    loginStatusDiv.textContent = 'Fehler bei der Registrierung:\n' + (result.message || 'Unbekannter Fehler');
                    loginStatusDiv.style.display = "block";
                }
            } catch(error) {
                console.error("Fehler: ", error)
                loginStatusDiv.textContent = 'Ein Fehler beim Registrieren ist aufgetreten: \n' + (error.message || 'Unbekannter Fehler');
                loginStatusDiv.style.display = "block";
            }
        });

        document.getElementById('loginForm').addEventListener('submit', async function(event) {
            event.preventDefault();

            const emailInput = document.getElementById('loginEmail');
            const passwordInput = document.getElementById('loginPassword');
            const  loginStatusDiv = document.getElementById('loginStatus');
            const email = emailInput.value;
            const password = passwordInput.value;
            
            loginStatusDiv.style.display = "none";
            loginStatusDiv.textContent = "";
            
            const   loginData = {
                email: email,
                password: password
            }
    
            try {
                const response =  await fetch("http://127.0.0.1:5555/login", {
                    method: "POST",
                    headers: {
                       'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(loginData)
                });

                if (response.ok) {

                    try {
                        const token =  await response.json();
                        console.log('Response JSON', token);
              
                        if(typeof(token) === 'string' ) { // JWT Decodierung wenn er ein Token  ist (falls die backend api anders liefert, dass hier die anpassungen gemacht werden können)
                            const base64Url = token.split('.')[1]; // header.payload.signature
                            const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
                            const jsonPayload = decodeURIComponent(atob(base64).split('').map(function(c) {
                                return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
                            }).join('')); // Decodierung mit utf-8 struktur (atob gibt sonst error bei speziellen symbole. Der urlencoded format ist auch in den string kodiert, was in einer anderen encoding zu "not a valid structure" error liefern könnte, was das decode fehlschlagen lassen könnte). Da diese Format eine Url formatiert ist werden spezielle character noch mals encoded in URL, das durch decodieren (uri) wieder korrigiert wird, um wieder einen korrekt formatierten String zu geben)
                            let decodedPayload;

                            try{
                                decodedPayload = JSON.parse(jsonPayload)
                                console.log("decoded Token", decodedPayload)
                            } catch (e) {
                                console.error('Could not decode, its not a correct token JSON. Output will be: ' ,jsonPayload )
                                localStorage.setItem('userEmail',token) // Token ohne struktur als output 
                            }
                            localStorage.setItem('userEmail', decodedPayload.email  || token)
                        } else {
                         localStorage.setItem('userEmail',token.email)   // normale json dekodierung wie er eigentlich ankommen soll
                        }
                        window.location.href = "dashboard.html";
                    } catch (jsonError) {
                        console.error("Fehler beim Lesen der JSON Daten nach login : ", jsonError);
                        loginStatusDiv.textContent = 'Fehler bei Login-Auswertung:\n'+ (jsonError.message || 'Unbekannter Fehler');
                        loginStatusDiv.style.display = "block";
                    }
                } else {   // Wenn der response code NOT OK. (also fehlerhafter login etc. wird versucht)
                    try {
                        const result = await response.json();
                        console.error("Fehler beim Anmelden: ", result);
                        loginStatusDiv.textContent = 'Fehler beim Anmelden:\n' + (result.message || 'Unbekannter Fehler');
                        loginStatusDiv.style.display = "block";
                    } catch (error) { // kein valid json
                        console.error("Fehler beim Auslesen der Daten vom Response", error);
                        loginStatusDiv.textContent = 'Fehler bei der Response\n';
                        loginStatusDiv.style.display = "block";
                    }
                }
            } catch (error) {
                console.error("Fehler beim Anmelden: ", error)
                loginStatusDiv.textContent = 'Ein Fehler beim Login ist aufgetreten:\n' + (error.message || 'Unbekannter Fehler');
                loginStatusDiv.style.display = "block";
            }
        });