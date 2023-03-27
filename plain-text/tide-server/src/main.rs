use tide::prelude::*;
use tide::Request;

async fn hello_world(_: Request<()>) -> tide::Result<String> {
    Ok("Hello, world!".to_string())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(hello_world);
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
