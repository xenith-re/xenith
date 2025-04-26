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

use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::{Error as LibvirtError, clear_error_callback};
use virt::sys;

use crate::error::DriverError;

pub struct Driver {
    connection: Connect,
}

impl Driver {
    const XEN_URI: &'static str = "xen:///system";

    pub fn new() -> Result<Self, DriverError> {
        // Initialize the libvirt connection
        let connection =
            Connect::open(Some(Driver::XEN_URI)).map_err(|e| DriverError::Connection(e))?;

        // Do not print errors to stdout
        clear_error_callback();

        Ok(Driver { connection })
    }
}
