use sqlx::{Connection, Executor, MsSql};
use sqlx_test::new;

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn it_connects() -> anyhow::Result<()> {
    new::<MsSql>().await?;
    Ok(())
}
