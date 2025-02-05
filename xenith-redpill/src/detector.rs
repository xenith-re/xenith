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
use thiserror::Error;

use crate::techniques::*;

/// Error type for techniques
///
/// This error type is used to represent errors that can occur when running a technique.
///
#[derive(Error, Debug)]
pub enum TechniqueError {
    #[error("Technique failed")]
    Failed(),
    #[error("Technique not implemented")]
    NotImplemented,
    #[error("Unknown error")]
    Unknown,
}

pub type TechniqueResult = Result<bool, TechniqueError>;
pub type TechniqueFn = fn() -> TechniqueResult;

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
    fn get_techniques(&self) -> Vec<TechniqueFn>;

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
    fn detect(&self) -> Result<bool, TechniqueError> {
        let techniques = self.get_techniques();
        for technique in techniques {
            if technique()? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// Run all techniques to detect the presence of the Xen hypervisor
///
/// This function runs all techniques to detect the presence of the Xen hypervisor
/// by analyzing different detection aspects of the system like behavior, signatures, and time.
pub fn run_all_techniques() {
    let techniques = vec![behavior::BehaviorTechniques];

    // TODO:
    // - enhance this to run all techniques in parallel
    // - return a list of detected techniques
    // - handle errors
    for technique in techniques {
        if technique.detect().unwrap() {
            println!("Xen detected!");
            return;
        }
    }
}
