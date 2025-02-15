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

//! # Signature-Based Detection
//!
//! This module implements signature-based techniques to detect the presence of the Xen hypervisor
//! by analyzing memory for known patterns or OS-specific structures.

use log::error;
use raw_cpuid::CpuId;
use static_init::dynamic;

use crate::{
    detector::{register_technique, DetectionResult, Technique, TechniqueResult},
    prelude::TechniqueError,
};

use xenith_redpill_macros::technique;

#[technique(
    name = "VMID",
    description = "Check CPUID output of manufacturer ID for known VMs/hypervisors at leaf 0",
    os = "all"
)]
fn vmid() -> TechniqueResult {
    let vmid = "XenVMMXenVMM";

    let cpuid = CpuId::new();

    if let Some(vendor_id) = cpuid.get_vendor_info() {
        if vendor_id.as_str() == vmid {
            return Ok(DetectionResult::Detected);
        }
    } else {
        return Err(TechniqueError::Failed());
    };

    Ok(DetectionResult::NotDetected)
}
