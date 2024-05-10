use crate::handlers::PaymentHandler;
use crate::handlers::{card_payment_handler::CardPaymentHandler,cash_payment_handler::CashPaymentHandler};
use crate::models::payment_request::PaymentRequest;
use crate::enums::PaymentErrorEnum::ErrUnsupportedType;
use crate::utils::error_to_string;

pub async fn process_payment(payment_request: &PaymentRequest) -> Result<(), String> {
    let handler: Box<dyn PaymentHandler> = match payment_request.typ_platby.as_str() {
        "CARD" => Box::new(CardPaymentHandler),
        "CASH" => Box::new(CashPaymentHandler),
        _ => return Err(error_to_string(ErrUnsupportedType)),
    };

    match handler.validate(payment_request) {
        Ok(_) => handler.process(payment_request),
        Err(err) => Err(err),
    }
}
