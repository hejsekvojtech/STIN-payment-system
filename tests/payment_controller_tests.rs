use actix_web::http::StatusCode;
use actix_web::test;
use actix_web::{web, App};
use chrono::Utc;
use stin_payment_program::controllers::payment_controller;
use stin_payment_program::models::payment_request::PaymentRequest;
use stin_payment_program::utils::error_to_string;
use stin_payment_program::enums::PaymentErrorEnum::{ErrFutureDate, ErrUnsupportedType};

#[actix_rt::test]
async fn test_process_payment_success() {
    let mut app = test::init_service(App::new().route(
        "/process_payment",
        web::post().to(payment_controller::process_payment),
    ))
    .await;

    let payment_request = PaymentRequest {
        castka: 100.0,
        mena: String::from("CZK"),
        datum: Utc::now(),
        typ_platby: String::from("CARD"),
        seznam_polozek: vec![String::from("máslo")],
    };

    let req = test::TestRequest::post()
        .uri("/process_payment")
        .set_json(&payment_request)
        .to_request();

    let response = test::call_service(&mut app, req).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = test::read_body(response).await;
    assert_eq!(body, r#"1"#);
}

#[actix_rt::test]
async fn test_process_payment_future_date() {
    let mut app = test::init_service(App::new().route(
        "/process_payment",
        web::post().to(payment_controller::process_payment),
    ))
    .await;

    let payment_request = PaymentRequest {
        castka: 100.0,
        mena: String::from("EUR"),
        datum: Utc::now() + chrono::Duration::hours(1),
        typ_platby: String::from("CARD"),
        seznam_polozek: vec![String::from("máslo")],
    };

    let req = test::TestRequest::post()
        .uri("/process_payment")
        .set_json(&payment_request)
        .to_request();

    let response = test::call_service(&mut app, req).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = test::read_body(response).await;
    assert_eq!(body, error_to_string(ErrFutureDate));
}

#[actix_rt::test]
async fn test_process_payment_unsupported_type() {
    let mut app = test::init_service(App::new().route(
        "/process_payment",
        web::post().to(payment_controller::process_payment),
    ))
    .await;

    let payment_request = PaymentRequest {
        castka: 100.0,
        mena: String::from("EUR"),
        datum: Utc::now(),
        typ_platby: String::from("CHECK"),
        seznam_polozek: vec![String::from("máslo")],
    };

    let req = test::TestRequest::post()
        .uri("/process_payment")
        .set_json(&payment_request)
        .to_request();

    let response = test::call_service(&mut app, req).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = test::read_body(response).await;
    assert_eq!(body, error_to_string(ErrUnsupportedType));
}