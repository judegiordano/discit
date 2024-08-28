use discit_backend::logger;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::Value;

pub async fn handler(_: LambdaEvent<Value>) -> Result<(), Error> {
    tracing::info!("cron hit");
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    run(service_fn(handler)).await
}
