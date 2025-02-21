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

use crate::error::CpuidError;

use std::{fmt::Display, path::PathBuf};

use mac_address::MacAddress;

/// List of supported disk formats
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DiskFormat {
    /// This is a simple, unstructured format that provides direct access to the disk image.
    /// It is straightforward and offers good performance but lacks advanced features like snapshots.
    Raw,
    /// This is an updated version of the raw format that supports features like snapshots, encryption,
    /// and compression.
    Qcow,
    /// This is a popular disk image format that supports features like snapshots, compression, and encryption.
    /// It is widely used due to its flexibility and efficiency.
    #[default]
    Qcow2,
    /// This format is commonly used in Microsoft environments and is supported by Xen for compatibility with
    /// other hypervisors like Hyper-V.
    Vhd,
    /// QED was an attempt at creating a better performing image format by removing some features compared to qcow2.
    /// However, it turned out that the achieved performance improvements were mostly related to an improved
    /// implementation rather than the file format per se.
    ///
    /// ⚠️ QED is deprecated and only supported for compatibility with existing images (similar to qcow1).
    Qed,
}

impl Display for DiskFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskFormat::Raw => write!(f, "raw"),
            DiskFormat::Qcow => write!(f, "qcow"),
            DiskFormat::Qcow2 => write!(f, "qcow2"),
            DiskFormat::Vhd => write!(f, "vhd"),
            DiskFormat::Qed => write!(f, "qed"),
        }
    }
}

/// Access control information for a disk
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DiskAccess {
    ReadOnly,
    #[default]
    ReadWrite,
}

impl Display for DiskAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskAccess::ReadOnly => write!(f, "ro"),
            DiskAccess::ReadWrite => write!(f, "rw"),
        }
    }
}

/// Represents a disk attached to a virtual machine
/// The disk can be used for storing the operating system, data, or other files.
/// It can be attached to the virtual machine as a boot disk or a data disk, which
/// can be formatted in different formats like raw, qcow2, vhd, or vmdk.
/// It can be accessed in read-only or read-write mode and attached to a specific
/// device like `xvda` or `sda`.
///
/// See `man xl-disk-configuration` for more information.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Disk {
    /// Block device or image file path.  When this is used as a path, /dev will be
    /// prepended if the path doesn't start with a '/'.
    target: PathBuf,
    /// Size of the disk in bytes.  This is required for file-based disk images.
    size: u64,
    /// Specifies the format of image file. See [`DiskFormat`] for more information.
    format: DiskFormat,
    /// Specified access control information. Whether or not the block device is
    /// provided to the guest in read-only or read-write mode depends on this
    /// attribute.
    access: DiskAccess,
    /// Virtual device as seen by the guest (also referred to as guest drive
    /// designation in some specifications).  See xen-vbd-interface(7).
    virtual_device: String,
}

impl Display for Disk {
    /// Display the disk information in the Xen disk configuration format.
    /// Size is not displayed as it is not required, it is only used for
    /// showing the user.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "format={}, vdev={}, access={}, target={}",
            self.format,
            self.virtual_device,
            self.access,
            self.target.display()
        )
    }
}

/// Represents the type of network interface attached to a virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NetworkInterfaceType {
    /// Device will be provided as an emulate device to the
    /// guest and also as a paravirtualised device which the guest may choose to use
    /// instead if it has suitable drivers available.
    #[default]
    IoEmu,
    /// Device will be provided as a paravirtualised device only.
    Vif,
}

impl Display for NetworkInterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkInterfaceType::IoEmu => write!(f, "ioemu"),
            NetworkInterfaceType::Vif => write!(f, "vif"),
        }
    }
}

/// Represents the model of network interface to use
/// This is only available for HVM guests.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NetworkInterfaceModel {
    /// Realtek RTL8139
    #[default]
    Rtl8139,
    /// Intel E1000
    E1000,
    /// Any device supported by device model
    AnySupported(String),
}

impl Display for NetworkInterfaceModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkInterfaceModel::Rtl8139 => write!(f, "rtl8139"),
            NetworkInterfaceModel::E1000 => write!(f, "e1000"),
            NetworkInterfaceModel::AnySupported(model) => write!(f, "{}", model),
        }
    }
}

/// Represents a network interface attached to a domain.
///
/// The network interface can be attached to a specific bridge, have a specific MAC address,
/// and use a specific network interface model. It can also be attached to a specific gateway
/// device.
///
/// See `man xl-network-configuration` for more information.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NetworkInterface {
    /// Specifies the backend device name for the virtual device.
    /// If the domain is an HVM domain then the associated emulated (tap) device will have a
    /// "-emu" suffice added.
    ///
    /// The default name for the virtual device is "vifDOMID.DEVID" where "DOMID" is the
    /// guest domain ID and "DEVID" is the device number. Likewise the default tap name is
    /// "vifDOMID.DEVID-emu".
    name: String,
    /// If specified then this option specifies the MAC address inside the guest of this VIF
    /// device. The value is a 48-bit number represented as six groups of two hexadecimal
    /// digits, separated by colons (:).
    ///
    /// The default if this keyword is not specified is to be automatically generate a MAC
    /// address inside the space assigned to Xen's Organizationally Unique Identifier
    /// <https://en.wikipedia.org/wiki/Organizationally_Unique_Identifier> (00:16:3e).
    /// If you are choosing a MAC address then it is strongly recommend to follow one of the
    /// following strategies:
    ///
    /// •   Generate a random sequence of 6 byte, set the locally administered bit (bit 2 of
    ///     the first byte) and clear the multicast bit (bit 1 of the first byte). In other
    ///     words the first byte should have the bit pattern xxxxxx10 (where x is a randomly
    ///     generated bit) and the remaining 5 bytes are randomly generated See
    ///     [https://en.wikipedia.org/wiki/MAC_address] for more details the structure of a
    ///     MAC address.
    ///
    /// •   Allocate an address from within the space defined by your organization's OUI (if
    ///     you have one) following your organization's procedures for doing so.
    ///
    /// •   Allocate an address from within the space defined by Xen's OUI (00:16:3e). Taking
    ///     care not to clash with other users of the physical network segment where this VIF
    ///     will reside.
    ///
    /// If you have an OUI for your own use then that is the preferred strategy. Otherwise in
    /// general you should prefer to generate a random MAC and set the locally administered
    /// bit since this allows for more bits of randomness than using the Xen OUI.
    mac: MacAddress,
    /// Specifies the name of the network bridge which this VIF should be added to. The
    /// default is "xenbr0". The bridge must be configured using your distribution's network
    /// configuration tools. See the [wiki](https://wiki.xenproject.org/wiki/Network_Configuration_Examples_(Xen_4.1%2B)) for
    /// guidance and examples.
    bridge: String,
    /// Specifies the name of the network interface which has an IP and which is in the
    /// network the VIF should communicate with. This is used in the host by the vif-route
    /// hotplug script. See [wiki](https://wiki.xenproject.org/wiki/Vif-route) for guidance
    /// and examples.
    gateway_device: String,
    /// The type of network interface to use.
    /// ⚠️ Only available for HVM guests.
    r#type: NetworkInterfaceType,
    /// The model of network interface to use.
    /// Only valid if `type` is `IoEmu`.
    /// ⚠️ Only available for HVM guests.
    model: Option<NetworkInterfaceModel>,
}

impl Default for NetworkInterface {
    fn default() -> Self {
        Self {
            name: String::default(),
            mac: MacAddress::default(),
            bridge: "xenbr0".to_string(),
            gateway_device: String::default(),
            r#type: NetworkInterfaceType::default(),
            model: Some(NetworkInterfaceModel::Rtl8139),
        }
    }
}

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

/// Represents the action to take when a domain event occurs
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EventAction {
    /// Destroy the domain
    #[default]
    Destroy,
    /// Destroy the domain and immediately create a new domain with the same
    /// configuration
    Restart,
    /// Rename the domain which terminated, and then immediately create a new domain
    /// with the same configuration as the original
    RenameRestart,
    /// Keep the domain. It can be examined, and later destroyed.
    Preserve,
    /// Write a "coredump" of the domain to `/var/lib/xen/dump/NAME` and then destroy
    /// the domain.
    CoreDumpDestroy,
    /// Write a "coredump" of the domain to `/var/lib/xen/dump/NAME` and then restart
    /// the domain.
    CoreDumpRestart,
    /// Reset all Xen specific interfaces for the Xen-aware HVM domain allowing it to
    /// reestablish these interfaces and continue executing the domain. PV and non-
    /// Xen-aware HVM guests are not supported.
    SoftReset,
}

impl Display for EventAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventAction::Destroy => write!(f, "destroy"),
            EventAction::Restart => write!(f, "restart"),
            EventAction::RenameRestart => write!(f, "rename-restart"),
            EventAction::Preserve => write!(f, "preserve"),
            EventAction::CoreDumpDestroy => write!(f, "coredump-destroy"),
            EventAction::CoreDumpRestart => write!(f, "coredump-restart"),
            EventAction::SoftReset => write!(f, "soft-reset"),
        }
    }
}

/// Represents the actions to take when a domain event occurs
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DomainActions {
    /// Specifies what should be done with the domain if it shuts itself down.
    pub on_poweroff: EventAction,
    /// Action to take if the domain shuts down with a reason code requesting a reboot.
    pub on_reboot: EventAction,
    /// Action to take if the domain shuts down due to a Xen watchdog timeout.
    pub on_watchdog: EventAction,
    /// Action to take if the domain crashes.
    pub on_crash: EventAction,
    /// Action to take if the domain performs a 'soft reset' (e.g. does `kexec`).
    pub on_soft_reset: EventAction,
}

impl Default for DomainActions {
    fn default() -> Self {
        Self {
            on_poweroff: EventAction::Destroy,
            on_reboot: EventAction::Restart,
            on_watchdog: EventAction::Destroy,
            on_crash: EventAction::Destroy,
            on_soft_reset: EventAction::SoftReset,
        }
    }
}

impl Display for DomainActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "on_poweroff={}, on_reboot={}, on_watchdog={}, on_crash={}, on_soft_reset={}",
            self.on_poweroff, self.on_reboot, self.on_watchdog, self.on_crash, self.on_soft_reset
        )
    }
}

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
            Firmware::Path(path) => write!(f, "path={}", path.display()),
        }
    }
}

/// Represents the boot device for the virtual machine
///
/// The boot device is used to specify the device from which the virtual machine should boot.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BootDevice {
    #[default]
    HardDisk,
    CdRom,
    Network,
}

impl Display for BootDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootDevice::HardDisk => write!(f, "c"),
            BootDevice::CdRom => write!(f, "d"),
            BootDevice::Network => write!(f, "n"),
        }
    }
}

/// Represents the type of emulated disk controller to use
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EmulatedDiskControllerType {
    /// Adds an emulated IDE controller, which is
    /// suitable even for older operation systems.
    Ide,
    /// Adds an ich9 disk controller in AHCI mode and
    /// uses it with upstream QEMU to emulate disks instead of IDE. It decreases boot
    /// time but may not be supported by default in older operating systems, e.g.
    /// Windows XP.
    #[default]
    Ahci,
}

impl Display for EmulatedDiskControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmulatedDiskControllerType::Ide => write!(f, "ide"),
            EmulatedDiskControllerType::Ahci => write!(f, "ahci"),
        }
    }
}

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
    hypervisor: CpuidFeatureBit,
    /// The vendor info is a 12-byte (96 bit) long string, which is used to
    /// identify the vendor of the CPU. This is used by some software to
    /// determine the CPU vendor, and can be used to detect if the CPU is
    /// running in a virtual machine.
    vendor: [u8; 12],
    /// Processor Brand String is a 48-byte (384 bit) long string, which is
    /// used to identify the brand of the CPU. This is used by some software
    /// to determine the CPU brand, and can be used to detect if the CPU is
    /// running in a virtual machine.
    ///
    /// See https://en.wikipedia.org/wiki/CPUID#EAX=8000'0002h,8000'0003h,8000'0004h:_Processor_Brand_String
    processor_brand_string: [u8; 48],
    /// The hypervisor brand is a 12-byte (96 bit) long string, which is used
    /// to identify the brand of the hypervisor. This is used by some software
    /// to determine the hypervisor brand, and can be used to detect if the CPU
    /// is running in a virtual machine.
    ///
    /// See https://en.wikipedia.org/wiki/CPUID#EAX=4000'0000h-4FFFF'FFFh:_Reserved_for_Hypervisors
    hypervisor_brand: [u8; 12],
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
            vendor: vendor,
            processor_brand_string: processor_brand,
            hypervisor_brand,
        })
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
    disks: Vec<Disk>,
    /// List of network interfaces attached to the virtual machine
    network_interfaces: Vec<NetworkInterface>,
    /// Actions to take when a domain event occurs
    domain_actions: DomainActions,
    /// The firmware to use for the virtual machine
    firmware: Firmware,
    /// Specifies the emulated virtual device to boot from.
    ///
    /// **Note**: multiple options can be given and will be attempted in the order they are
    /// given, e.g. to boot from CD-ROM but fall back to the hard disk you can specify it
    /// as dc.
    boot_device: Vec<BootDevice>,
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
}
