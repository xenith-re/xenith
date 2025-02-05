use std::fmt::Display;

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
use log::debug;
use thiserror::Error;

use crate::techniques::*;

pub type TechniqueResult = Result<DetectionResult, TechniqueError>;
pub type TechniqueFn = fn() -> TechniqueResult;

/// Error type for techniques
///
/// This error type is used to represent errors that can occur when running a technique.
#[derive(Error, Debug)]
pub enum TechniqueError {
    #[error("Technique failed")]
    Failed(),
    #[error("Technique not implemented")]
    NotImplemented,
    #[error("Unknown error")]
    Unknown,
}

/// A redpill technique
///
/// This struct represents a redpill technique that can be used to detect the presence of the Xen hypervisor.
/// It contains a name, a description, and a function pointer to the technique implementation.
///
/// # Example
///
/// ```
/// use xenith_redpill::detector::{DetectionResult, Technique, TechniqueError};
///
/// fn technique_fn() -> Result<DetectionResult, TechniqueError> {
///    // Technique implementation
///    Ok(DetectionResult::Detected)
/// }
///
/// let technique = Technique::new(
///    String::from("technique_name"),
///    String::from("Technique description"),
///    technique_fn,
/// );
///
/// let result = technique.run().unwrap();
///
/// let detected = match result {
///   DetectionResult::Detected => true,
///   DetectionResult::NotDetected => false,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Technique {
    name: String,
    description: String,
    fn_ptr: TechniqueFn,
}

impl Technique {
    /// Create a new technique
    ///
    /// This function creates a new technique with the given name, description, and function pointer.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the technique
    /// - `description`: The description of the technique
    /// - `fn_ptr`: The function pointer to the technique implementation
    ///
    /// # Returns
    ///
    /// A new technique
    pub fn new(name: String, description: String, fn_ptr: TechniqueFn) -> Self {
        Self {
            name,
            description,
            fn_ptr,
        }
    }

    /// Run the technique
    ///
    /// This function runs the technique and returns the result.
    ///
    /// # Returns
    ///
    /// The result of the technique
    pub fn run(&self) -> TechniqueResult {
        (self.fn_ptr)()
    }

    /// Get the name of the technique
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the description of the technique
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get the function pointer of the technique
    pub fn fn_ptr(&self) -> TechniqueFn {
        self.fn_ptr
    }
}

impl Display for Technique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

/// Detection result
///
/// This enum represents the result of a detection technique. It can be either detected or not detected.
/// If detected, it contains the name and description of the used technique.
pub enum DetectionResult {
    Detected,
    NotDetected,
}

/// Trait for techniques
///
/// This trait defines the interface for a list of techniques that can be used to detect the presence of the Xen hypervisor.
pub trait TechniqueList {
    /// Get a list of techniques
    ///
    /// This function returns a list of techniques that are implemented by the technique list.
    ///
    /// # Returns
    ///
    /// A list of techniques
    fn get_techniques(&self) -> Vec<Technique>;

    /// Run all techniques in the list
    ///
    /// This function runs all techniques in the list and returns true if any of them
    /// return true. If none of the techniques return true, the function returns false.
    ///
    /// # Errors
    ///
    /// This function returns an error if any of the techniques fail.
    ///
    /// TODO:
    /// - return a list of detected techniques
    fn detect(&self) -> Vec<(Technique, TechniqueResult)> {
        let techniques = self.get_techniques();
        let mut results = Vec::new();
        for technique in techniques {
            debug!("Running technique: {technique}");
            let result = technique.run();
            results.push((technique, result));
        }
        results
    }
}

/// Run all techniques to detect the presence of the Xen hypervisor
///
/// This function runs all techniques to detect the presence of the Xen hypervisor
/// by analyzing different detection aspects of the system like behavior, signatures, and time.
///
/// # Returns
///
/// A list of detection results for each technique
pub fn run_all_techniques() -> Vec<(Technique, TechniqueResult)> {
    let techniques_list = vec![behavior::BehaviorTechniques];

    let mut results = Vec::new();
    for techniques in techniques_list {
        results.extend(techniques.detect());
    }
    results
}

/// Run all techniques in parallel to detect the presence of the Xen hypervisor
///
/// This function runs all techniques in parallel to detect the presence of the Xen hypervisor
/// by analyzing different detection aspects of the system like behavior, signatures, and time.
///
/// # Returns
///
/// A list of detection results for each technique
pub fn run_all_techniques_parallel() -> Vec<TechniqueResult> {
    unimplemented!("Parallel detection is not implemented yet")
}
