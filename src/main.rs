use winapi;
use winapi::um::winuser::{GetDC, GetDesktopWindow};
use winapi::shared::windef::HWND;

struct Payloads;
impl Payloads {
    fn random_pat(&self) {
        let hwnd: HWND = unsafe { GetDesktopWindow() };
    }
}

fn main() {
    let payloads = Payloads;

    payloads.random_pat();
}