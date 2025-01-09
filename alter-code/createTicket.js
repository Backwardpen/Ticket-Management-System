document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('createTicketForm');

    form.addEventListener('submit', async (event) => {
        event.preventDefault();

        const formData = new FormData(form);
        const data = Object.fromEntries(formData.entries());

        try {
            const response = await fetch('http://127.0.0.1:8080/createTicket', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(data)
            });

            const result = await response.json();

            if (response.ok) {
                alert(result);
                form.reset();
            } else {
                alert(result);
            }
        } catch (error) {
            console.error('Error:', error);
            alert('An error occurred while creating the ticket.');
        }
    });
});