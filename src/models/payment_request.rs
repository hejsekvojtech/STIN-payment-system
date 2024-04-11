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

         if self.seznam_polozek.len() == 0 {
            return Err(String::from("Item list cannot be empty"));
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
