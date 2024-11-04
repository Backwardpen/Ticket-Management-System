document.addEventListener('DOMContentLoaded', function() {
    function viewTickets() {
        window.location.href = "viewTicket.html";
    }

    document.getElementById("createTicketBtn").addEventListener("click", function() {
        const title = document.getElementById("ticketTitle").value;
        const name = document.getElementById("name").value;
        const email = document.getElementById("email").value;
        const description = document.getElementById("ticketDescription").value;
        const raum = document.getElementById("raum").value;

        fetch('http://127.0.0.1:5555/tickets', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                title: title, 
                name: name,
                email: email,
                description: description,
                raum: raum,
            }),
        })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok ' + response.statusText);
            }
            return response.json();
        })
        .then(data => console.log('Ticket erstellt:', data))
        .catch((error) => console.error('Error:', error));
    });
});

// Alter Code für einen Testserver mit JavaScript
//const express = require('express');

//const app = express();
//const port = 5500;

//app.use(express.json());

//app.post('/tickets', (req, res) => {
//    const { title, name, email, description, raum } = req.body;
    // Handle the ticket creation logic here
//    res.status(201).json({ message: 'Ticket created successfully', data: req.body });
//});

//app.listen(port, () => {
//    console.log(`Server running at http://127.0.0.1:${port}/`);
//});