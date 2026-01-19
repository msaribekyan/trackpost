pub mod request {
    #[derive(serde::Deserialize)]
    struct Result
    {
        #[allow(dead_code)]
        #[serde(rename="local_Date")]
        local_date: String,
        #[allow(dead_code)]
        country: String,
        location: String,
        event: String,
        #[allow(dead_code)]
        category: String,
        #[allow(dead_code)]
        #[serde(rename="nextOffice")]
        next_office: String,
        #[allow(dead_code)]
        #[serde(rename="extraInformation")]
        extra_information: String,
        #[allow(dead_code)]
        #[serde(rename="eventDate")]
        event_date: String,
    }

    #[derive(serde::Deserialize)]
    struct Data
    {
        result: Vec<Result>,
        #[allow(dead_code)]
        #[serde(rename="estimatedDelivery")]
        estimated_delivery: Option<String>,
        #[allow(dead_code)]
        #[serde(rename="isEMS")]
        is_ems: bool,
        delivered: bool,
    }

    #[derive(serde::Deserialize)]
    struct Package
    {
        error: bool,
        message: Option<String>,
        data: Data,
    }

    pub async fn request(name: String, tracking_number: String) -> String {
        let mut out_string = format!("{name} ({tracking_number}): ");
        let client = reqwest::Client::new();
        let url = format!("https://api.haypost.am/trackingNumber/?trackingNumber={}&lng=en", tracking_number);
        match client.get(url).send().await {
            Ok(result) => {
                let json_str = result.text().await.ok().unwrap();
                match serde_json::from_str::<Package>(&json_str) {
                    Ok(package) => {
                        if package.error {
                            out_string.push_str(&format!("{}", package.message.unwrap()))
                        }
                        else
                        {
                            if package.data.delivered {
                               out_string.push_str(&format!("Package delilvered"));
                            }
                            else
                            {
                                let result = package.data.result;
                                let latest_result = &result[result.len() - 1];
                                out_string.push_str(&format!("{} - {}", latest_result.location, latest_result.event))
                            }
                        }
                    },
                    Err(e) => {
                        out_string.push_str(&format!("JSON error: {:#?}", e));
                    },
                }
            },
           Err(e) => out_string.push_str(&format!("Fetching error: {:#?}", e)),
        }
        out_string
    }
}
