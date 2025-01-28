# TODO:
# - check PCI passthrough (https://vagrant-libvirt.github.io/vagrant-libvirt/configuration.html#pci-device-passthrough)
# - edit host configuration (https://wiki.xenproject.org/wiki/Category:Host_Configuration)

Vagrant.configure(2) do |config|
    config.vm.box = "debian/bookworm64"
    config.vm.define :xenith do |xenith|
        xenith.vm.hostname = "xenith"
        xenith.vm.network :private_network, :ip => "192.168.124.10"
    end

    # Disable synced folder
    config.vm.synced_folder ".", "/vagrant", disabled: true

    # Configure provider-specific settings
    config.vm.provider :libvirt do |libvirt|
        libvirt.driver = "kvm"
        libvirt.kvm_hidden = true
        libvirt.nested = true
        libvirt.machine_virtual_size = 100

        libvirt.storage :file, :size => '30G' # vdb
        libvirt.storage :file, :size => '30G' # vdc

        # Configure CPU and memory
        libvirt.cpus = 8
        # check https://libvirt.org/formatdomain.html#cpu-model-and-topology
        libvirt.cpu_mode = 'host-model'
        libvirt.cpu_fallback = 'forbid'
        libvirt.memory = 8192

        # Configure network
        libvirt.nic_model_type = "virtio"
        libvirt.management_network_name = 'xenith-network'
        libvirt.management_network_address = '192.168.124.0/24'
    end

    # Provisioning
    ANSIBLE_COMPATIBILITY_MODE = "2.0"
    ANSIBLE_VERBOSITY = "" # can be up to "-vvv" for more verbosity

    config.vm.provision "ansible" do |ansible|
        ansible.compatibility_mode = ANSIBLE_COMPATIBILITY_MODE
        ansible.verbose = ANSIBLE_VERBOSITY
        ansible.playbook = "ansible/pre_reboot.yml"
    end

    config.vm.provision :reload

    config.vm.provision "ansible" do |ansible|
        ansible.compatibility_mode = ANSIBLE_COMPATIBILITY_MODE
        ansible.verbose = ANSIBLE_VERBOSITY
        ansible.playbook = "ansible/post_reboot.yml"
    end
end
