use soprintln::soprintln;
use std::sync::atomic::Ordering;

#[no_mangle]
pub extern "Rust" fn init() {
    soprintln::init!();
    mokio::MOKIO_TL1.with(|s| s.fetch_add(1, Ordering::Relaxed));
    mokio::MOKIO_PL1.fetch_add(1, Ordering::Relaxed);

    let dangerous = mokio::inc_dangerous();
    soprintln!("DANGEROUS is now {}", dangerous);
}
