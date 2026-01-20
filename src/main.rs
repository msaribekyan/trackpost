use std::process::exit;
mod request;
use request::request::request;

fn main() {
    let tracking: [(&str, &str); _] = [
        ("Package 1", "UB848893366LV"),
        ("Package 2", "UB849021351LV"),
    ];

    for track in tracking {
        let resp = match request(track.1) {
            Ok(r) => println!("{r}"),
            Err(e) => println!("Error: {e}"),
        };
    }

    exit(0);
}
