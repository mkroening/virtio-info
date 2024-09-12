use std::fmt;

use bitflags::Flags;
use virtio::le128;

use crate::bit_iter::u128Ext;

pub struct Printer<'a, T: ?Sized> {
    flags: &'a T,
}

impl<'a, T> fmt::Display for Printer<'a, T>
where
    T: Flags<Bits = le128>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in self.flags.bits().to_ne().iter_bits() {
            let name = T::from_bits_retain((1 << bit).into())
                .iter_names()
                .next()
                .map(|(name, _f)| name)
                .unwrap_or("UNKNOWN");
            writeln!(f, "{bit:>3}: {name}")?;
        }
        Ok(())
    }
}

pub trait FlagsExt {
    fn display(&self) -> Printer<'_, Self>;
}

impl<T: ?Sized> FlagsExt for T {
    fn display(&self) -> Printer<'_, Self> {
        Printer { flags: self }
    }
}
