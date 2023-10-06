use sqlx::{query, PgPool};
use uuid::Uuid;

use crate::{
    model::{Answer, Question},
    Error, Result,
};

// -- DatabaseController

use crate::model::{AnswerDetails, QuestionDetails};

#[derive(Clone)]
pub struct DatabaseController {
    db: PgPool,
}

impl DatabaseController {
    pub fn new(db: PgPool) -> Result<Self> {
        return Ok(Self { db });
    }

    pub async fn create_question(&self, question: Question) -> Result<QuestionDetails> {
        let record = query!(
            r#"
                INSERT INTO questions (title, description)
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            question.title,
            question.description,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| Error::DBError(Box::new(err)))?;
        println!("question record: {:?}", record);

        return Ok(QuestionDetails {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            posted_at: record.posted_at.to_string(),
            modified_at: record.modified_at.to_string(),
        });
    }

    pub async fn list_questions(&self) -> Result<Vec<QuestionDetails>> {
        let records = query!(
            r#"
                SELECT * FROM questions
            "#,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| Error::DBError(Box::new(err)))?;
        println!("question records: {:?}", records);

        let questions: Vec<QuestionDetails> = records
            .iter()
            .map(|record| QuestionDetails {
                question_uuid: record.question_uuid.to_string(),
                title: record.title.to_string(),
                description: record.description.to_string(),
                posted_at: record.posted_at.to_string(),
                modified_at: record.modified_at.to_string(),
            })
            .collect();

        return Ok(questions);
    }

    pub async fn delete_question(&self, question_uuid: String) -> Result<()> {
        let uuid =
            Uuid::parse_str(question_uuid.as_ref()).map_err(|_| Error::MissingQuestionId {
                id: Uuid::from_u128(0),
            })?;

        let deletion = query!(
            r#"
                DELETE FROM questions
                WHERE question_uuid = $1
            "#,
            uuid
        )
        .execute(&self.db)
        .await
        .map_err(|err| Error::DBError(Box::new(err)))?;
        println!("question deletion: {:?}", deletion);

        return Ok(());
    }

    pub async fn create_answer(&self, answer: Answer) -> Result<AnswerDetails> {
        let uuid = Uuid::parse_str(answer.question_uuid.as_ref()).map_err(|_| {
            Error::MissingQuestionId {
                id: Uuid::from_u128(0),
            }
        })?;
        println!("uuid: {:?}", uuid);

        let record = query!(
            r#"
                INSERT INTO answers (question_uuid, content)
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            uuid,
            answer.content,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| Error::DBError(Box::new(err)))?;

        println!("answer record: {:?}", record);

        return Ok(AnswerDetails {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content.to_string(),
            posted_at: record.posted_at.to_string(),
            modified_at: record.modified_at.to_string(),
        });
    }

    pub async fn list_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetails>> {
        let uuid =
            Uuid::parse_str(question_uuid.as_ref()).map_err(|_| Error::MissingQuestionId {
                id: Uuid::from_u128(0),
            })?;

        let records = query!(
            r#"
                SELECT * FROM answers WHERE question_uuid = $1
            "#,
            uuid
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| Error::DBError(Box::new(err)))?;
        println!("answer records: {:?}", records);

        let answers: Vec<AnswerDetails> = records
            .iter()
            .map(|record| AnswerDetails {
                answer_uuid: record.answer_uuid.to_string(),
                question_uuid: record.question_uuid.to_string(),
                content: record.content.to_string(),
                posted_at: record.posted_at.to_string(),
                modified_at: record.modified_at.to_string(),
            })
            .collect();
        println!("answers: {:?}", answers);

        return Ok(answers);
    }

    pub async fn delete_answer(&self, answer_uuid: String) -> Result<()> {
        let uuid = Uuid::parse_str(answer_uuid.as_ref()).map_err(|_| Error::MissingAnswerId {
            id: Uuid::from_u128(0),
        })?;

        println!("uuid: {:?}", uuid);

        let deletion = query!(
            r#"
                DELETE FROM answers
                WHERE answer_uuid = $1
            "#,
            uuid
        )
        .execute(&self.db)
        .await
        .map_err(|err| Error::DBError(Box::new(err)))?;
        println!("answer deletion: {:?}", deletion);

        return Ok(());
    }
}
