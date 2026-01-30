mod rms {
    tinc::include_proto!("rms");
}

mod json_convert;

use rms::gsc_gateway_api_tinc::GscGatewayApiTinc;
use rms::rms_config_api_tinc::RmsConfigApiTinc;

mod gateway_service;
use gateway_service::{GscApiProxy, RmsApiGateway};

use axum::body::{Body, Bytes};
use futures_util::stream;
use http::header::{CONTENT_TYPE, HeaderValue};
use serde_json::Value;
use std::convert::Infallible;
use tower_http::set_header::SetRequestHeaderLayer;

async fn hello_world() -> String {
    "hello, world".to_string()
}

fn map_json_body(body: Body, convert: fn(&Value) -> Value) -> Body {
    Body::from_stream(stream::once(async move {
        let bytes = match axum::body::to_bytes(body, usize::MAX).await {
            Ok(bytes) => bytes,
            Err(_) => return Ok::<Bytes, Infallible>(Bytes::new()),
        };

        Ok(convert_json_bytes(&bytes, convert))
    }))
}

fn convert_json_bytes(bytes: &Bytes, convert: fn(&Value) -> Value) -> Bytes {
    if bytes.is_empty() {
        return Bytes::copy_from_slice(bytes);
    }

    match serde_json::from_slice::<Value>(bytes) {
        Ok(value) => match serde_json::to_vec(&convert(&value)) {
            Ok(vec) => Bytes::from(vec),
            Err(_) => Bytes::copy_from_slice(bytes),
        },
        Err(_) => Bytes::copy_from_slice(bytes),
    }
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

    // V2 API
    let api_router = tinc_svc_rms.into_router().merge(tinc_svc_gsc.into_router());
    let app = app.nest("/rms/api/v2", api_router.clone());

    // V1 API with backward compatibility layers
    // Transform json body keys between lowerCamelCase and snake_case for backward compatibility
    let api_v1_router = api_router.layer(tower_http::map_request_body::MapRequestBodyLayer::new(
        |body| map_json_body(body, json_convert::json_keys_to_snake_case),
    ));
    let api_v1_router =
        api_v1_router.layer(tower_http::map_response_body::MapResponseBodyLayer::new(
            |body| map_json_body(body, json_convert::json_keys_to_lower_camel_case),
        ));
    // Add content-type for backward compatibility
    let api_v1_router = api_v1_router.layer(SetRequestHeaderLayer::overriding(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    ));

    let app = app.nest("/rms/api/v1", api_v1_router.clone());

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
