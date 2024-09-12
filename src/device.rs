use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

use virtio::le128;

use crate::flags_display::FlagsExt;

pub struct Device {
    path: PathBuf,
    id: virtio::Id,
    flags: le128,
}

impl Device {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let id = read_device(&path.join("device"))?;
        let flags = read_features(&path.join("features"))?;
        Ok(Self { path, id, flags })
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{:?}", self.id).to_ascii_lowercase();

        macro_rules! info_impl {
            ($F:ty) => {{
                let flags = <$F>::from_bits_retain(self.flags);
                write!(
                    f,
                    "{} {name} device active feature bits:\n{}",
                    self.path.as_path().display(),
                    flags.display()
                )?;
            }};
        }

        match self.id {
            virtio::Id::Net => info_impl!(virtio::net::F),
            virtio::Id::Fs => info_impl!(virtio::fs::F),
            virtio::Id::Vsock => info_impl!(virtio::vsock::F),
            _ => info_impl!(virtio::F),
        }

        Ok(())
    }
}

fn read_device(path: &Path) -> io::Result<virtio::Id> {
    let read = fs::read_to_string(path)?;
    assert!(read.starts_with("0x"));

    let trim = read.trim_start_matches("0x").trim_end_matches('\n');
    let n = u8::from_str_radix(trim, 16).unwrap();

    Ok(n.into())
}

fn read_features(path: &Path) -> io::Result<le128> {
    let read = fs::read_to_string(path)?;

    let mut n = 0u128;
    for (i, bit) in read.bytes().enumerate() {
        match bit {
            b'1' => n |= 1 << i,
            b'0' | b'\n' => {}
            bit => panic!("unexpected bit {:?} in {}", char::from(bit), path.display()),
        }
    }

    Ok(n.into())
}
