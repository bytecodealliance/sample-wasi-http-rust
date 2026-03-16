use wstd::http::{body::Bytes, Body, Request, Response, Result, StatusCode};
use wstd::time::{Duration, Instant};

#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>> {
    match req.uri().path_and_query().unwrap().as_str() {
        "/wait" => wait(req).await,
        "/echo" => echo(req).await,
        "/echo-headers" => echo_headers(req).await,
        "/echo-trailers" => echo_trailers(req).await,
        "/" => home(req).await,
        _ => not_found(req).await,
    }
}

async fn home(_req: Request<Body>) -> Result<Response<Body>> {
    // To send a single string as the response body:
    Ok(Response::new("Hello, wasi:http/proxy world!\n".into()))
}

async fn wait(_req: Request<Body>) -> Result<Response<Body>> {
    // Get the time now
    let now = Instant::now();

    // Sleep for one second.
    wstd::task::sleep(Duration::from_secs(1)).await;

    // Compute how long we slept for.
    let elapsed = Instant::now().duration_since(now).as_millis();

    Ok(Response::new(
        format!("slept for {elapsed} millis\n").into(),
    ))
}

async fn echo(req: Request<Body>) -> Result<Response<Body>> {
    // Stream data from the req body to the response body.
    let req_body = req.into_body();
    Ok(Response::new(req_body))
}

async fn echo_headers(req: Request<Body>) -> Result<Response<Body>> {
    let mut res = Response::builder();
    *res.headers_mut().unwrap() = req.into_parts().0.headers;
    Ok(res.body(().into()).expect("builder success"))
}

async fn echo_trailers(req: Request<Body>) -> Result<Response<Body>> {
    use http_body_util::{BodyExt, Full};
    let collected = req.into_body().into_boxed_body().collect().await?;
    let (trailers, report) = if let Some(trailers) = collected.trailers() {
        (
            Some(Ok(trailers.clone())),
            format!("recieved trailers: {trailers:?}"),
        )
    } else {
        (None, "request had no trailers".to_owned())
    };

    Ok(Response::new(Body::from_http_body(
        Full::new(Bytes::from(report)).with_trailers(async { trailers }),
    )))
}

async fn not_found(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(().into())
        .expect("builder succeeds"))
}
