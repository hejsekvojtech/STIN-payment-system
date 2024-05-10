pub enum PaymentInputEnum {
    CashData(String),
    CardData(String, f64),
}

pub enum PaymentErrorEnum {
    ErrUnsupportedType,
    ErrEmptyList,
    ErrFutureDate,
    ErrNonPositiveAmount,
}