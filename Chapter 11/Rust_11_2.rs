use std::fs::File;
use std::io::{Result, Write};

fn write_to_device(data: &[u8]) -> Result<()> {
    let mut file = File::create("/dev/lp0")?; // Replace 'lp0' with the appropriate device file

    file.write_all(data)?; // Write data to the device

    Ok(())
}
