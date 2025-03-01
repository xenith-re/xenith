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

//! Processor and platforms configuration structures and options for a domain.
//!
//! <div class="info">
//!
//! You could wonder why we don't directly specify CPUID values in the domain configuration as
//! xl.cfg allows to do. The reason is that xl, when specifying cpuid for hypervisor leaves
//! (0x4000xxxx major group) only the lowest 8 bits of leaf's 0x4000xx00 EAX register are processed,
//! the rest are ignored (these 8 bits signify maximum number of hypervisor leaves).
//!
//! This is an important limitation, as we need to hook the hypervisor leaves to hide the fact that
//! the CPU is running in a virtual machine. To do this, we need to specify the full leaf value.
//!
//! If this changes, we could add it back with the `Xend` format, as the `Libxl` format does not allow
//! to specify the full leaf value.
//!
//! </div>

use crate::XlConfiguration;

use std::fmt::Display;

/// Represents the access mode to the alternate-p2m capability
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AlternateP2mMode {
    /// Altp2m is disabled for the domain
    #[default]
    Disabled,
    /// The mixed mode allows access to the altp2m interface for both in-guest and
    /// external tools as well.
    Mixed,
    /// Enables access to the alternate-p2m capability by external privileged tools.
    External,
    /// Enables limited access to the alternate-p2m capability, ie. giving the guest
    /// access only to enable/disable the VMFUNC and #VE features.
    Limited,
}

impl Display for AlternateP2mMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlternateP2mMode::Disabled => write!(f, "disabled"),
            AlternateP2mMode::Mixed => write!(f, "mixed"),
            AlternateP2mMode::External => write!(f, "external"),
            AlternateP2mMode::Limited => write!(f, "limited"),
        }
    }
}

impl XlConfiguration for AlternateP2mMode {
    // altp2m="ALTP2M_MODE"
    fn xl_config(&self) -> String {
        format!("altp2m = \"{}\"", self)
    }
}

/// Represents the SMBIOS information for a domain
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SmBios {
    pub bios_vendor: Option<String>,
    pub bios_version: Option<String>,
    pub system_manufacturer: Option<String>,
    pub system_product_name: Option<String>,
    pub system_version: Option<String>,
    pub system_serial_number: Option<String>,
    pub baseboard_manufacturer: Option<String>,
    pub baseboard_product_name: Option<String>,
    pub baseboard_version: Option<String>,
    pub baseboard_serial_number: Option<String>,
    pub baseboard_asset_tag: Option<String>,
    pub baseboard_location_in_chassis: Option<String>,
    pub enclosure_manufacturer: Option<String>,
    pub enclosure_serial_number: Option<String>,
    pub enclosure_asset_tag: Option<String>,
    pub battery_manufacturer: Option<String>,
    pub battery_device_name: Option<String>,
    pub oems: Option<Vec<String>>,
}

impl Display for SmBios {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = [
            ("bios_vendor", &self.bios_vendor),
            ("bios_version", &self.bios_version),
            ("system_manufacturer", &self.system_manufacturer),
            ("system_product_name", &self.system_product_name),
            ("system_version", &self.system_version),
            ("system_serial_number", &self.system_serial_number),
            ("baseboard_manufacturer", &self.baseboard_manufacturer),
            ("baseboard_product_name", &self.baseboard_product_name),
            ("baseboard_version", &self.baseboard_version),
            ("baseboard_serial_number", &self.baseboard_serial_number),
            ("baseboard_asset_tag", &self.baseboard_asset_tag),
            (
                "baseboard_location_in_chassis",
                &self.baseboard_location_in_chassis,
            ),
            ("enclosure_manufacturer", &self.enclosure_manufacturer),
            ("enclosure_serial_number", &self.enclosure_serial_number),
            ("enclosure_asset_tag", &self.enclosure_asset_tag),
            ("battery_manufacturer", &self.battery_manufacturer),
            ("battery_device_name", &self.battery_device_name),
        ];

        // oem is a special case, as it is a list of "oem=value" pairs
        // for example, if oems = ["Xenith", "Xenith VM"], then the string
        // representation should be "oem=Xenith, oem=Xenith VM"
        let mut oems_str = String::new();
        for oem in self.oems.iter().flatten() {
            oems_str.push_str(&format!("oem={}, ", oem));
        }
        oems_str.pop();
        oems_str.pop();

        let mut smbios_str = fields
            .iter()
            .filter_map(|(name, value)| value.as_deref().map(|v| format!("{}={}", name, v)))
            .collect::<Vec<String>>();

        smbios_str.push(oems_str);

        if smbios_str.is_empty() {
            write!(f, "")
        } else {
            write!(f, "{}", smbios_str.join(", "))
        }
    }
}

impl XlConfiguration for SmBios {
    // smbios=[ "SMBIOS_SPEC_STRING", "SMBIOS_SPEC_STRING", ...]
    fn xl_config(&self) -> String {
        // add quotes around each smbios spec string
        let mut smbios_str = self.to_string().replace(", ", "\", \"");
        smbios_str.insert(0, '"');
        smbios_str.push('"');
        format!("smbios = [ {} ]", smbios_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternatep2m_mode_display() {
        assert_eq!(AlternateP2mMode::Disabled.to_string(), "disabled");
        assert_eq!(AlternateP2mMode::Mixed.to_string(), "mixed");
        assert_eq!(AlternateP2mMode::External.to_string(), "external");
        assert_eq!(AlternateP2mMode::Limited.to_string(), "limited");
    }

    #[test]
    fn test_alternatep2m_mode_xl_config() {
        assert_eq!(
            AlternateP2mMode::Disabled.xl_config(),
            "altp2m = \"disabled\""
        );
        assert_eq!(AlternateP2mMode::Mixed.xl_config(), "altp2m = \"mixed\"");
        assert_eq!(
            AlternateP2mMode::External.xl_config(),
            "altp2m = \"external\""
        );
        assert_eq!(
            AlternateP2mMode::Limited.xl_config(),
            "altp2m = \"limited\""
        );
    }

    #[test]
    fn test_smbios_display() {
        let smbios = SmBios {
            bios_vendor: Some("Xenith".to_string()),
            bios_version: Some("1.0".to_string()),
            system_manufacturer: Some("Xenith".to_string()),
            system_product_name: Some("Xenith VM".to_string()),
            system_version: Some("1.0".to_string()),
            system_serial_number: Some("123".to_string()),
            baseboard_manufacturer: Some("Xenith".to_string()),
            baseboard_product_name: Some("Xenith VM".to_string()),
            baseboard_version: Some("1.0".to_string()),
            baseboard_serial_number: Some("123".to_string()),
            baseboard_asset_tag: Some("123".to_string()),
            baseboard_location_in_chassis: Some("123".to_string()),
            enclosure_manufacturer: Some("Xenith".to_string()),
            enclosure_serial_number: Some("123".to_string()),
            enclosure_asset_tag: Some("123".to_string()),
            battery_manufacturer: Some("Xenith".to_string()),
            battery_device_name: Some("Xenith VM".to_string()),
            oems: Some(vec!["Xenith".to_string(), "Xenith VM".to_string()]),
        };

        assert_eq!(
            smbios.to_string(),
            "bios_vendor=Xenith, bios_version=1.0, system_manufacturer=Xenith, system_product_name=Xenith VM, system_version=1.0, system_serial_number=123, baseboard_manufacturer=Xenith, baseboard_product_name=Xenith VM, baseboard_version=1.0, baseboard_serial_number=123, baseboard_asset_tag=123, baseboard_location_in_chassis=123, enclosure_manufacturer=Xenith, enclosure_serial_number=123, enclosure_asset_tag=123, battery_manufacturer=Xenith, battery_device_name=Xenith VM, oem=Xenith, oem=Xenith VM"
        );
    }

    #[test]
    fn test_smbios_xl_config() {
        let smbios = SmBios {
            bios_vendor: Some("Xenith".to_string()),
            bios_version: Some("1.0".to_string()),
            system_manufacturer: Some("Xenith".to_string()),
            system_product_name: Some("Xenith VM".to_string()),
            system_version: Some("1.0".to_string()),
            system_serial_number: Some("123".to_string()),
            baseboard_manufacturer: Some("Xenith".to_string()),
            baseboard_product_name: Some("Xenith VM".to_string()),
            baseboard_version: Some("1.0".to_string()),
            baseboard_serial_number: Some("123".to_string()),
            baseboard_asset_tag: Some("123".to_string()),
            baseboard_location_in_chassis: Some("123".to_string()),
            enclosure_manufacturer: Some("Xenith".to_string()),
            enclosure_serial_number: Some("123".to_string()),
            enclosure_asset_tag: Some("123".to_string()),
            battery_manufacturer: Some("Xenith".to_string()),
            battery_device_name: Some("Xenith VM".to_string()),
            oems: Some(vec!["Xenith".to_string(), "Xenith VM".to_string()]),
        };

        assert_eq!(
            smbios.xl_config(),
            "smbios = [ \"bios_vendor=Xenith\", \"bios_version=1.0\", \"system_manufacturer=Xenith\", \"system_product_name=Xenith VM\", \"system_version=1.0\", \"system_serial_number=123\", \"baseboard_manufacturer=Xenith\", \"baseboard_product_name=Xenith VM\", \"baseboard_version=1.0\", \"baseboard_serial_number=123\", \"baseboard_asset_tag=123\", \"baseboard_location_in_chassis=123\", \"enclosure_manufacturer=Xenith\", \"enclosure_serial_number=123\", \"enclosure_asset_tag=123\", \"battery_manufacturer=Xenith\", \"battery_device_name=Xenith VM\", \"oem=Xenith\", \"oem=Xenith VM\" ]"
        );
    }
}
