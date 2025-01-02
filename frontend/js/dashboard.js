document.addEventListener('DOMContentLoaded', () => { // Methode wurde mittels KI generiert/verbessert, da mein Code nicht vollständig funktioniert hat bzw nicht so wie ich es wollte

    const userToken = localStorage.getItem('userEmail');   // Geholte text vom Localstore (z.b. hier = JWT string)

    // helper Funktion zur Base64 Dekodierung , ich suche das Value der EmailAdresse)
    function base64Decode(str) {    // JavaScript interne Methode: UTF-8 String zum auslesen der variablen `Subject`, da dieser ein encoded string als Payload  ist --> Payload ist der 2. Teil des JWT Tokens, der die eigentlichen Daten enthält.
        try {
            return decodeURIComponent(atob(str).split('').map(function (c) {    // "decodeURIComponent" zur de-codierung des JSON-Strings --> atob => ascii to  base 64
                return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);   // Das Array.Map liefert alle ASCII Zeichen
            }).join(''));
        }
        catch (e) {
            return "noPayloadGiven";
        }
    }
    let emailToShow = "Gast";  // Standard output text --> sollte nie angezeigt werden

    if (userToken) {
        try {
            let payload = base64Decode(userToken.split('.')[1])   // Splitte den Text des string auf der Basis des "." 
            const emailFromToken = JSON.parse(payload);  // Hier werden Payload  mit  dem JS-Code von json.parse geparsed. (von string --> JS Objekt)
            if (emailFromToken && emailFromToken.sub) {
                emailToShow = emailFromToken.sub;
            }
        } catch (error) {
            console.log(error, 'JSON Parse Error!');
            loginStatusDiv.textContent = ` Ein Decodier-Fehler ist aufgetreten!\n   Bitte melden Sie dies der Technik-AG: \n` + (error.message || 'unbekannter Fehler');
            loginStatusDiv.style.display = "block";
        }
    }
    const greetingElement = document.getElementById('greeting');   // <h1> Tag Referenz! --> Hier wird der Text in h1 eingefügt
    greetingElement.textContent = `Willkommen, ${emailToShow}!`;
});

function toggleDropdown() {
    document.getElementById("dropdown-content").classList.toggle("show");
}

// Schliesst das Dropdown Menü, wenn der User auf ein Element ausserhalb des Dropdown Menüs klickt
window.onclick = function (event) {
    if (!event.target.matches('.dropbtn')) {
        var dropdowns = document.getElementsByClassName("dropdown-content");
        var i;
        for (i = 0; i < dropdowns.length; i++) {
            var openDropdown = dropdowns[i];
            if (openDropdown.classList.contains('show')) {
                openDropdown.classList.remove('show');
            }
        }
    }
}