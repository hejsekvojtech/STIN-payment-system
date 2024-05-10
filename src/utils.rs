use crate::models::payment_request::PaymentRequest;
use crate::enums::PaymentErrorEnum;

pub fn error_to_string(error: PaymentErrorEnum) -> String {
    match error {
        PaymentErrorEnum::ErrUnsupportedType => String::from("Unsupported payment type"),
        PaymentErrorEnum::ErrEmptyList => String::from("Item list cannot be empty"),
        PaymentErrorEnum::ErrFutureDate => String::from("Payment date cannot be in the future"),
        PaymentErrorEnum::ErrNonPositiveAmount => String::from("Amount must be positive"),
    }
}

pub fn generate_xml_data(payment_request: &PaymentRequest) -> String {
    format!(
        r#"<PaymentRequest><castka>{}</castka><mena>{}</mena><datum>{}</datum><typ_platby>{}</typ_platby></PaymentRequest>"#,
        payment_request.castka, payment_request.mena, payment_request.datum, payment_request.typ_platby
    )
}