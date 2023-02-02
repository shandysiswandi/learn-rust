use crate::{model::todo::Todo, state::AppState};
use actix_web::{web, HttpResponse, Result};

/// To get all todos data
///
/// GET /todos
pub async fn todos(deps: web::Data<AppState>) -> HttpResponse {
    println!("hit todos");
    match deps.todo_svc.find() {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// To get one todo by id
///
/// GET /todos/{id}
pub async fn todo_by_id(deps: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    println!("hit todo_by_id");
    match deps.todo_svc.find_one(id.to_string()) {
        Ok(opt) => match opt {
            Some(todo) => HttpResponse::Ok().json(todo),
            None => {
                println!("not found");
                HttpResponse::NotFound().finish()
            }
        },
        Err(e) => {
            println!("not found {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// To create todo
///
/// POST /todos
pub async fn create_todo(deps: web::Data<AppState>, body: web::Json<Todo>) -> HttpResponse {
    let todo = body.into_inner();
    match deps.todo_svc.create(todo) {
        Ok(_) => HttpResponse::Ok().json(vec![""]),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// To update tod by id
///
/// PUT /todos/{id}
pub async fn update_todo(_id: web::Path<u32>, body: web::Json<Todo>) -> Result<HttpResponse> {
    let a = body.into_inner();
    Ok(HttpResponse::Ok().json(a))
}

/// To delete todo by id
///
/// DELETE /todos/{id}
pub async fn delete_todo(_id: web::Path<u32>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Todo::new()))
}

// #[cfg(test)]
// mod unit_test {
//     use super::*;
//     use actix_web::{body, http::StatusCode, web::Bytes};
//     use serde_json::to_string;

//     trait BodyTest {
//         fn as_str(&self) -> &str;
//     }

//     impl BodyTest for Bytes {
//         fn as_str(&self) -> &str {
//             std::str::from_utf8(self).unwrap()
//         }
//     }

//     #[actix_web::test]
//     async fn test_endpoint_todos_ok() {
//         // // Arrange
//         // let resp = todos().unwrap();
//         // let todos = vec![
//         //     Todo::new(),
//         //     Todo::new(),
//         //     Todo::new(),
//         //     Todo::new(),
//         //     Todo::new(),
//         // ];

//         // // Act
//         // let status = resp.status();
//         // let byte_body = body::to_bytes(resp.into_body()).await.unwrap();
//         // let result = to_string(&todos).unwrap();

//         // //  Assert
//         // assert_eq!(status, StatusCode::OK);
//         // assert_eq!(byte_body.as_str(), result.as_str());
//     }
// }
