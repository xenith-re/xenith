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

//! Xenith domain configuration templating

use crate::XlConfiguration;
use crate::domain::Domain;

use tera::{Context, Tera};

/// Domain configuration templating
///
/// This struct is used to generate a domain configuration file from a [`Domain`] object
#[derive(Debug, Clone, Default)]
pub struct DomainTemplate {
    tera: Tera,
    context: Context,
}

impl DomainTemplate {
    pub const DEFAULT_CONFIG_TEMPLATE: &str = "templates/default-config.cfg";

    /// Create a new [`Tera`] domain template
    ///
    /// # Arguments
    ///
    /// * `domain` - The Xenith [`Domain`] to be templated
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the [`DomainTemplate`] if successful, or a [`tera::Error`] if not
    pub fn new(domain: Domain) -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
        tera.add_template_file(DomainTemplate::DEFAULT_CONFIG_TEMPLATE, None)?;

        let mut context = Context::new();

        // Generic domain configuration
        context.insert("name", &domain.name.xl_config());
        context.insert("domain_type", &domain.r#type.xl_config());
        context.insert("memory", &domain.memory.xl_config());
        context.insert("maximum_memory", &domain.maximum_memory.xl_config());
        context.insert("nested_hvm", &domain.nested_hvm.xl_config());

        // Boot
        context.insert("firmware", &domain.firmware.xl_config());
        context.insert("boot_devices", &domain.boot_devices.xl_config());

        // Devices
        context.insert("disks", &domain.disks.xl_config());
        context.insert(
            "emulated_disk_controller",
            &domain.emulated_disk_controller.xl_config(),
        );

        // Network
        context.insert("network_interfaces", &domain.network_interfaces.xl_config());

        // Events
        context.insert("domain_actions", &domain.domain_actions.xl_config());

        // Processor
        context.insert("virtual_cpus", &domain.virtual_cpus.xl_config());
        context.insert(
            "maximum_virtual_cpus",
            &domain.maximum_virtual_cpus.xl_config(),
        );
        context.insert("alternate_p2m", &domain.alternate_p2m.xl_config());
        context.insert("smbios", &domain.smbios.xl_config());

        // Time
        context.insert("tsc_mode", &domain.tsc_mode.xl_config());

        Ok(Self { tera, context })
    }

    /// Render the domain configuration template
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the rendered domain configuration as a [`String`] if successful, or a [`tera::Error`] if not
    pub fn render(&self) -> Result<String, tera::Error> {
        self.tera
            .render(DomainTemplate::DEFAULT_CONFIG_TEMPLATE, &self.context)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use super::*;
    use crate::domain::*;

    #[test]
    fn test_domain_template() -> Result<(), tera::Error> {
        // Create a realist domain configuration
        let name = DomainName("Xenith".to_string());
        let r#type = DomainType::Hvm;
        let memory = MemoryCapacity(8000); // 8GB
        let maximum_memory = MaximumMemoryCapacity(10000); // 16GB
        let nested_hvm = NestedHvm(true);
        let firmware = Firmware::Uefi;
        let boot_devices = BootDevices(vec![
            BootDevice::HardDisk,
            BootDevice::CdRom,
            BootDevice::Network,
        ]);
        let disks = DiskDevices(vec![
            Disk {
                target: PathBuf::from("/dev/sda"),
                size: u32::MAX as u64,
                format: DiskFormat::Qcow2,
                access: DiskAccess::ReadWrite,
                virtual_device: "xvda".to_string(),
            },
            Disk {
                target: PathBuf::from("/dev/sdb"),
                size: u64::MAX,
                format: DiskFormat::Raw,
                access: DiskAccess::ReadOnly,
                virtual_device: "xvdb".to_string(),
            },
        ]);
        let emulated_disk_controller = EmulatedDiskControllerType::Ahci;
        let network_interfaces = NetworkInterfaces(vec![
            NetworkInterface {
                name: "vif0.0".to_string(),
                mac: MacAddress::from_str("00:16:3e:00:00:00").unwrap(),
                bridge: "xenbr0".to_string(),
                gateway_device: "eth0".to_string(),
                r#type: NetworkInterfaceType::IoEmu,
                model: Some(NetworkInterfaceModel::Rtl8139),
            },
            NetworkInterface {
                name: "vif0.1".to_string(),
                mac: MacAddress::from_str("00:16:3e:00:00:01").unwrap(),
                bridge: "xenbr0".to_string(),
                gateway_device: "eth0".to_string(),
                r#type: NetworkInterfaceType::IoEmu,
                model: Some(NetworkInterfaceModel::Rtl8139),
            },
        ]);
        let domain_actions = DomainActions {
            on_poweroff: EventAction::Destroy,
            on_reboot: EventAction::Restart,
            on_watchdog: EventAction::Destroy,
            on_crash: EventAction::Destroy,
            on_soft_reset: EventAction::SoftReset,
        };
        let virtual_cpus = VirtualCpuNumber(4);
        let maximum_virtual_cpus = MaximumVirtualCpuNumber(8);
        let alternate_p2m = AlternateP2mMode::Mixed;
        let smbios = SmBios {
            bios_vendor: Some("Bios Vendor".to_string()),
            bios_version: Some("1.0.0".to_string()),
            system_manufacturer: Some("System Manufacturer".to_string()),
            system_product_name: Some("System Product Name".to_string()),
            system_version: Some("1.0".to_string()),
            system_serial_number: Some("0123456789".to_string()),
            baseboard_manufacturer: Some("Baseboard".to_string()),
            baseboard_product_name: Some("Baseboard Product Name".to_string()),
            baseboard_version: Some("1.0".to_string()),
            baseboard_serial_number: Some("0123456789".to_string()),
            baseboard_asset_tag: Some("0123456789".to_string()),
            baseboard_location_in_chassis: Some("123".to_string()),
            enclosure_manufacturer: Some("Enclosure Manufacturer".to_string()),
            enclosure_serial_number: Some("0123456789".to_string()),
            enclosure_asset_tag: Some("0123456789".to_string()),
            battery_manufacturer: Some("Battery Manufacturer".to_string()),
            battery_device_name: Some("Battery Device".to_string()),
            oems: Some(vec!["Xenith".to_string(), "Xen".to_string()]),
        };
        let tsc_mode = TimeStampCounterMode::Native;

        let domain = Domain {
            name,
            r#type,
            memory,
            maximum_memory,
            nested_hvm,
            firmware,
            boot_devices,
            disks,
            emulated_disk_controller,
            network_interfaces,
            domain_actions,
            virtual_cpus,
            maximum_virtual_cpus,
            alternate_p2m,
            smbios,
            tsc_mode,
        };

        // Create a new domain template and render it
        let template = DomainTemplate::new(domain)?;
        let rendered = template.render()?;

        // Read test fixture and compare line by line, this allows easier debugging
        let expected = std::fs::read_to_string("tests/fixtures/default-config.cfg")?;
        for (i, (expected_line, rendered_line)) in
            expected.lines().zip(rendered.lines()).enumerate()
        {
            assert_eq!(expected_line, rendered_line, "Line {} does not match", i);
        }

        Ok(())
    }
}
