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

use std::fmt::Display;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::configuration::Disk;
use crate::configuration::Template;

/// Domain configuration
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Domain {
    /// Name of the domain
    name: String,
    /// Path to the domain configuration directory
    /// This is the directory where all domain configurations are stored.
    path: PathBuf,
    /// Path to the domain libvirt configuration file
    /// This is the file that contains the domain configuration
    /// in libvirt XML format.
    /// It is used to create the domain in libvirt.
    configuration_file: Option<PathBuf>,
    /// List of disks for the domain
    /// This is the list of disks that are used by the domain.
    disks: Vec<Disk>,
    /// Packer templates for the domain
    templates: Option<Template>,
}

impl Domain {
    pub fn new(
        name: String,
        path: PathBuf,
        configuration_file: Option<PathBuf>,
        disks: Vec<Disk>,
        templates: Option<Template>,
    ) -> Self {
        Self {
            name,
            path,
            configuration_file,
            disks,
            templates,
        }
    }

    /// Get the domain name
    /// This is the name of the domain that is used to create
    /// the domain in libvirt.
    /// It is also used to create the domain configuration directory.
    ///
    /// # Returns
    ///
    /// * `&String` - The name of the domain.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get the domain configuration directory
    /// This is the directory where all domain configurations are stored.
    /// It is used to create the domain configuration directory.
    ///
    /// # Returns
    ///
    /// * `&PathBuf` - The path to the domain configuration directory.
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the domain configuration file
    /// This is the file that contains the domain configuration
    /// in libvirt XML format.
    /// It is used to create the domain in libvirt.
    ///
    /// It may not exist if the domain is not created yet.
    ///
    /// # Returns
    ///
    /// * `Option<&PathBuf>` - The path to the domain configuration file.
    pub fn get_configuration_file(&self) -> Option<&PathBuf> {
        self.configuration_file.as_ref()
    }

    /// Get the disks for the domain
    /// This is the list of disks that are used by the domain.
    /// It is used to create the domain in libvirt.
    ///
    /// # Returns
    ///
    /// * `&Vec<Disk>` - The list of disks for the domain.
    pub fn get_disks(&self) -> &Vec<Disk> {
        &self.disks
    }

    /// Get the Packer templates for the domain
    /// It is used to create the domain disk image.
    ///
    /// It may not exist if the user uses a custom image.
    ///
    /// # Returns
    ///
    /// * `Option<&Template>` - The templates for the domain.
    pub fn get_templates(&self) -> Option<&Template> {
        self.templates.as_ref()
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Domain({{ name: {}, path: {}, disks: {:?} }})",
            self.name,
            self.path.display(),
            self.disks
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::domain::DiskFormat;

    #[test]
    fn test_domain_creation() {
        let disk = Disk::new(
            "test_disk".to_string(),
            PathBuf::from("/xenith/disks/test_disk.img"),
            1024 * 1024 * 1024,
            DiskFormat::Raw,
        );
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            Some(PathBuf::from("/xenith/domains/test_domain/config.xml")),
            vec![disk.clone()],
            None,
        );

        assert_eq!(domain.get_name(), "test_domain");
        assert_eq!(
            domain.get_path(),
            &PathBuf::from("/xenith/domains/test_domain")
        );
        assert_eq!(
            domain.get_configuration_file(),
            Some(&PathBuf::from("/xenith/domains/test_domain/config.xml"))
        );
        assert_eq!(domain.get_disks(), &vec![disk]);
        assert!(domain.get_templates().is_none());
    }
}
