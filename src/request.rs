#[derive(serde::Deserialize)]
struct Location {
    location: String,
    event: String,
}

#[derive(serde::Deserialize)]
struct Data {
    result: Vec<Location>,
    delivered: bool,
}

#[derive(serde::Deserialize)]
struct Package {
    error: bool,
    message: Option<String>,
    data: Option<Data>,
}

pub enum RequestError {
    FetchingError,
    BodyError,
    JSONError,
    PackageError(String),
    APIError,
    EmptyError,
    NoInfoError,
}

pub fn request(tracking_number: &str) -> Result<String, RequestError> {
    let mut package_option: Option<Package> = None;
    for _ in 0..3 {
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://api.haypost.am/trackingNumber/?trackingNumber={tracking_number}&lng=en"
        );
        let result = client
            .get(url)
            .send()
            .map_err(|_| RequestError::FetchingError)?;

        let json_str = result.text().map_err(|_| RequestError::BodyError)?;

        let package =
            serde_json::from_str::<Package>(&json_str).map_err(|_| RequestError::JSONError)?;

        if package.error {
            match package.data {
                Some(_) => match package.message {
                    Some(message) => {
                        return Err(RequestError::PackageError(format!("{}", message)));
                    }
                    None => (),
                },
                None => continue,
            }
        };
        package_option = Some(package);
    }

    let package = match package_option {
        Some(package) => package,
        None => return Err(RequestError::APIError),
    };

    let data = match package.data {
        Some(data) => data,
        None => return Err(RequestError::EmptyError),
    };

    if data.delivered {
        return Ok("Package delivered!".to_string());
    }

    let result = data.result;
    if result.len() == 0 {
        return Err(RequestError::NoInfoError);
    }

    let latest_result = &result[result.len() - 1];
    return Ok(format!(
        "{} - {}",
        latest_result.location, latest_result.event
    ));
}
