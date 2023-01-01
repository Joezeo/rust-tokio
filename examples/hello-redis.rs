use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let ret = client.get("hello").await?;

    println!("Get result: {:?}", ret);

    Ok(())
}
