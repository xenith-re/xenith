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
//! This module contains the configuration for the Xenith
//! domain. It contains the configuration for the domain
//! disks, templates, and the domain configuration itself.
//! It also contains the functions to create the necessary
//! directories for the domain configuration and images.
//!
//! The configuration is stored in the `/xenith` directory
//! and the images are stored in the `/xenith/images` directory.
//! The domain configuration files are stored in the `/xenith/domains` directory.
//! The ansible playbooks and roles are stored in the `/xenith/ansible` directory.

use std::fs::create_dir_all;
use std::path::Path;

/// Xenith domain configuration directory
/// This is the directory where all domain configurations are stored.
pub const DOMAIN_CONFIGURATION_PATH: &str = "/xenith";

/// Xenith images directory
/// This directory is used to store the images of the domains
/// created by Xenith.
pub const IMAGES_PATH: &str = "/xenith/images";

/// Xenith domains directory
/// This directory is used to store the domains created by Xenith.
/// It is used to store the domain configuration files and
/// the domain images.
pub const DOMAINS_PATH: &str = "/xenith/domains";

/// Xenith ansible directory
/// This directory is used to store the ansible playbooks
/// and roles used to configure the domains created by Xenith.
pub const ANSIBLE_PATH: &str = "/xenith/ansible";

/// Creates the necessary directories for Xenith
/// domains configuration and images.
pub fn create_paths() -> std::io::Result<()> {
    let paths = [
        DOMAIN_CONFIGURATION_PATH,
        IMAGES_PATH,
        DOMAINS_PATH,
        ANSIBLE_PATH,
    ];

    for path in paths.iter() {
        if !Path::new(path).exists() {
            create_dir_all(path)?;
        }
    }

    Ok(())
}

/// Creates the base domain configuration directory
/// for a given domain name. It creates the necessary directories
/// for the domain configuration, disks, and templates.
/// If the directory already exists, it does nothing.
/// Returns the path to the domain configuration directory.
pub fn create_base_domain_configuration(domain_name: &str) -> Result<String, std::io::Error> {
    let path = format!("{}/{}", DOMAIN_CONFIGURATION_PATH, domain_name);

    if !Path::new(&path).exists() {
        create_dir_all(&path)?;

        let disks_path = format!("{}/disks/snapshots", path);
        let templates_path = format!("{}/templates", path);

        create_dir_all(&disks_path)?;
        create_dir_all(&templates_path)?;
    }

    Ok(path)
}

/// Returns the path to the domain configuration directory
/// for a given domain name. This is used to get the path
/// to the domain configuration directory without creating it.
pub fn get_domain_configuration_path(domain_name: &str) -> String {
    format!("{}/{}", DOMAIN_CONFIGURATION_PATH, domain_name)
}

/// Returns the path to the domain disks directory
/// for a given domain name. This is used to get the path
/// to the domain disks directory without creating it.
pub fn get_domain_disks_path(domain_name: &str) -> String {
    format!("{}/disks", get_domain_configuration_path(domain_name))
}

/// Returns the path to the domain snapshots directory
/// for a given domain name. This is used to get the path
/// to the domain snapshots directory without creating it.
pub fn get_domain_snapshots_path(domain_name: &str) -> String {
    format!("{}/snapshots", get_domain_disks_path(domain_name))
}

/// Returns the path to the domain templates directory
/// for a given domain name. This is used to get the path
/// to the domain templates directory without creating it.
pub fn get_domain_templates_path(domain_name: &str) -> String {
    format!("{}/templates", get_domain_configuration_path(domain_name))
}
