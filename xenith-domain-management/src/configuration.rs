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
//! This module contains the configuration for Xenith
//! domains like disks, templates, and the domain configuration
//! itself.
//!
//! The configuration is stored in the `/xenith` directory
//! and the images are stored in the `/xenith/images` directory.
//! The domain configuration files are stored in the `/xenith/domains` directory.
//! The ansible playbooks and roles are stored in the `/xenith/ansible` directory.

use std::fs::create_dir_all;
use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::DiskFormat;
use crate::error::ConfigurationError;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Image {
    name: String,
    path: PathBuf,
    checksum: String,
}

impl Image {
    pub fn new(name: String, path: PathBuf, checksum: String) -> Self {
        Self {
            name,
            path,
            checksum,
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_checksum(&self) -> &String {
        &self.checksum
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image({{ name: {}, path: {}, checksum: {} }})",
            self.name,
            self.path.display(),
            self.checksum
        )
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Template {
    image_template: PathBuf,
    variables: Option<PathBuf>,
}

impl Template {
    pub fn new(image_template: PathBuf, variables: Option<PathBuf>) -> Self {
        Self {
            image_template,
            variables,
        }
    }

    pub fn get_image_template(&self) -> &PathBuf {
        &self.image_template
    }

    pub fn get_variables(&self) -> Option<&PathBuf> {
        self.variables.as_ref()
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Template({{ image_template: {}, variables: {} }})",
            self.image_template.display(),
            self.variables
                .as_ref()
                .map_or("None".to_string(), |v| v.display().to_string())
        )
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Disk {
    name: String,
    path: PathBuf,
    /// Size of the disk in bytes. This is required for file-based disk images.
    size: u64,
    format: DiskFormat,
}

impl Disk {
    pub fn new(name: String, path: PathBuf, size: u64, format: DiskFormat) -> Self {
        Self {
            name,
            path,
            size,
            format,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_format(&self) -> &DiskFormat {
        &self.format
    }

    pub fn get_size_in_gb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    pub fn get_size_in_mb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0)
    }

    pub fn get_size_in_kb(&self) -> f64 {
        self.size as f64 / 1024.0
    }

    pub fn get_size_in_bytes(&self) -> u64 {
        self.size
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Disk({{ name: {}, path: {}, size: {}, format: {} }})",
            self.name,
            self.path.display(),
            self.get_size_in_gb(),
            self.format
        )
    }
}

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

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Configuration {
    images: Vec<Image>,
    domains: Vec<Domain>,
}

impl Configuration {
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

    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            domains: Vec::new(),
        }
    }

    pub fn get_images(&self) -> &Vec<Image> {
        &self.images
    }

    pub fn get_domains(&self) -> &Vec<Domain> {
        &self.domains
    }

    /// Adds an image to the configuration.
    ///
    /// # Arguments
    ///
    /// * `image` - The image to add.
    pub fn add_image(&mut self, image: Image) {
        self.images.push(image);
    }

    /// Adds a domain to the configuration.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain to add.
    pub fn add_domain(&mut self, domain: Domain) {
        self.domains.push(domain);
    }

    /// Creates the necessary directories for Xenith
    /// domains configuration and images.
    ///
    /// If the directory already exists, it does nothing.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the directories were created successfully.
    /// * `Err(std::io::Error)` - If the directories could not be created.
    pub fn create_configuration(&self) -> Result<(), ConfigurationError> {
        let paths = [
            Self::DOMAIN_CONFIGURATION_PATH,
            Self::IMAGES_PATH,
            Self::DOMAINS_PATH,
            Self::ANSIBLE_PATH,
        ];

        for path in paths.iter() {
            if !PathBuf::from(path).exists() {
                create_dir_all(path)?;
            }
        }

        Ok(())
    }

    /// Creates the base domain configuration directory
    /// for a given domain name. It creates the necessary directories
    /// for the domain configuration, disks, and templates.
    ///
    /// If the directory already exists, it does nothing.
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Ok(PathBuf)` - The path to the domain configuration directory.
    /// * `Err(std::io::Error)` - If the directory could not be created.
    pub fn create_domain_configuration(domain_name: &str) -> Result<PathBuf, ConfigurationError> {
        let path = PathBuf::from(format!("{}/{}", Self::DOMAINS_PATH, domain_name));

        if !path.exists() {
            create_dir_all(&path)?;

            let disks_path = path.join("disks").join("snapshots");
            let templates_path = path.join("templates");

            create_dir_all(&disks_path)?;
            create_dir_all(&templates_path)?;
        }

        Ok(path)
    }

    /// Get an image by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the image.
    ///
    /// # Returns
    ///
    /// * `Option<&Image>` - The image if it exists, otherwise None.
    pub fn get_image(&self, name: &str) -> Option<&Image> {
        self.images.iter().find(|image| image.name == name)
    }

    /// Get the path to an image by its name
    ///
    /// # Arguments
    ///
    /// * `image_name` - The name of the image.
    ///
    /// # Returns
    ///
    /// * `Option<PathBuf>` - The path to the image if it exists, otherwise None.
    pub fn get_image_path(&self, image_name: &str) -> Option<PathBuf> {
        self.get_image(image_name).map(|image| image.path.clone())
    }

    /// Get a domain by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&Domain>` - The domain if it exists, otherwise None.
    pub fn get_domain(&self, name: &str) -> Option<&Domain> {
        self.domains.iter().find(|domain| domain.name == name)
    }

    /// Get the path to a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<PathBuf>` - The path to the domain if it exists, otherwise None.
    pub fn get_domain_path(&self, domain_name: &str) -> Option<PathBuf> {
        self.get_domain(domain_name)
            .map(|domain| domain.path.clone())
    }

    /// Get the disks for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&Vec<Disk>>` - The disks for the domain if it exists, otherwise None.
    pub fn get_domain_disks(&self, domain_name: &str) -> Option<&Vec<Disk>> {
        self.get_domain(domain_name).map(|domain| &domain.disks)
    }

    /// Get the libvirt configuration file for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&PathBuf>` - The path to the libvirt configuration file if it exists, otherwise None.
    pub fn get_domain_libvirt_configuration_file(&self, domain_name: &str) -> Option<&PathBuf> {
        self.get_domain(domain_name)
            .and_then(|domain| domain.configuration_file.as_ref())
    }

    /// Get a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<&Disk>` - The disk for the domain if it exists, otherwise None.
    pub fn get_disk(&self, domain_name: &str, disk_name: &str) -> Option<&Disk> {
        self.get_domain_disks(domain_name)
            .and_then(|disks| disks.iter().find(|disk| disk.name == disk_name))
    }

    /// Get the path to a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<PathBuf>` - The path to the disk if it exists, otherwise None.
    pub fn get_disk_path(&self, domain_name: &str, disk_name: &str) -> Option<PathBuf> {
        self.get_disk(domain_name, disk_name)
            .map(|disk| disk.path.clone())
    }

    /// Get the size of a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<u64>` - The size of the disk in bytes if it exists, otherwise None.
    pub fn get_disk_size(&self, domain_name: &str, disk_name: &str) -> Option<u64> {
        self.get_disk(domain_name, disk_name).map(|disk| disk.size)
    }

    /// Get the format of a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<DiskFormat>` - The format of the disk if it exists, otherwise None.
    pub fn get_disk_format(&self, domain_name: &str, disk_name: &str) -> Option<DiskFormat> {
        self.get_disk(domain_name, disk_name)
            .map(|disk| disk.format.clone())
    }

    /// Get the Packer templates for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&Template>` - The templates for the domain if it exists, otherwise None.
    pub fn get_domain_templates(&self, domain_name: &str) -> Option<&Template> {
        self.get_domain(domain_name)
            .and_then(|domain| domain.templates.as_ref())
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Configuration({{ images: {:?}, domains: {:?} }})",
            self.images, self.domains
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let image = Image::new(
            "test_image".to_string(),
            PathBuf::from("/xenith/images/test_image.img"),
            "checksum123".to_string(),
        );

        assert_eq!(
            image.get_path(),
            &PathBuf::from("/xenith/images/test_image.img")
        );
        assert_eq!(image.get_checksum(), "checksum123");
    }

    #[test]
    fn test_template_creation() {
        let template = Template::new(
            PathBuf::from("/xenith/templates/template.json"),
            Some(PathBuf::from("/xenith/templates/variables.json")),
        );

        assert_eq!(
            template.get_image_template(),
            &PathBuf::from("/xenith/templates/template.json")
        );
        assert_eq!(
            template.get_variables(),
            Some(&PathBuf::from("/xenith/templates/variables.json"))
        );
    }

    #[test]
    fn test_disk_creation() {
        let disk = Disk::new(
            "test_disk".to_string(),
            PathBuf::from("/xenith/disks/test_disk.img"),
            1024 * 1024 * 1024,
            DiskFormat::Raw,
        );

        assert_eq!(disk.get_name(), "test_disk");
        assert_eq!(
            disk.get_path(),
            &PathBuf::from("/xenith/disks/test_disk.img")
        );
        assert_eq!(disk.get_size_in_gb(), 1.0);
        assert_eq!(disk.get_format(), &DiskFormat::Raw);
    }

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

    #[test]
    fn test_configuration_add_image() {
        let mut config = Configuration::new();
        let image = Image::new(
            "test_image".to_string(),
            PathBuf::from("/xenith/images/test_image.img"),
            "checksum123".to_string(),
        );

        config.add_image(image.clone());
        assert_eq!(config.get_images(), &vec![image]);
    }

    #[test]
    fn test_configuration_add_domain() {
        let mut config = Configuration::new();
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            None,
            vec![],
            None,
        );

        config.add_domain(domain.clone());
        assert_eq!(config.get_domains(), &vec![domain]);
    }

    #[test]
    fn test_configuration_get_image() {
        let mut config = Configuration::new();
        let image = Image::new(
            "test_image".to_string(),
            PathBuf::from("/xenith/images/test_image.img"),
            "checksum123".to_string(),
        );

        config.add_image(image.clone());
        assert_eq!(config.get_image("test_image"), Some(&image));
        assert!(config.get_image("nonexistent_image").is_none());
    }

    #[test]
    fn test_configuration_get_domain() {
        let mut config = Configuration::new();
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            None,
            vec![],
            None,
        );

        config.add_domain(domain.clone());
        assert_eq!(config.get_domain("test_domain"), Some(&domain));
        assert!(config.get_domain("nonexistent_domain").is_none());
    }

    // This must be mocked or run in a test environment

    // #[test]
    // fn test_configuration_create_configuration() {
    //     let config = Configuration::new();

    //     let result = config.create_configuration();
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_configuration_create_domain_configuration() {
    //     let result = Configuration::create_domain_configuration("test_domain");
    //     assert!(result.is_ok());
    //     assert_eq!(
    //         result.unwrap(),
    //         PathBuf::from("/xenith/domains/test_domain")
    //     );
    // }

    #[test]
    fn test_configuration_get_disk() {
        let disk = Disk::new(
            "test_disk".to_string(),
            PathBuf::from("/xenith/disks/test_disk.img"),
            1024 * 1024 * 1024,
            DiskFormat::Raw,
        );
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            None,
            vec![disk.clone()],
            None,
        );
        let mut config = Configuration::new();
        config.add_domain(domain);

        assert_eq!(config.get_disk("test_domain", "test_disk"), Some(&disk));
        assert!(config.get_disk("test_domain", "nonexistent_disk").is_none());
    }
}
