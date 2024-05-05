pub fn entry_page() -> &'static str {
    return r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Enter Martial Art</title>
        </head>
        <body>
            <h1>Martial Art Information</h1>
            <form id="createForm">
                <label for="title">Title:</label>
                <input type="text" id="title" name="title"><br>
                <label for="category">Category:</label>
                <input type="text" id="category" name="category"><br>
                <label for="description">Description:</label>
                <input type="text" id="description" name="description"><br>
                <label for="origin">Origin:</label>
                <input type="text" id="origin" name="origin"><br>
                <label for="founded_date">Founded Date:</label>
                <input type="text" id="founded_date" name="founded_date"><br>
                <button type="submit">Submit</button>
            </form>
    
            <script>
                document.getElementById('createForm').addEventListener('submit', async (event) => {
                    event.preventDefault();
    
                    const formData = new FormData(event.target);
                    const requestData = Object.fromEntries(formData.entries());
    
                    console.log(requestData);
    
                    try {
                        const response = await fetch('/api/martial_arts/', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json'
                            },
                            body: JSON.stringify(requestData)
                        });
    
                        if (!response.ok) {
                            throw new Error('Failed to create martial art');
                        }
                        alert('Martial art created successfully!');
                    } catch (error) {
                        console.error(error);
                        alert('Failed to create martial art');
                    }
                })
            </script>
        </body>
        </html>
    "#;
}