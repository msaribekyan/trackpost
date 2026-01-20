use std::process::exit;
mod request;
use request::request::request;

fn main() {
    let tracking: [(&str, &str); _] = [
        ("Package 1", "UB848893366LV"),
        ("Package 2", "UB849021351LV"),
    ];

    for track in tracking {
        match request(track.1) {
            Ok(r) => println!("{}: {r}", track.0),
            Err(e) => println!("{}: Error: {e}", track.0),
        };
    }

    exit(0);
}
