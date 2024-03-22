mod sys_monitor;
mod cli;

use clap::{App, SubCommand};
use crate::sys_monitor::{show_all_stats, show_cpu_usage, show_disk_usage, show_memory_usage, show_network_usage, show_live_stats};

fn main() {
    let matches = App::new("rustytool")
        .version("0.1.0")
        .author("@erazoor")
        .about("Let you monitor your hardware using a simple cli")
        .subcommand(SubCommand::with_name("sys")
            .about("Monitor system stats")
            .subcommand(SubCommand::with_name("cpu").about("Shows CPU usage"))
            .subcommand(SubCommand::with_name("mem").about("Shows Memory usage"))
            .subcommand(SubCommand::with_name("disk").about("Shows Disk usage"))
            .subcommand(SubCommand::with_name("net").about("Shows Network usage"))
            .subcommand(SubCommand::with_name("all").about("Shows all stats"))
            .subcommand(SubCommand::with_name("live").about("Shows live stats"))
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("sys") {
        match matches.subcommand_name() {
            Some("cpu") => show_cpu_usage(),
            Some("mem") => show_memory_usage(),
            Some("disk") => show_disk_usage(),
            Some("net") => show_network_usage(),
            Some("all") => show_all_stats(),
            Some("live") => show_live_stats().unwrap(),
            _ => println!("Invalid subcommand. Use --help to see available subcommands"),
        }
    }
}
