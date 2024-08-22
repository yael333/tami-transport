fetch('http://localhost:4000/json')
    .then(response => response.json())
    .then(data => console.log(data));
