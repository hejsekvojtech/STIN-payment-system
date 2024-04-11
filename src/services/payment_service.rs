use crate::models::payment_request::ProcessPayment;

pub async fn process_payment<T: ProcessPayment>(payment: T) -> Result<(), String> {
    payment.process()
}
