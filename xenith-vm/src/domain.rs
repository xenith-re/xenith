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
    HVM,
    /// Paravirtualization : This is a lightweight virtualization technique that requires the guest
    /// operating system to be modified to run on the virtual machine. It provides better performance
    /// than full virtualization but requires the guest operating system to be modified.
    PV,
    /// Paravirtualization with Hardware support : This is a hybrid virtualization technique that
    /// combines the benefits of paravirtualization and hardware virtualization. It provides better
    /// performance and compatibility with different operating systems without requiring the guest
    /// operating system to be modified.
    PVH,
}

impl Display for DomainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainType::HVM => write!(f, "hvm"),
            DomainType::PV => write!(f, "pv"),
            DomainType::PVH => write!(f, "pvh"),
        }
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

/// Represents a Xen domain configuration
/// This is not a complete list of all the configuration options available for a Xen domain,
/// as Xenith does not need to expose all the options to the user. It only exposes the most
/// commonly used options for creating a virtual machine.
///
/// See `man xl.cfg` for more information.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Domain {
    /// Type of virtual machine
    r#type: DomainType,
    /// Name of the virtual machine
    name: String,
    /// Number of virtual CPUs
    virtual_cpus: u8,
    /// Allow the guest to bring up a maximum of M vCPUs. When starting the guest, if
    /// vcpus=N is less than maxvcpus=M then the first N vCPUs will be created online and
    /// the remainder will be created offline.
    maximum_virtual_cpus: u8,
    /// Initial memory allocation in mega bytes
    memory: u64,
    /// Maximum memory size in mega bytes
    /// If this is greater than `memory' then the slack will start ballooned
    /// (this assumes guest kernel support for ballooning)
    maximum_memory: u64,
    /// Disk devices attached to the virtual machine
    disks: DiskDevices,
    /// List of network interfaces attached to the virtual machine
    network_interfaces: NetworkInterfaces,
    /// Actions to take when a domain event occurs
    domain_actions: DomainActions,
    /// The firmware to use for the virtual machine
    firmware: Firmware,
    /// Specifies the emulated virtual device to boot from.
    ///
    /// **Note**: multiple options can be given and will be attempted in the order they are
    /// given, e.g. to boot from CD-ROM but fall back to the hard disk you can specify it
    /// as dc.
    boot_device: BootDevices,
    /// Specifies the type of emulated disk controller to use.
    emulated_disk_controller: EmulatedDiskControllerType,
    /// Alternative p2m (altp2m) allows external monitoring of guest memory
    /// by maintaining multiple physical to machine (p2m) memory mappings.
    /// Specifies the access mode to the alternate-p2m capability.
    /// Alternative p2m allows a guest to manage multiple physical to machine (p2m) guest
    /// physical "memory views" (as opposed to a single p2m). You may want this option if
    /// you want to access-control/isolate access to specific guest physical memory pages
    /// accessed by the guest, e.g. for domain memory introspection or for
    /// isolation/access-control of memory between components within a single guest domain.
    /// This option is disabled by default.
    alternate_2pm: Alternate2pmMode,
    /// Enable or disables guest access to hardware virtualisation features, e.g. it
    /// allows a guest Operating System to also function as a hypervisor. You may want
    /// this option if you want to run another hypervisor (including another copy of Xen)
    /// within a Xen guest or to support a guest Operating System which uses hardware
    /// virtualisation extensions (e.g. Windows XP compatibility mode on more modern
    /// Windows OS).
    nested_hvm: bool,
    /// Configure the value returned when a guest executes the CPUID instruction.
    cpuid: Cpuid,
    /// SMBIOS information for the domain
    smbios: SmBios,
    /// Specifies how the TSC (Time Stamp Counter) should be provided to the
    /// guest.
    tsc_mode: TimeStampCounterMode,
}
