# Xenith - Xen-based security hypervisor
# Copyright (C) 2025 Xenith contributors

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

Vagrant.configure(2) do |config|
    config.vm.box = "debian/bookworm64"
    config.vm.define :xenith do |xenith|
        xenith.vm.hostname = "xenith"
        xenith.vm.network :private_network, :ip => "192.168.126.10"
    end

    # Configure libvirt settings
    config.vm.provider :libvirt do |libvirt|
        libvirt.driver = "kvm"
        libvirt.kvm_hidden = true
        libvirt.nested = true
        libvirt.machine_virtual_size = 100

        # Configure storage
        libvirt.storage :file, :size => '30G' # vdb, dom0 storage
        libvirt.storage :file, :size => '30G' # vdc, domU storage

        # Configure CPU and memory
        libvirt.cpus = 8
        # check https://libvirt.org/formatdomain.html#cpu-model-and-topology
        libvirt.cpu_mode = 'host-model'
        libvirt.cpu_fallback = 'forbid'
        libvirt.memory = 8192

        # Configure network
        libvirt.nic_model_type = "virtio"
        libvirt.management_network_name = 'xenith-network'
        libvirt.management_network_address = '192.168.126.0/24'

        # Configure graphics
        libvirt.video_type = "qxl"
        libvirt.graphics_type = "vnc"

        libvirt.memorybacking :access, :mode => "shared"
    end

    # Synced folders
    config.vm.synced_folder "./", "/vagrant", type: "virtiofs"

    # Provisioning
    ANSIBLE_COMPATIBILITY_MODE = "2.0"
    ANSIBLE_VERBOSITY = "" # can be up to "-vvv" for more verbosity

    # Pre-reboot
    # - Install Xen
    # - Install common packages
    config.vm.provision "ansible" do |ansible|
        ansible.compatibility_mode = ANSIBLE_COMPATIBILITY_MODE
        ansible.verbose = ANSIBLE_VERBOSITY
        ansible.playbook = "ansible/pre_reboot.yml"
    end

    config.vm.provision :reload

    # Post-reboot
    # - Install and configure SSH daemon
    # - Disk partitioning
    config.vm.provision "ansible" do |ansible|
        ansible.compatibility_mode = ANSIBLE_COMPATIBILITY_MODE
        ansible.verbose = ANSIBLE_VERBOSITY
        ansible.playbook = "ansible/post_reboot.yml"
    end
end
