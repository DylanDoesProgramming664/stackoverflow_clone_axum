#![allow(unused)]
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// -- QUESTIONS

#[derive(Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct QuestionDetails {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub posted_at: String,
    pub modified_at: String,
}

// -- ANSWERS

#[derive(Deserialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnswerDetails {
    pub question_uuid: String,
    pub answer_uuid: String,
    pub content: String,
    pub posted_at: String,
    pub modified_at: String,
}
