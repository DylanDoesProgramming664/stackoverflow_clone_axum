use crate::{
    model::{Answer, AnswerDetails, Question, QuestionDetails},
    persistence::database_controller::DatabaseController,
    Result,
};

use axum::Json;

pub async fn create_question(
    dc: DatabaseController,
    question: Question,
) -> Result<Json<QuestionDetails>> {
    return Ok(Json(dc.create_question(question).await?));
}

pub async fn list_questions(dc: DatabaseController) -> Result<Json<Vec<QuestionDetails>>> {
    return Ok(Json(dc.list_questions().await?));
}

pub async fn delete_question(dc: DatabaseController, q_id: String) -> Result<()> {
    dc.delete_question(q_id).await?;
    return Ok(());
}

pub async fn create_answer(dc: DatabaseController, answer: Answer) -> Result<Json<AnswerDetails>> {
    return Ok(Json(dc.create_answer(answer).await?));
}

pub async fn list_answers(
    dc: DatabaseController,
    q_id: String,
) -> Result<Json<Vec<AnswerDetails>>> {
    return Ok(Json(dc.list_answers(q_id).await?));
}

pub async fn delete_answer(dc: DatabaseController, a_id: String) -> Result<()> {
    dc.delete_answer(a_id).await?;
    return Ok(());
}
