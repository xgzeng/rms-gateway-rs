mod rms {
    tinc::include_proto!("rms");
}

use rms::gsc_gateway_api_tinc::GscGatewayApiTinc;
use rms::rms_config_api_tinc::RmsConfigApiTinc;

mod gateway_service;
use gateway_service::{RmsApiGateway, GscApiProxy};

use http::header::{CONTENT_TYPE, HeaderValue};
use tower_http::set_header::SetRequestHeaderLayer;

async fn hello_world() -> String {
    "hello, world".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    use tinc::TincService;

    let rms_endpoint = "https://127.0.0.1:10000/";
    let gsc_endpoint = "https://127.0.0.1:8890/";

    let app = axum::Router::new().route("/", axum::routing::get(hello_world));

    let tinc_svc_rms = RmsConfigApiTinc::new(RmsApiGateway::new(rms_endpoint).await);
    let tinc_svc_gsc = GscGatewayApiTinc::new(GscApiProxy::new(gsc_endpoint).await);

    let api_router = tinc_svc_rms.into_router().merge(tinc_svc_gsc.into_router());

    let app = app.nest("/rms/api/v1", api_router.clone());
    let app = app.nest("/rms/api/v2", api_router);

    // for compability
    let app = app.layer(SetRequestHeaderLayer::overriding(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    ));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
