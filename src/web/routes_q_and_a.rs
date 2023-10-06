use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    handlers::{
        create_answer, create_question, delete_answer, delete_question, list_answers,
        list_questions,
    },
    persistence::database_controller::DatabaseController,
};

pub fn routes(dc: DatabaseController) -> Router {
    return Router::new()
        .route("/questions", get(list_questions))
        .route("/question", post(create_question))
        .route("/question/:question_id", delete(delete_question))
        .route("/answers/:question_id", get(list_answers))
        .route("/answer", post(create_answer))
        .route("/answer/:answer_id", delete(delete_answer))
        .with_state(dc);
}
