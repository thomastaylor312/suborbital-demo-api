use suborbital::runnable;

struct ParseLocation {}

impl runnable::Runnable for ParseLocation {
    fn run(&self, _input: Vec<u8>) -> Option<Vec<u8>> {
        let state_code = String::from_utf8(suborbital::req::body_raw()).unwrap();

        if state_code.contains(' ') {
            suborbital::log::error("Invalid state code given");
            return None;
        }

        Some(state_code.as_bytes().to_vec())
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &ParseLocation = &ParseLocation {};

#[no_mangle]
pub extern "C" fn init() {
    runnable::set(RUNNABLE);
}
