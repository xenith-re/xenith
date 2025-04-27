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

#[cfg(test)]
mod tests {
    use super::*;

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
}
