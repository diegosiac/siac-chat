use actix_web::middleware;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::Error;

use crate::middlewares::auth::auth_middleware;
use crate::models::user::UserModel;
use crate::AppState;

#[get("/")]
async fn get_user_handler(data: web::Data<AppState>) -> impl Responder {
    let user_id = uuid::Uuid::new_v4();

    let query_result = sqlx::query_as!(UserModel, r#"SELECT * FROM users WHERE id = $1"#, user_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            let note_response = json!({"status": "success","data": json!({
                "user": "success"
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(Error::RowNotFound) => {
            return HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "User not found"
            }))
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

// #[patch("/update")]
// pub async fn update_user_handler(
//     opts: web::Query<FilterOptions>,
//     data: web::Data<AppState>,
// ) -> impl Responder {
//     let limit = opts.limit.unwrap_or(10);
//     let offset = (opts.page.unwrap_or(1) - 1) * limit;

//     let notes: Vec<NoteModel> = sqlx::query_as!(
//         NoteModel,
//         r#"SELECT * FROM notes ORDER by id LIMIT ? OFFSET ?"#,
//         limit as i32,
//         offset as i32
//     )
//     .fetch_all(&data.db)
//     .await
//     .unwrap();

//     let note_responses = notes
//         .into_iter()
//         .map(|note| filter_db_record(&note))
//         .collect::<Vec<NoteModelResponse>>();

//     let json_response = serde_json::json!({
//         "status": "success",
//         "results": note_responses.len(),
//         "notes": note_responses
//     });
//     HttpResponse::Ok().json(json_response)
// }

// fn filter_db_record(note: &NoteModel) -> NoteModelResponse {
//     NoteModelResponse {
//         id: note.id.to_owned(),
//         title: note.title.to_owned(),
//         content: note.content.to_owned(),
//         category: note.category.to_owned().unwrap(),
//         published: note.published != 0,
//         createdAt: note.created_at.unwrap(),
//         updatedAt: note.updated_at.unwrap(),
//     }
// }

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/user")
        .wrap(middleware::from_fn(auth_middleware))
        .service(get_user_handler);
    // .service(update_user_handler);

    conf.service(scope);
}
