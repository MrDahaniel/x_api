const res = require('express/lib/response');

// Esta constante funciona para inicializar la api haciendo uso de express.
const app = require('express')();
const PORT = 8081;

// Listado de estudiantes que devolvera el endpoint
const students = {
    estudiantes : {
        student1: {
            nombre: 'Juan',
            apellido: 'Duarte',
            semestre: 8
        },
        student2: {
            nombre: 'Daniel',
            apellido: 'Delgado',
            semestre: 10
        }
    }
};

app.listen(
    PORT, 
    () => console.log(`La api de prueba a sido creada. Se ejecuta en http://localhost:${PORT}`)
);

app.get('/students', (request, response) => {
    response.status(200).send(students);
});

