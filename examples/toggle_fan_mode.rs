use libc;
use notify_rust::Notification;


fn is_superuser() -> bool {
    unsafe { libc::geteuid() == 0 }
}
fn main() {
    if !is_superuser() {
        Notification::new()
            .summary("Fan control")
            .body("Please run this program as root")
            .show()
            .unwrap();
        return;
    }
    println!("Hello, world!");
}
