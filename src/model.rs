use serde::{Deserialize, Serialize};

// -- QUESTIONS

#[derive(Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct QuestionDetails {
    pub question_uuid: Box<str>,
    pub title: String,
    pub description: String,
    pub posted_at: Box<str>,
    pub modified_at: String,
}

// -- ANSWERS

#[derive(Deserialize)]
pub struct Answer {
    pub question_uuid: Box<str>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnswerDetails {
    pub question_uuid: Box<str>,
    pub answer_uuid: Box<str>,
    pub content: String,
    pub posted_at: Box<str>,
    pub modified_at: String,
}
