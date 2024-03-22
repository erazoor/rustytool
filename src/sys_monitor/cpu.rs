use sysinfo::{ System };

pub fn show_cpu_usage() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("CPU Usage: ");
    println!("{}%", sys.global_cpu_info().cpu_usage());
}