use sysinfo::{ Networks };

pub fn show_network_usage() {
    let mut network = Networks::new();
    network.refresh();
    network.refresh_list();

    println!("Network Usage:");
    for (interface_name, data) in network.iter() {
        println!("Interface: {}", interface_name);
        println!("Received: {:.2} KB", data.received() as f64 / 1_024.0);
        println!("Transmitted {:.2} KB", data.transmitted() as f64 / 1_024.0);
    }
}