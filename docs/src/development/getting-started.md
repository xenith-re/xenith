# Getting Started

Xenith is a complex project that requires some setup to get started. This guide will walk you through the steps to get your development environment up and running. We use technologies like [Vagrant](https://www.vagrantup.com/) and [Ansible](https://docs.ansible.com/ansible/latest/index.html) to make the setup process as easy as possible managing powerful tools like [libvirt](https://libvirt.org/) and [KVM](https://linux-kvm.org/page/Main_Page)/[QEMU](https://www.qemu.org/).

## Prerequisites

### KVM

```admonish summary
KVM, Kernel-based Virtual Machine, is a hypervisor built into the Linux kernel. It is similar to Xen in purpose but much simpler to get running. Unlike native QEMU, which uses emulation, KVM is a special operating mode of QEMU that uses CPU extensions (HVM) for virtualization via a kernel module.
```

We use KVM as the backend [VMM](https://en.wikipedia.org/wiki/Hypervisor) for QEMU which runs Xen, managed by Vagrant through libvirt. As KVM is built into the Linux kernel you only need to ensure that the necessary kernel modules are loaded and it supports **nested virtualization** (enables existing virtual machines to be run on third-party hypervisors).

**Arch Linux**

Refer to the [Arch Wiki](https://wiki.archlinux.org/title/KVM) for more information.

**Ubuntu**

Refer to the official Ubuntu blog post [KVM hypervisor: a beginners’ guide](https://ubuntu.com/blog/kvm-hyphervisor).

### QEMU

```admonish summary
QEMU is a generic and open source machine emulator and virtualizer. QEMU can use other hypervisors like Xen or KVM to use CPU extensions (HVM) for virtualization. When used as a virtualizer, QEMU achieves near native performances by executing the guest code directly on the host CPU.
```

You can install QEMU on your system using the following commands:

**Arch Linux**

```shell
sudo pacman -S qemu-desktop
```

**Ubuntu**

```shell
sudo apt install qemu qemu-kvm bridge-utils
```

### libvirt

```admonish summary
Libvirt is a collection of software that provides a convenient way to manage virtual machines and other virtualization functionality, such as storage and network interface management. These software pieces include a long term stable C API, a daemon (libvirtd), and a command line utility (virsh).
```

You can install libvirt on your system using the following commands:

**Arch Linux**

```shell
sudo pacman -S libvirt
```

**Ubuntu**

```shell
sudo apt install libvirt-daemon-system
```

The easiest way to ensure your user has access to libvirt daemon is to add member to `libvirt` user group. Members of the libvirt group have passwordless access to the RW daemon socket by default. You can do that by running the following commands:

```shell
sudo usermod -aG libvirt $USER
sudo usermod -aG libvirt-qemu $USER
sudo usermod -aG libvirt-kvm $USER
```

And then start the `libvirtd` service:

```shell
sudo systemctl start libvirtd
```

~~~admonish tip
If you don't want to start the `libvirtd` service after each reboot, you can enable it by running the following command:
```shell
sudo systemctl start libvirtd
```
~~~

### Vagrant

```admonish summary
Vagrant is a source-available software product for building and maintaining portable virtual software development environments; e.g., for VirtualBox, KVM, Hyper-V, Docker containers, VMware, Parallels, and AWS. It tries to simplify the software configuration management of virtualization in order to increase development productivity.
```

We use Vagrant to manage the virtual machine that enables us to boot Xen and its [dom0](https://wiki.xenproject.org/wiki/Dom0) Debian VM. You can install Vagrant on your system using the following commands:

**Arch Linux**

```shell
sudo pacman -S vagrant
```

**Ubuntu**

```shell
# Install repository addition dependencies
sudo apt update
sudo apt -y install apt-transport-https ca-certificates curl software-properties-common

# Import repository GPG keys
wget -O- https://apt.releases.hashicorp.com/gpg | gpg --dearmor | sudo tee /usr/share/keyrings/hashicorp-archive-keyring.gpg

# Add the official Vagrant APT repository to your system
echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list

# Once the repo is added, proceed to install vagrant
sudo apt update
sudo apt install vagrant
```

#### vagrant-libvirt plugin

```admonish summary
Vagrant-libvirt is a Vagrant plugin that adds a Libvirt provider to Vagrant, allowing Vagrant to control and provision machines via Libvirt toolkit.
See [vagrant-libvirt](https://vagrant-libvirt.github.io/vagrant-libvirt/) for more information.
```

**Arch Linux**

As mentioned in the [Arch Wiki](https://wiki.archlinux.org/title/Vagrant#vagrant-libvirt), the libvirt plugin is not compatible with the ruby gems as currently shipped with the vagrant package in the Arch repos (which are up-to-date). This might cause an error such as `Vagrant failed to properly resolve required dependencies`.

A quick workaround is to disable the strict dependency enforcement by setting the `VAGRANT_DISABLE_STRICT_DEPENDENCY_ENFORCEMENT` environment variable to `1` before installing the plugin:

```shell
export VAGRANT_DISABLE_STRICT_DEPENDENCY_ENFORCEMENT=1
```

This manipulation can be done in your shell configuration file (e.g. `~/.bashrc`, `~/.zshrc`, etc.) to avoid running it every time you want to install and/or update a plugin.

**Ubuntu**

Make sure you have the necessary dependencies installed:

```shell
sudo apt install ebtables libguestfs-tools ruby-fog-libvirt
```

Then install the plugin:

```shell
vagrant plugin install vagrant-libvirt
```

#### vagrant-reload plugin

```admonish summary
A plugin that allows you to reload a Vagrant plugin as a provisioning step.
See [vagrant-reload](https://github.com/aidanns/vagrant-reload) for more information.
```

```shell
vagrant plugin install vagrant-reload
```

### Ansible

```admonish summary
Ansible is a suite of software tools that enables infrastructure as code. It is open-source and the suite includes software provisioning, configuration management, and application deployment functionality.
```

We use Ansible to provision the virtual machines that we use for development. You can install Ansible on your system using the following commands:

**Arch Linux**

```shell
sudo pacman -S ansible
```

**Ubuntu**

```shell
sudo apt-add-repository ppa:ansible/ansible
sudo apt update
sudo apt install ansible -y
```

Once you have installed Ansible, you will need to install the `ansible.posix` collection. This collection provides a set of Ansible modules that are used to interact with POSIX-like systems.

You can do this by running the following command:

```shell
ansible-galaxy collection install ansible.posix
```
