// DAS MUSS JETZT IN RUST PROGRAMMIERT WERDEN UND ZWAR ORDENTLICH!!!
// Alert: Alter Code
const express = require('express');
const cors = require('cors');

const app = express();
const port = 5555;

app.use(express.json());
app.use(cors());

app.post('/tickets', (req, res) => {
    // Handle the ticket creation logic here
    res.status(201).json({ message: 'Ticket created successfully', data: req.body });
});

app.listen(port, () => {
    console.log(`Server running at http://127.0.0.1:${port}/`);
});