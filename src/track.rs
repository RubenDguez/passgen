use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn write(pass: &str) -> io::Result<()> {
    const FILE_PATH: &str = "/Applications/utils/passgen/pass.log";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("")
        .as_millis();
    let content = String::from(format!("[{:?}]: [:: {} ::]\n", timestamp, pass));
    file.write_all(content.as_bytes())?;

    Ok(())
}
