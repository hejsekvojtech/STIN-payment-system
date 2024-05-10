use crate::models::payment_request::PaymentRequest;
use crate::handlers::PaymentHandler;
use crate::enums::PaymentInputEnum;

pub struct CardPaymentHandler;

impl PaymentHandler for CardPaymentHandler {
    fn process(&self, payment_request: &PaymentRequest) -> Result<(), String> {
        self.pay(PaymentInputEnum::CardData(payment_request.mena.clone(), payment_request.castka))
    }
}
