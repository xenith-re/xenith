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

use crate::{error::CpuidError, XlConfiguration};

use std::fmt::Display;

/// Represents the access mode to the alternate-p2m capability
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Alternate2pmMode {
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

impl Display for Alternate2pmMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alternate2pmMode::Disabled => write!(f, "disabled"),
            Alternate2pmMode::Mixed => write!(f, "mixed"),
            Alternate2pmMode::External => write!(f, "external"),
            Alternate2pmMode::Limited => write!(f, "limited"),
        }
    }
}

/// Represents the notation for a CPUID feature bit
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CpuidFeatureBit {
    /// Force the corresponding bit to 1
    Force1,
    /// Force the corresponding bit to 0
    Force0,
    /// Get a safe value (pass through and mask with the default policy)
    SafeValue,
    /// pass through the host bit value (at boot only - value preserved on
    /// migrate)
    #[default]
    Passthrough,
}

impl Display for CpuidFeatureBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuidFeatureBit::Force1 => write!(f, "1"),
            CpuidFeatureBit::Force0 => write!(f, "0"),
            CpuidFeatureBit::SafeValue => write!(f, "x"),
            CpuidFeatureBit::Passthrough => write!(f, "k"),
        }
    }
}

/// The CPUID configuration for a domain
///
/// This employs the xend format, which consists of an array of one or more strings of the form
/// "leaf:reg=bitstring,...".
///
/// List of keys taking a character can be found in the public header file:
/// `xen/include/public/arch-x86/cpufeatureset.h`
///
/// This does not implement every possible key, only the most useful ones for
/// Xenith, mainly for evading VM detection.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Cpuid {
    /// The CPUID feature bit for the hypervisor
    pub hypervisor: CpuidFeatureBit,
    /// The vendor info is a 12-byte (96 bit) long string, which is used to
    /// identify the vendor of the CPU. This is used by some software to
    /// determine the CPU vendor, and can be used to detect if the CPU is
    /// running in a virtual machine.
    pub vendor: [u8; 12],
    /// Processor Brand String is a 48-byte (384 bit) long string, which is
    /// used to identify the brand of the CPU. This is used by some software
    /// to determine the CPU brand, and can be used to detect if the CPU is
    /// running in a virtual machine.
    ///
    /// See https://en.wikipedia.org/wiki/CPUID#EAX=8000'0002h,8000'0003h,8000'0004h:_Processor_Brand_String
    pub processor_brand_string: [u8; 48],
    /// The hypervisor brand is a 12-byte (96 bit) long string, which is used
    /// to identify the brand of the hypervisor. This is used by some software
    /// to determine the hypervisor brand, and can be used to detect if the CPU
    /// is running in a virtual machine.
    ///
    /// See https://en.wikipedia.org/wiki/CPUID#EAX=4000'0000h-4FFFF'FFFh:_Reserved_for_Hypervisors
    pub hypervisor_brand: [u8; 12],
}

impl Default for Cpuid {
    fn default() -> Self {
        Self {
            hypervisor: CpuidFeatureBit::default(),
            vendor: [0; 12],
            processor_brand_string: [0; 48],
            hypervisor_brand: [0; 12],
        }
    }
}

impl Cpuid {
    /// Create a new *hidden* CPUID configuration with host values.
    ///
    /// This is used to hide the fact that the CPU is running in a virtual machine.
    /// It sets the hypervisor feature bit to 0, and sets the vendor, processor brand string,
    /// and hypervisor brand to the host values.
    pub fn new_hidden() -> Result<Self, CpuidError> {
        let host_cpuid = raw_cpuid::CpuId::new();

        let vendor_info = host_cpuid.get_vendor_info().ok_or(CpuidError::VendorInfo)?;
        let vendor = vendor_info
            .as_str()
            .as_bytes()
            .try_into()
            .map_err(|e| CpuidError::ConversionError(format!("Vendor info: {e}")))?;

        let processor_brand_string = host_cpuid
            .get_processor_brand_string()
            .ok_or(CpuidError::ProcessorBrandString)?;
        let processor_brand = processor_brand_string
            .as_str()
            .as_bytes()
            .try_into()
            .map_err(|e| CpuidError::ConversionError(format!("Processor brand string: {e}")))?;

        // Because there is no hypervisor 😉
        let hypervisor_brand = [0u8; 12];

        Ok(Self {
            hypervisor: CpuidFeatureBit::Force0,
            vendor,
            processor_brand_string: processor_brand,
            hypervisor_brand,
        })
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
        let smbios_str = self.to_string().replace(", ", "\", \"");
        format!("smbios=[ {} ]", smbios_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternate2pm_mode_display() {
        assert_eq!(Alternate2pmMode::Disabled.to_string(), "disabled");
        assert_eq!(Alternate2pmMode::Mixed.to_string(), "mixed");
        assert_eq!(Alternate2pmMode::External.to_string(), "external");
        assert_eq!(Alternate2pmMode::Limited.to_string(), "limited");
    }

    #[test]
    fn test_cpuid_feature_bit_display() {
        assert_eq!(CpuidFeatureBit::Force1.to_string(), "1");
        assert_eq!(CpuidFeatureBit::Force0.to_string(), "0");
        assert_eq!(CpuidFeatureBit::SafeValue.to_string(), "x");
        assert_eq!(CpuidFeatureBit::Passthrough.to_string(), "k");
    }

    // #[test]
    // fn test_cpuid_display() {
    //     let cpuid = Cpuid {
    //         hypervisor: CpuidFeatureBit::Force0,
    //         vendor: [0; 12],
    //         processor_brand_string: [0; 48],
    //         hypervisor_brand: [0; 12],
    //     };

    //     assert_eq!(
    //         cpuid.to_string(),
    //         "hypervisor=0, vendor=, processor_brand_string=, hypervisor_brand="
    //     );
    // }

    #[test]
    fn test_cpuid_new_hidden() -> Result<(), CpuidError> {
        let cpuid = Cpuid::new_hidden()?;

        assert_eq!(cpuid.hypervisor, CpuidFeatureBit::Force0);
        assert_eq!(cpuid.vendor, [0; 12]);
        assert_eq!(cpuid.processor_brand_string, [0; 48]);
        assert_eq!(cpuid.hypervisor_brand, [0; 12]);

        Ok(())
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
}
