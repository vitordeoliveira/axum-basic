#![allow(unused)] // For beginning only.

use anyhow::Result;
// use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Mary").await?.print().await?;
    hc.do_get("/hello2/Mary").await?.print().await?;

    Ok(())
}
