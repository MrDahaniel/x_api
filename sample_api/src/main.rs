use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::sync::Arc;
use tide::{Body, Request, Response};

#[derive(Clone, Debug)]
struct State {
    students: Arc<RwLock<HashMap<String, Student>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Student {
    code: String,
    name: String,
    level: u8,
    program: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = State {
        students: Default::default(),
    };

    let mut app = tide::with_state(state);

    app.at("/students")
        .post(|mut req: Request<State>| async move {
            let student: Student = req.body_json().await?;
            println!("student name: {}", student.name);

            let mut student_state = req.state().students.write().await;

            student_state.insert(String::from(&student.code), student.clone());

            let mut res = Response::new(201);
            res.set_body(Body::from_json(&student)?);

            Ok(res)
        })
        .get(|req: Request<State>| async move {
            let students = req.state().students.read().await;

            let student_vec: Vec<Student> = students.values().cloned().collect();

            let mut res = Response::new(200);
            res.set_body(Body::from_json(&student_vec)?);

            Ok(res)
        });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
