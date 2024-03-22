use std::process::exit;

mod cpu;
mod memory;
mod disk;
mod process;
mod network;
mod live_stats;

pub use self::cpu::show_cpu_usage;
pub use self::memory::show_memory_usage;
pub use self::disk::show_disk_usage;
pub use self::network::show_network_usage;

pub use self::live_stats::show_live_stats;

pub fn show_all_stats() {
    exit(0);
}
