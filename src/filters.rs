use crate::mailer::MailOptions;
use crate::{routes, RestApiConfig};
use warp::http::StatusCode;
use warp::Filter;

pub fn rest_smtp_api(
    api_config: RestApiConfig,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    mail_send(api_config).or(health())
}

/// GET /health
fn health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health").map(|| StatusCode::OK)
}

/// POST /send
fn mail_send(
    api_config: RestApiConfig,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("send")
        .and(warp::post())
        .and(with_api_config(api_config))
        .and(with_api_token())
        .and(json_body())
        .and_then(routes::send_mail)
}

fn with_api_config(
    api_config: RestApiConfig,
) -> impl Filter<Extract = (RestApiConfig,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || api_config.clone())
}

fn with_api_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::header("x-api-token")
}

fn json_body() -> impl Filter<Extract = (MailOptions,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 1024 * 16).and(warp::body::json())
}
