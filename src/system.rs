use clap::Subcommand;
use colored::Colorize;
use sysinfo::{Component, Disk, Disks, System};

#[derive(Subcommand)]
pub enum SystemCommands{
    #[clap(name = "disks", about = "View space of all installed disks.")]
    Disks
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

pub fn list_disks() {
    let disks = Disks::new_with_refreshed_list();
    
    println!("{}", "Disks:".bold().underline());
    println!("");
    for disk in disks.list() {
        let display_name = disk.name().to_string_lossy();
        let kind = disk.kind();
        let letter = disk.mount_point();
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let available_pct = if total_space > 0 {
            (available_space as f64 / total_space as f64) * 100.0
        } else {
            -1.0
        };
        let total_str = format_bytes(total_space);
        let available_str = format_bytes(available_space);

        if display_name.is_empty() {
            println!("{} [{}]", letter.display().to_string().bold().cyan(), kind);
        } else {
            println!("{} '{}' [{}]", letter.display().to_string().bold().cyan(), display_name.bold().cyan(), kind);
        }
        println!("{} {}", "- Total space:".bold(), total_str);
        println!("{} {} ({:.2}%)", "- Available space:".bold(), available_str, available_pct);

        const BAR_WIDTH: usize = 32;
        let used_pct = if total_space > 0 {
            100.0 - available_pct
        } else {
            0.0
        };
        print!("[");
        let bar_length = (used_pct / 100.0 * BAR_WIDTH as f64) as usize;
        for _ in 0..bar_length {
            print!("▰");
        }
        for _ in bar_length..BAR_WIDTH {
            print!("▱");
        }
        println!("]");
        println!("");
    }
}