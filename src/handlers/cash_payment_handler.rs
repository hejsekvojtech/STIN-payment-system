use crate::models::payment_request::PaymentRequest;
use crate::handlers::PaymentHandler;
use crate::utils::generate_xml_data;
use crate::enums::PaymentInputEnum;
pub struct CashPaymentHandler;

impl PaymentHandler for CashPaymentHandler {
    fn process(&self, payment_request: &PaymentRequest) -> Result<(), String> {
        self.pay(PaymentInputEnum::CashData(generate_xml_data(payment_request)))
    }
}
