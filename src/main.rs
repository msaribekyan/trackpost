use std::process::exit;
mod request;
use request::request::request;

#[tokio::main]
async fn main() {
    let tracking: [(&str, &str); _] = [
        ("Package 1", "UB848893366LV"),
        ("Package 2", "UB849021351LV"),
    ];

    let mut handles = vec![];
    for track in tracking {
        let handle = tokio::spawn(async move {
            request(track.0.to_string(), track.1.to_string()).await
        });
        handles.push(handle)
    }

    for handle in handles {
        let result = handle.await;

        match result {
            Ok(value) => println!("{value}"),
            Err(e) => eprintln!("{e}"),
        }
    }
    exit(0);
}
