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

//! Xenith domain configuration
//!
//! This module provides a high-level interface for managing Xenith domains.
//! All structures were created following the respective Xen configuration manuals,
//! but not exhaustively. Only the most common options are exposed to the user.
//!
//! This crate does not need to be a second `xl` tool, but it should provide a simple
//! and easy-to-use interface for managing domains on Xenith.

mod boot;
mod devices;
mod events;
mod network;
mod processor;
mod time;

pub use boot::*;
pub use devices::*;
pub use events::*;
pub use network::*;
pub use processor::*;
pub use time::*;

use crate::XlConfiguration;

use std::fmt::Display;

/// Represents the type of Xen virtual machine
///
/// ⚠️ Even though PV and PVH are supported by Xen and listed here, they are not supported by Xenith.
/// Those are kept here for future compatibility.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DomainType {
    /// Hardware Virtual Machine : This is a full virtualization technique that allows the guest
    /// operating system to run on the virtual machine without any modifications. It provides better
    /// performance and compatibility with different operating systems but requires hardware support
    /// for virtualization.
    ///
    /// **Note**: this also emulates BIOS, disk, network peripherals, etc.
    #[default]
    Hvm,
    /// Paravirtualization : This is a lightweight virtualization technique that requires the guest
    /// operating system to be modified to run on the virtual machine. It provides better performance
    /// than full virtualization but requires the guest operating system to be modified.
    Pv,
    /// Paravirtualization with Hardware support : This is a hybrid virtualization technique that
    /// combines the benefits of paravirtualization and hardware virtualization. It provides better
    /// performance and compatibility with different operating systems without requiring the guest
    /// operating system to be modified.
    Pvh,
}

impl Display for DomainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainType::Hvm => write!(f, "hvm"),
            DomainType::Pv => write!(f, "pv"),
            DomainType::Pvh => write!(f, "pvh"),
        }
    }
}

impl XlConfiguration for DomainType {
    fn xl_config(&self) -> String {
        format!("type = \"{}\"", self)
    }
}

/// Represents the type of VGA console to use for the guest
/// The VGA console is used to display the guest operating system's graphical output.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GuestConsole {
    /// Simple DirectMedia Layer (SDL) : This is a cross-platform multimedia library that provides
    /// low-level access to audio, keyboard, mouse, and display hardware. It is used to create
    /// graphical user interfaces for applications and games.
    #[default]
    Sdl,
    /// Virtual Network Computing (VNC) : This is a remote desktop protocol that allows you to view
    /// and interact with the guest operating system's graphical output over a network connection.
    Vnc,
}

impl Display for GuestConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuestConsole::Sdl => write!(f, "sdl"),
            GuestConsole::Vnc => write!(f, "vnc"),
        }
    }
}

impl XlConfiguration for GuestConsole {
    fn xl_config(&self) -> String {
        // TODO: Implement this following xl.cfg manual
        unimplemented!()
    }
}

/// Represents the number of virtual CPUs to allocate to the virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualCpuNumber(pub u8);

impl Display for VirtualCpuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vcpus = {}", self.0)
    }
}

impl XlConfiguration for VirtualCpuNumber {
    fn xl_config(&self) -> String {
        self.to_string()
    }
}

/// Represents the number of maximum virtual CPUs to allocate to the virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MaximumVirtualCpuNumber(pub u8);

impl Display for MaximumVirtualCpuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "maxvcpus = {}", self.0)
    }
}

impl XlConfiguration for MaximumVirtualCpuNumber {
    fn xl_config(&self) -> String {
        self.to_string()
    }
}

/// Represents the initial memory capacity of the virtual machine
/// This is the amount of memory that will be allocated to the virtual machine when it starts.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MemoryCapacity(pub u64);

impl Display for MemoryCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "memory = {}", self.0)
    }
}

impl XlConfiguration for MemoryCapacity {
    fn xl_config(&self) -> String {
        self.to_string()
    }
}

/// Represents the maximum memory capacity of the virtual machine
/// This is the maximum amount of memory that the virtual machine can use.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MaximumMemoryCapacity(pub u64);

impl Display for MaximumMemoryCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "maxmem = {}", self.0)
    }
}

impl XlConfiguration for MaximumMemoryCapacity {
    fn xl_config(&self) -> String {
        self.to_string()
    }
}

/// Specifies if the domain should have access to virtualization extensions
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NestedHvm(pub bool);

impl Display for NestedHvm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nestedhvm = {}", self.0 as u8)
    }
}

impl XlConfiguration for NestedHvm {
    fn xl_config(&self) -> String {
        self.to_string()
    }
}

/// Represents the name of the virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DomainName(pub String);

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name = \"{}\"", self.0)
    }
}

impl XlConfiguration for DomainName {
    fn xl_config(&self) -> String {
        self.to_string()
    }
}

/// Represents a Xen domain configuration
/// This is not a complete list of all the configuration options available for a Xen domain,
/// as Xenith does not need to expose all the options to the user. It only exposes the most
/// commonly used options for creating a virtual machine.
///
/// See `man xl.cfg` for more information.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Domain {
    /// Name of the virtual machine
    pub name: DomainName,
    /// Type of virtual machine
    pub r#type: DomainType,
    /// Number of virtual CPUs
    pub virtual_cpus: VirtualCpuNumber,
    /// Allow the guest to bring up a maximum of M vCPUs. When starting the guest, if
    /// vcpus=N is less than maxvcpus=M then the first N vCPUs will be created online and
    /// the remainder will be created offline.
    pub maximum_virtual_cpus: MaximumVirtualCpuNumber,
    /// Initial memory allocation in mega bytes
    pub memory: MemoryCapacity,
    /// Maximum memory size in mega bytes
    /// If this is greater than `memory' then the slack will start ballooned
    /// (this assumes guest kernel support for ballooning)
    pub maximum_memory: MaximumMemoryCapacity,
    /// Disk devices attached to the virtual machine
    pub disks: DiskDevices,
    /// List of network interfaces attached to the virtual machine
    pub network_interfaces: NetworkInterfaces,
    /// Actions to take when a domain event occurs
    pub domain_actions: DomainActions,
    /// The firmware to use for the virtual machine
    pub firmware: Firmware,
    /// Specifies the emulated virtual device to boot from.
    ///
    /// **Note**: multiple options can be given and will be attempted in the order they are
    /// given, e.g. to boot from CD-ROM but fall back to the hard disk you can specify it
    /// as dc.
    pub boot_devices: BootDevices,
    /// Specifies the type of emulated disk controller to use.
    pub emulated_disk_controller: EmulatedDiskControllerType,
    /// Alternative p2m (altp2m) allows external monitoring of guest memory
    /// by maintaining multiple physical to machine (p2m) memory mappings.
    /// Specifies the access mode to the alternate-p2m capability.
    /// Alternative p2m allows a guest to manage multiple physical to machine (p2m) guest
    /// physical "memory views" (as opposed to a single p2m). You may want this option if
    /// you want to access-control/isolate access to specific guest physical memory pages
    /// accessed by the guest, e.g. for domain memory introspection or for
    /// isolation/access-control of memory between components within a single guest domain.
    /// This option is disabled by default.
    pub alternate_p2m: AlternateP2mMode,
    /// Enable or disables guest access to hardware virtualisation features, e.g. it
    /// allows a guest Operating System to also function as a hypervisor. You may want
    /// this option if you want to run another hypervisor (including another copy of Xen)
    /// within a Xen guest or to support a guest Operating System which uses hardware
    /// virtualisation extensions (e.g. Windows XP compatibility mode on more modern
    /// Windows OS).
    pub nested_hvm: NestedHvm,
    /// SMBIOS information for the domain
    pub smbios: SmBios,
    /// Specifies how the TSC (Time Stamp Counter) should be provided to the
    /// guest.
    pub tsc_mode: TimeStampCounterMode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_type_display() {
        assert_eq!(DomainType::Hvm.to_string(), "hvm");
        assert_eq!(DomainType::Pv.to_string(), "pv");
        assert_eq!(DomainType::Pvh.to_string(), "pvh");
    }

    #[test]
    fn test_domain_type_xl_config() {
        assert_eq!(DomainType::Hvm.xl_config(), "type = \"hvm\"");
        assert_eq!(DomainType::Pv.xl_config(), "type = \"pv\"");
        assert_eq!(DomainType::Pvh.xl_config(), "type = \"pvh\"");
    }

    #[test]
    fn test_guest_console_display() {
        assert_eq!(GuestConsole::Sdl.to_string(), "sdl");
        assert_eq!(GuestConsole::Vnc.to_string(), "vnc");
    }

    #[test]
    #[should_panic] // TODO: Implement this
    fn test_guest_console_xl_config() {
        GuestConsole::Sdl.xl_config();
    }

    #[test]
    fn test_virtual_cpu_number_display() {
        assert_eq!(VirtualCpuNumber(1).to_string(), "vcpus = 1");
    }

    #[test]
    fn test_virtual_cpu_number_xl_config() {
        assert_eq!(VirtualCpuNumber(1).xl_config(), "vcpus = 1");
    }

    #[test]
    fn test_maximum_virtual_cpu_number_display() {
        assert_eq!(MaximumVirtualCpuNumber(1).to_string(), "maxvcpus = 1");
    }

    #[test]
    fn test_maximum_virtual_cpu_number_xl_config() {
        assert_eq!(MaximumVirtualCpuNumber(1).xl_config(), "maxvcpus = 1");
    }

    #[test]
    fn test_memory_capacity_display() {
        assert_eq!(MemoryCapacity(1024).to_string(), "memory = 1024");
    }

    #[test]
    fn test_memory_capacity_xl_config() {
        assert_eq!(MemoryCapacity(1024).xl_config(), "memory = 1024");
    }

    #[test]
    fn test_maximum_memory_capacity_display() {
        assert_eq!(MaximumMemoryCapacity(1024).to_string(), "maxmem = 1024");
    }

    #[test]
    fn test_maximum_memory_capacity_xl_config() {
        assert_eq!(MaximumMemoryCapacity(1024).xl_config(), "maxmem = 1024");
    }

    #[test]
    fn test_nested_hvm_display() {
        assert_eq!(NestedHvm(true).to_string(), "nestedhvm = 1");
        assert_eq!(NestedHvm(false).to_string(), "nestedhvm = 0");
    }

    #[test]
    fn test_nested_hvm_xl_config() {
        assert_eq!(NestedHvm(true).xl_config(), "nestedhvm = 1");
        assert_eq!(NestedHvm(false).xl_config(), "nestedhvm = 0");
    }

    #[test]
    fn test_domain_name_display() {
        assert_eq!(
            DomainName("test".to_string()).to_string(),
            "name = \"test\""
        );
    }

    #[test]
    fn test_domain_name_xl_config() {
        assert_eq!(
            DomainName("test".to_string()).xl_config(),
            "name = \"test\""
        );
    }

    #[test]
    fn test_domain_default() {
        let domain = Domain::default();
        assert_eq!(domain.r#type, DomainType::Hvm);
        assert_eq!(domain.name, DomainName::default());
        assert_eq!(domain.virtual_cpus, VirtualCpuNumber(0));
        assert_eq!(domain.maximum_virtual_cpus, MaximumVirtualCpuNumber(0));
        assert_eq!(domain.memory, MemoryCapacity(0));
        assert_eq!(domain.maximum_memory, MaximumMemoryCapacity(0));
        assert_eq!(domain.disks, DiskDevices::default());
        assert_eq!(domain.network_interfaces, NetworkInterfaces::default());
        assert_eq!(domain.domain_actions, DomainActions::default());
        assert_eq!(domain.firmware, Firmware::default());
        assert_eq!(domain.boot_devices, BootDevices::default());
        assert_eq!(
            domain.emulated_disk_controller,
            EmulatedDiskControllerType::default()
        );
        assert_eq!(domain.alternate_p2m, AlternateP2mMode::default());
        assert_eq!(domain.nested_hvm, NestedHvm::default());
        assert_eq!(domain.smbios, SmBios::default());
        assert_eq!(domain.tsc_mode, TimeStampCounterMode::default());
    }
}
