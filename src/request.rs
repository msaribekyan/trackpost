pub mod request {
    #[derive(serde::Deserialize)]
    struct Location {
        #[allow(dead_code)]
        #[serde(rename = "local_Date")]
        local_date: String,
        #[allow(dead_code)]
        country: String,
        location: String,
        event: String,
        #[allow(dead_code)]
        category: String,
        #[allow(dead_code)]
        #[serde(rename = "nextOffice")]
        next_office: String,
        #[allow(dead_code)]
        #[serde(rename = "extraInformation")]
        extra_information: String,
        #[allow(dead_code)]
        #[serde(rename = "eventDate")]
        event_date: String,
    }

    #[derive(serde::Deserialize)]
    struct Data {
        result: Vec<Location>,
        #[allow(dead_code)]
        #[serde(rename = "estimatedDelivery")]
        estimated_delivery: Option<String>,
        #[allow(dead_code)]
        #[serde(rename = "isEMS")]
        is_ems: bool,
        delivered: bool,
    }

    #[derive(serde::Deserialize)]
    struct Package {
        error: bool,
        message: Option<String>,
        data: Data,
    }

    pub fn request(tracking_number: &str) -> Result<String, String> {
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://api.haypost.am/trackingNumber/?trackingNumber={tracking_number}&lng=en"
        );
        let result = match client.get(url).send() {
            Ok(res) => res,
            Err(e) => return Err(format!("Fetching error: {:#?}", e)),
        };

        let json_str = match result.text() {
            Ok(json_str) => json_str,
            Err(e) => return Err(format!("Something error: {:#?}", e)),
        };

        let package = match serde_json::from_str::<Package>(&json_str) {
            Ok(package) => package,
            Err(e) => return Err(format!("JSON error: {:#?}", e)),
        };

        if package.error {
            match package.message {
                Some(message) => return Err(format!("{}", message)),
                None => return Err(format!("Package error with no message.")),
            }
        }

        if package.data.delivered {
            return Ok("Package delivered!".to_string());
        }

        let result = package.data.result;
        if result.len() == 0 {
            return Err(format!("No info."));
        }

        let latest_result = &result[result.len() - 1];
        return Ok(format!("{} - {}", latest_result.location, latest_result.event));
    }
}
