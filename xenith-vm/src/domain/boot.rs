/*
Xenith - Xen-based security hypervisor
Copyright (C) 2025 Xenith contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Boot configuration structures and options for a domain.

use crate::XlConfiguration;

use std::fmt::Display;
use std::path::PathBuf;

/// The firmware is used to boot the domain and load the operating system.
/// Non direct kernel boot allows booting guests with a firmware. This can be used by all
/// types of guests, although the selection of options is different depending on the
/// guest type.
/// This option provides the flexibly of letting the guest decide which kernel they want
/// to boot, while preventing having to poke at the guest file system form the toolstack
/// domain.
///
/// ⚠️ Those options are only available for HVM guests.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Firmware {
    /// Boot the guest using the default BIOS firmware, which depends on the chosen
    /// device model.
    Bios,
    /// Boot the guest using the default UEFI firmware, currently OVMF.
    #[default]
    Uefi,
    /// Boot the guest using the SeaBIOS BIOS firmware.
    Seabios,
    /// Boot the guest using the ROMBIOS BIOS firmware.
    Rombios,
    /// Boot the guest using the OVMF UEFI firmware.
    Ovmf,
    /// Load the specified file as firmware for the guest.
    Path(PathBuf),
}

impl Display for Firmware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Firmware::Bios => write!(f, "bios"),
            Firmware::Uefi => write!(f, "uefi"),
            Firmware::Seabios => write!(f, "seabios"),
            Firmware::Rombios => write!(f, "rombios"),
            Firmware::Ovmf => write!(f, "ovmf"),
            Firmware::Path(path) => write!(f, "{}", path.display()),
        }
    }
}

impl XlConfiguration for Firmware {
    fn xl_config(&self) -> String {
        format!("firmware={}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_display() {
        assert_eq!(Firmware::Bios.to_string(), "bios");
        assert_eq!(Firmware::Uefi.to_string(), "uefi");
        assert_eq!(Firmware::Seabios.to_string(), "seabios");
        assert_eq!(Firmware::Rombios.to_string(), "rombios");
        assert_eq!(Firmware::Ovmf.to_string(), "ovmf");
        assert_eq!(
            Firmware::Path(PathBuf::from("/path/to/file")).to_string(),
            "/path/to/file"
        );
    }

    #[test]
    fn test_firmware_xl_config() {
        assert_eq!(Firmware::Bios.xl_config(), "firmware=bios");
        assert_eq!(Firmware::Uefi.xl_config(), "firmware=uefi");
        assert_eq!(Firmware::Seabios.xl_config(), "firmware=seabios");
        assert_eq!(Firmware::Rombios.xl_config(), "firmware=rombios");
        assert_eq!(Firmware::Ovmf.xl_config(), "firmware=ovmf");
        assert_eq!(
            Firmware::Path(PathBuf::from("/path/to/file")).xl_config(),
            "firmware=/path/to/file"
        );
    }
}
