use std::fs;

// Rustでは最後の式を自動でreturnする
fn read_trimmed(path: &str) -> String {
    fs::read_to_string(path)
    .unwrap_or_else(|_| "<unknown>".to_string())
    .trim()
    .to_string()
}

fn read_cmdline(path: &str) -> String {
    let bytes = fs::read(path).unwrap_or_default();

    if bytes.is_empty() {
        return "<empty>".to_string();
    }

    let parts: Vec<String> = bytes
        .split(|b| *b == 0)
        .filter(|part| !part.is_empty())
        .map(|part| String::from_utf8_lossy(part).to_string())
        .collect();

    if parts.is_empty() {
        "<empty>".to_string()
    } else {
        parts.join(" ")
    }
}


fn main() -> std::io::Result<()> {
    for entry in fs::read_dir("/proc")? {
        let entry = entry?;
        let name = entry.file_name(); // name: OsString
        let pid = name.to_string_lossy(); // pid: string
        
        if !pid.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        let comm = read_trimmed(&format!("/proc/{}/comm", pid));
        let cmdline = read_cmdline(&format!("/proc/{}/cmdline", pid));

        println!("PID {:>6} | {:<20} | {}", pid, comm, cmdline);
    }

    Ok(())
}
