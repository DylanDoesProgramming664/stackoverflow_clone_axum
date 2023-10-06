use axum::{
    extract::{Path, State},
    Json,
};
use axum_macros::debug_handler;
mod handlers_inner;

use crate::{
    model::{Answer, AnswerDetails, Question, QuestionDetails},
    persistence::database_controller::DatabaseController,
    Result,
};

#[debug_handler]
pub async fn create_question(
    State(dc): State<DatabaseController>,
    Json(question): Json<Question>,
) -> Result<Json<QuestionDetails>> {
    return handlers_inner::create_question(dc, question).await;
}

#[debug_handler]
pub async fn list_questions(
    State(dc): State<DatabaseController>,
) -> Result<Json<Vec<QuestionDetails>>> {
    return handlers_inner::list_questions(dc).await;
}

#[debug_handler]
pub async fn delete_question(
    State(dc): State<DatabaseController>,
    Path(q_id): Path<String>,
) -> Result<()> {
    return handlers_inner::delete_question(dc, q_id.into()).await;
}

#[debug_handler]
pub async fn create_answer(
    State(dc): State<DatabaseController>,
    Json(answer): Json<Answer>,
) -> Result<Json<AnswerDetails>> {
    return handlers_inner::create_answer(dc, answer).await;
}

#[debug_handler]
pub async fn list_answers(
    State(dc): State<DatabaseController>,
    Path(q_id): Path<String>,
) -> Result<Json<Vec<AnswerDetails>>> {
    return handlers_inner::list_answers(dc, q_id.into()).await;
}

#[debug_handler]
pub async fn delete_answer(
    State(dc): State<DatabaseController>,
    Path(a_id): Path<String>,
) -> Result<()> {
    return handlers_inner::delete_answer(dc, a_id.into()).await;
}
