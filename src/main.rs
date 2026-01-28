use anyhow::Result;

mod rms {
    tinc::include_proto!("rms");
}

async fn hello_world()-> String {
    "hello, world".to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = axum::Router::new().route("/", axum::routing::get(hello_world));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    println!("listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
