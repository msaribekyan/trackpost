use std::process::exit;
mod request;
use request::RequestError;
use request::request;

fn main() {
    let tracking: [(&str, &str); _] = [
        ("Package 1", "UB849330327LV"),
        ("Package 2", "UB849329641LV"),
    ];

    for (name, number) in tracking {
        match request(number) {
            Ok(r) => println!("{}: {r}", name),
            Err(e) => match e {
                RequestError::FetchingError => println!("{}: Fetching error", name),
                RequestError::BodyError => println!("{}: Response without body", name),
                RequestError::JSONError => println!("{}: Cannot parse JSON", name),
                RequestError::PackageError(err) => println!("{}: {}", name, err),
                RequestError::APIError => println!("{}: API error", name),
                RequestError::EmptyError => println!("{}: Package data empty", name),
                RequestError::NoInfoError => println!("{}: No info", name),
            },
        };
    }

    exit(0);
}
