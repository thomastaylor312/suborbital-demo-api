use chrono::{Duration, Local};
use serde::Deserialize;
use suborbital::runnable;

#[derive(Deserialize)]
struct Data {
    data: Cases,
}
#[derive(Deserialize)]
struct Cases {
    cases: Total,
}

#[derive(Deserialize)]
struct Total {
    total: Calculated,
}

#[derive(Deserialize)]
struct Calculated {
    calculated: Change,
}

#[derive(Deserialize)]
struct Change {
    change_from_prior_day: i64,
}

struct GetCaseData {}

impl runnable::Runnable for GetCaseData {
    fn run(&self, _input: Vec<u8>) -> Option<Vec<u8>> {
        let location = suborbital::req::state("location").to_lowercase();

        // Get yesterday (since today might not be available)
        let date = Local::now() - Duration::days(1);

        let url = format!(
            "https://api.covidtracking.com/v2beta/states/{}/{}.json",
            location,
            date.format("%Y-%m-%d").to_string()
        );

        let data: Data = serde_json::from_slice(&suborbital::http::get(&url, None)).unwrap();

        Some(
            data.data
                .cases
                .total
                .calculated
                .change_from_prior_day
                .to_string()
                .as_bytes()
                .to_vec(),
        )
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &GetCaseData = &GetCaseData {};

#[no_mangle]
pub extern "C" fn init() {
    runnable::set(RUNNABLE);
}
