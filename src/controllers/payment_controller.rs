use crate::models::payment_request::PaymentRequest;
use crate::services::payment_service;
use actix_web::{web, HttpResponse, Responder};

pub async fn process_payment(payment: web::Json<PaymentRequest>) -> impl Responder {
    let payment_request = payment.into_inner();

    match payment_service::process_payment(&payment_request).await {
        Ok(_) => HttpResponse::Ok().json(1),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
