use crate::models::payment_request::PaymentRequest;
use crate::enums::PaymentInputEnum;
use crate::utils::error_to_string;
use crate::enums::PaymentErrorEnum::{ErrEmptyList, ErrFutureDate, ErrNonPositiveAmount};

use chrono::Utc;

pub mod card_payment_handler;
pub mod cash_payment_handler;

pub trait PaymentHandler {
    fn process(&self, payment_request: &PaymentRequest) -> Result<(), String>;

    fn validate(&self, payment_request: &PaymentRequest) -> Result<(), String> {
        if payment_request.castka <= 0.0 {
            return Err(error_to_string(ErrNonPositiveAmount));
        }

        if payment_request.seznam_polozek.is_empty() {
            return Err(error_to_string(ErrEmptyList));
        }

        if payment_request.datum > Utc::now() {
            return Err(error_to_string(ErrFutureDate));
        }

        Ok(())
    }

    fn pay(&self, payment_input: PaymentInputEnum) -> Result<(), String> {
        match payment_input {
            PaymentInputEnum::CashData(xml) => {
                println!("Payment - XML data: {}", xml);
                Ok(())
            }
            PaymentInputEnum::CardData(currency, amount) => {
                println!("Payment - Amount: {}, Currency: {}", amount, currency);
                Ok(())
            }
        }
    }
}
