use std::process::Command;
use std::io;
use std::fs;

fn main() {
    match fs::metadata("build/kernel.elf") {
        Ok(meta) => println!("Size of kernel.elf: {} bytes", meta.len()),
        Err(e) => eprintln!("Error reading 'build/kernel.elf': {}", e),
    }

    // Run `readelf -h build/kernel.elf | grep "Entry point"`
    match run_command("readelf", &["-h", "build/kernel.elf"]) {
        Ok(out) => {
            if let Some(entry_point) = out.lines()
                .find(|line| line.contains("Entry point")) {
                println!("Entry point: {}", entry_point);
            } else {
                eprintln!("Entry point not found in the ELF file.");
            }
        },
        Err(e) => eprintln!("Error running 'readelf': {}", e),
    }

    // Run `nm build/kernel.elf | grep kmain`
    match run_command("nm", &["build/kernel.elf"]) {
        Ok(out) => {
            if let Some(kernel_main) = out.lines()
                .find(|line| line.contains("kmain")) {
                println!("kmain symbol: {}", kernel_main);
            } else {
                eprintln!("kmain symbol not found.");
            }
        },
        Err(e) => eprintln!("Error running 'nm': {}", e),
    }

    // Run `nm build/kernel.elf`
    match run_command("nm", &["build/kernel.elf"]) {
        Ok(out) => {
            println!("Symbols in kernel.elf:\n{}", out);
        }
        Err(e) => eprintln!("Error running 'nm': {}", e),
    }
}

fn run_command(command: &str, args: &[&str]) -> Result<String, io::Error> {
    let out = Command::new(command)
        .args(args)
        .output()?;

    if !out.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, format!("Command failed: {:?}", out)));
    }

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

