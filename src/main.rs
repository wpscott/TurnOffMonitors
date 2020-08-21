#[cfg(windows)]
extern crate winapi;

const WM_SYSCOMMAND: u32 = 0x0112;
const SC_MONITORPOWER: usize = 0xF170;

enum Options {
    ON = -1,
    LOW = 1,
    OFF = 2,
}

fn main() {
    use winapi::um::winuser::{PostMessageA, HWND_BROADCAST};
    unsafe {
        PostMessageA(
            HWND_BROADCAST,
            WM_SYSCOMMAND,
            SC_MONITORPOWER,
            Options::OFF as isize,
        )
    };
}
