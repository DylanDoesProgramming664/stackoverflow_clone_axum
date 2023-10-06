use anyhow::Result;
use serde_json::json;

#[tokio::test]
pub async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:1337")?;

    hc.do_post(
        "/question",
        json!({
            "title": "test title",
            "description": "test description"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/answer",
        json!({
            "question_uuid": "14186c54-5ce5-413c-807b-59772f764679",
            "content": "42"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/questions").await?.print().await?;

    hc.do_get("/answers/14186c54-5ce5-413c-807b-59772f764679")
        .await?
        .print()
        .await?;

    return Ok(());
}
