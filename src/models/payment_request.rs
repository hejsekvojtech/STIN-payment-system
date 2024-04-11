use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub castka: f64,
    pub mena: String,
    pub datum: DateTime<Utc>,
    pub typ_platby: String,
    pub seznam_polozek: Vec<String>,
}

pub trait ProcessPayment {
    fn process(&self) -> Result<(), String>;

    fn validate(&self) -> Result<(), String>;
}

impl ProcessPayment for PaymentRequest {
    fn process(&self) -> Result<(), String> {
        match self.typ_platby.as_str() {
            "CARD" => self.card(),
            "CASH" => self.cash(),
            _ => Err(String::from("Unsupported payment type")),
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.castka <= 0.0 {
            return Err(String::from("Amount must be positive"));
        }

        if self.seznam_polozek.is_empty() {
            return Err(String::from("Item list cannot be empty"));
        }

        if self.datum > Utc::now() {
            return Err(String::from("Payment date cannot be in the future"));
        }

        Ok(())
    }
}

impl PaymentRequest {
    fn card(&self) -> Result<(), String> {
        println!(
            "Platba kartou - castka: {}, mena: {}",
            self.castka, self.mena
        );
        Ok(())
    }

    fn cash(&self) -> Result<(), String> {
        let xml_data = self.to_xml();
        println!("Platba hotově - XML data: {}", xml_data);
        Ok(())
    }

    fn to_xml(&self) -> String {
        format!(
            r#"<PaymentRequest><castka>{}</castka><mena>{}</mena><datum>{}</datum><typ_platby>{}</typ_platby></PaymentRequest>"#,
            self.castka, self.mena, self.datum, self.typ_platby
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_request_positive_amount() {
        let payment_request = PaymentRequest {
            castka: 100.0,
            mena: String::from("CZK"),
            datum: Utc::now(),
            typ_platby: String::from("CARD"),
            seznam_polozek: vec![String::from("máslo")],
        };
        assert!(payment_request.validate().is_ok());
    }

    #[test]
    fn test_payment_request_negative_amount() {
        let payment_request = PaymentRequest {
            castka: -40.0,
            mena: String::from("CZK"),
            datum: Utc::now(),
            typ_platby: String::from("CARD"),
            seznam_polozek: vec![String::from("máslo")],
        };
        assert!(payment_request.validate().is_err());
    }

    #[test]
    fn test_payment_request_empty_list() {
        let payment_request = PaymentRequest {
            castka: 40.0,
            mena: String::from("CZK"),
            datum: Utc::now(),
            typ_platby: String::from("CARD"),
            seznam_polozek: vec![],
        };
        assert!(payment_request.validate().is_err());
    }

    #[test]
    fn test_payment_request_future_date() {
        let payment_request = PaymentRequest {
            castka: 40.0,
            mena: String::from("CZK"),
            datum: Utc::now() + chrono::Duration::days(2),
            typ_platby: String::from("CARD"),
            seznam_polozek: vec![String::from("máslo")],
        };
        assert!(payment_request.validate().is_err());
    }

    #[test]
    fn test_payment_request_past_date() {
        let payment_request = PaymentRequest {
            castka: 40.0,
            mena: String::from("CZK"),
            datum: Utc::now() - chrono::Duration::hours(2),
            typ_platby: String::from("CARD"),
            seznam_polozek: vec![String::from("máslo")],
        };
        assert!(payment_request.validate().is_ok());
    }

    #[test]
    fn test_payment_request_unsupported_payment() {
        let payment_request = PaymentRequest {
            castka: 40.0,
            mena: String::from("CZK"),
            datum: Utc::now(),
            typ_platby: String::from("NFT"),
            seznam_polozek: vec![String::from("máslo")],
        };
        assert!(payment_request.process().is_err());
    }

    #[test]
    fn test_payment_request_to_xml() {
        let payment_request = PaymentRequest {
            castka: 100.0,
            mena: String::from("CZK"),
            datum: Utc::now(),
            typ_platby: String::from("CASH"),
            seznam_polozek: vec![String::from("máslo")],
        };
        let xml = payment_request.to_xml();
        assert!(xml.contains("<PaymentRequest>"));
        assert!(xml.contains("</PaymentRequest>"));
        assert!(xml.contains("<castka>100</castka>"));
        assert!(xml.contains("<mena>CZK</mena>"));
        assert!(xml.contains("<typ_platby>CASH</typ_platby>"));
    }
}
