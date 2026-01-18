use std::env;
use std::process::exit;
use reqwest::blocking::{Client};
use serde::{Deserialize};
use serde_json::{from_str};

#[derive(Debug, Deserialize)]
struct Result
{
    #[serde(rename="local_Date")]
    local_date: String,
    country: String,
    location: String,
    event: String,
    category: String,
    #[serde(rename="nextOffice")]
    next_office: String,
    #[serde(rename="extraInformation")]
    extra_information: String,
    #[serde(rename="eventDate")]
    event_date: String,
}

#[derive(Debug, Deserialize)]
struct Data
{
    result: Vec<Result>,
    #[serde(rename="estimatedDelivery")]
    estimated_delivery: Option<String>,
    #[serde(rename="isEMS")]
    is_ems: bool,
    delivered: bool,
}

#[derive(Debug, Deserialize)]
struct Package
{
    error: bool,
    message: Option<String>,
    data: Data,
}

fn request(tracking_number: String) {
    let client = Client::new();
    let url = format!("https://api.haypost.am/trackingNumber/?trackingNumber={}&lng=en", tracking_number);
    match client.get(url).send() {
        Ok(result) => {
            let json_str = result.text().ok().unwrap();
            match from_str::<Package>(&json_str) {
                Ok(package) => {
                    if package.error {
                        println!("{}", package.message.unwrap())
                    }
                    else
                    {
                        let result = package.data.result;
                        let latest_result = &result[result.len() - 1];
                        println!("{}: {}", latest_result.location, latest_result.event)
                    }
                },
                Err(e) => {
                    println!("{}", json_str.clone());
                    println!("JSON error: {:#?}", e);
                },
            }
        },
        Err(e) => println!("Fetching error: {:#?}", e),
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Parameter not provided");
        exit(1);
    }
    else
    {
        request(args[1].clone());
    }
    exit(0);
}
