use suborbital::runnable;

struct PrettyMessage {}

impl runnable::Runnable for PrettyMessage {
    fn run(&self, _input: Vec<u8>) -> Option<Vec<u8>> {
        let location = suborbital::req::state("location").to_uppercase();
        let cases = suborbital::req::state("cases");

        Some(
            format!(
                "The number of new cases (change from the previous day) in the state of {} is {}\n",
                location, cases
            )
            .as_bytes()
            .to_vec(),
        )
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &PrettyMessage = &PrettyMessage {};

#[no_mangle]
pub extern "C" fn init() {
    runnable::set(RUNNABLE);
}
