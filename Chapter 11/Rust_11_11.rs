use rusb::{Context, Device, UsbContext};

fn print_device_info(device: &Device<Context>) -> rusb::Result<()> {
    let device_desc = device.device_descriptor()?;
    let product_string = device_desc.product_string_index(); //.unwrap_or("Unknown".to_string());

    println!("Device: {:?}", product_string);
    println!("Vendor ID: {:04x}", device_desc.vendor_id());
    println!("Product ID: {:04x}", device_desc.product_id());
    println!("Bus Number: {}", device.bus_number());
    println!("Device Address: {}", device.address());

    println!("----------------------------------");

    Ok(())
}

fn main() {
    let context = Context::new().expect("Error initializing USB context");

    let devices = context.devices().expect("Error getting device list");

    if devices.is_empty() {
        println!("No USB devices found.");
        return;
    }

    println!("Connected USB Devices:");

    for device in devices.iter() {
        if let Err(err) = print_device_info(&device) {
            eprintln!("Error: {}", err);
        }
    }
}
