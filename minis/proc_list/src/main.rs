use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir("/proc")? {
        let entry = entry?;

        let name = entry.file_name();
        let name = name.to_string_lossy();

        if name.chars().all(|c| c.is_ascii_digit()) {
            let comm_path = format!("/proc/{}/comm", name);

            let process_name = fs::read_to_string(comm_path).unwrap_or_else(|_| "<unknown>".to_string());

            println!("PID {:>6}: {}", name, process_name.trim());
        }
    }

    Ok(())
}
