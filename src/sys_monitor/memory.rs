use sysinfo::{ System };

pub fn show_memory_usage() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();

    println!("Memory Usage:");
    println!("Total Memory: {:.2} MB", total_memory as f64 / 1_024.0 / 1_024.0);
    println!("Used Memory: {:.2} MB", used_memory as f64 / 1_024.0 / 1_024.0);
    println!("Available Memory: {:.2} MB", available_memory as f64 / 1_024.0 / 1_024.0);
}