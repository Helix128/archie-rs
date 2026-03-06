use clap::Subcommand;
use colored::Colorize;
use sysinfo::{Component, Disk, Disks, System};

use crate::ui;

#[derive(Subcommand)]
pub enum SystemCommands {
    #[clap(name = "partitions", about = "View information about all mounted partitions.")]
    Partitions,
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

pub fn list_partitions() {
    let disks = Disks::new_with_refreshed_list();

    println!("{}", "Partitions:".bold().underline());
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
            println!(
                "{} '{}' [{}]",
                letter.display().to_string().bold().cyan(),
                display_name.bold().cyan(),
                kind
            );
        }
        println!("{} {}", "- Total space:".bold(), total_str);
        let color = if available_pct >= 50.0 {
            colored::Color::White
        } else if available_pct >= 25.0 {
            colored::Color::Yellow
        } else {
            colored::Color::Red
        };
        println!(
            "{} {} ({:.2}%)",
            "- Available space:".bold(),
            available_str.color(color),
            available_pct.to_string()
        );

        ui::fill_bar(available_pct,32, colored::Color::White, colored::Color::Cyan);
    }
}
