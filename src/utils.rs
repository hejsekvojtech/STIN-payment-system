
pub enum PaymentErrorEnum {
    ErrUnsupportedType,
    ErrEmptyList,
    ErrFutureDate,
    ErrNonPositiveAmount,
}

pub fn error_to_string(error: PaymentErrorEnum) -> String {
    match error {
        PaymentErrorEnum::ErrUnsupportedType => String::from("Unsupported payment type"),
        PaymentErrorEnum::ErrEmptyList => String::from("Item list cannot be empty"),
        PaymentErrorEnum::ErrFutureDate => String::from("Payment date cannot be in the future"),
        PaymentErrorEnum::ErrNonPositiveAmount => String::from("Amount must be positive"),
    }
}
