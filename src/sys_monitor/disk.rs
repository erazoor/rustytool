use sysinfo::{ Disks };

pub fn show_disk_usage() {
    let mut disks = Disks::new_with_refreshed_list();
    disks.refresh();

    for disk in &disks {
        let disk_name = disk.name().to_string_lossy();

        println!("Disk: {}", disk_name);
        println!("Total Space: {:.2} GB", disk.total_space() as f64 / 1_000_000_000.0);
        println!("Available Space: {:.2} GB", disk.available_space() as f64 / 1_000_000_000.0);
        println!("Used Space: {:.2} GB", disk.total_space() as f64 / 1_000_000_000.0 - disk.available_space() as f64 / 1_000_000_000.0);
    }
}