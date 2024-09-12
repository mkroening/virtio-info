mod bit_iter;
mod device;
mod flags_display;

use std::{fs, io};

use anyhow::bail;
use clap::Parser;

use self::device::Device;

#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {}

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();

    let bus = "/sys/bus";
    if !fs::exists(bus)? {
        bail!("{bus} does not exist. Is this Linux?");
    }

    print_all_devices()?;

    Ok(())
}

fn print_all_devices() -> io::Result<()> {
    for driver in fs::read_dir("/sys/bus/virtio/drivers")? {
        let driver = driver?;

        for device in fs::read_dir(driver.path())? {
            let device = device?;

            if !device
                .file_name()
                .to_str()
                .is_some_and(|s| s.starts_with("virtio"))
            {
                continue;
            }

            let device = Device::new(device.path())?;
            println!("{device}");
        }
    }

    Ok(())
}
