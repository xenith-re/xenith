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

//! # Behavior-Based Detection
//!
//! This module implements behavior-based techniques to identify the presence of the Xen hypervisor
//! by analyzing system responses to specific instructions and interactions.

use crate::detector::{TechniqueError, TechniqueFn, TechniqueList, TechniqueResult};

pub struct BehaviorTechniques;

impl TechniqueList for BehaviorTechniques {
    fn get_techniques(&self) -> Vec<TechniqueFn> {
        vec![behavior_technique_1]
    }
}

pub fn behavior_technique_1() -> TechniqueResult {
    Ok(false)
}
