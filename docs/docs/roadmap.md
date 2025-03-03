---
title: Roadmap
type: docs
weight: 1
---

Xenith aims to provide a powerful and stealthy hypervisor debugger environment for researchers and developers. This is not an easy task, and we are working hard to make it happen.

The following objectives outline the overarching goals and aspirations for the project :

- Robust and reliable hypervisor debugger environment
- Easy to use and understand (from domain management to debugging)
- Stealthy and hard to detect
- Compatible with multiple debuggers
- Compatible with Linux and Windows guest domains
- Automated tasks and workflows
- Scriptable with Rust and Python
- Extensible and modular, allowing for easy integration of new features
- Well-documented and easy to contribute to

{{% steps %}}

### v0.1.0 - Foundations

<div class="hx-mt-2"></div>
{{< badge content="Current version" type="info" icon="information-circle" >}}

This milestone builds the foundation for the project.

{{% details title="Details" closed="true" %}}

- [ ] Setup proper development environment
  - [x] Setup project structure
  - [x] Setup CI/CD pipeline
    - [x] Automated code testing (unit & integration, formatting, linting, code coverage)
    - [x] Automated documentation generation and deployment
  - [x] Add Xen upstream source tree as a submodule (wil allow us to easily update Xen)
  - [x] Add Depandabot to keep dependencies up to date
  - [ ] Setup Github branch protections
  - [x] Setup Vagrantfile for development environment
    - [x] Add custom Vagrant command for connecting graphically to the dom0
    - [x] Automated provisioning of the Debian dom0 through Ansible
- [x] Create a clean diataxis documentation
- [ ] Create base crates (without any functionality) and workspace
  - [x] Project workspace
  - [ ] `xenith-core` - Core functionality, shared between all other crates
  - [x] `xenith-cli` - Command line interface
  - [x] `xenith-vm` - Xen domain management
  - [x] `xenith-vmi` - [Virtual Machine Introspection](../reference/vmi) (VMI) wrappers
  - [ ] `xenith-debugger` - [Debugger](../reference/debugger) interface
  - [ ] `xenith-scripting` - Scripting interface
  - [x] `xenith-redpill` - Automated testing of virtual machines detection techniques ([redpills](../reference/redpill))
  - [ ] `xenith-gui` - Graphical user interface
- [ ] Package Xenith tools for Debian dom0
- [ ] Automated domU Linux and Windows image creation through Packer and Ansible

{{% /details %}}

### v0.2.0 - Domain Management

This milestone focuses on the domain management of Xenith, adding basic functionality to interact with the hypervisor and manage domains.

{{% details title="Details" closed="true" %}}

First, we will wrap the `xl` command to manage domains, and in the future we will use the proper `libxl` bindings to enhance functionalities, but those are to be created (see [xenith-re/libxl-sys](https://github.com/xenith-re/libxl-sys/) for raw bindings and [xenith-re/libxl](https://github.com/xenith-re/libxl) for a safe wrapper).

In `xenith-vm` crate:

- [ ] Disk managing
  - [ ] Create disk
  - [ ] Delete disk
  - [ ] Resize disk
- [ ] Domain managing
  - [ ] Create
  - [ ] Delete
  - [ ] Start
  - [ ] Stop
  - [ ] Pause
  - [ ] Continue
  - [ ] List domains
  - [ ] Get domain information (state, memory, CPU, network, …)
- [ ] Snapshot managing
  - [ ] Create snapshot
  - [ ] Delete snapshot
  - [ ] Restore snapshot
- [ ] Configuration managing (defaults to `/xenith` to manage everything)
  - [ ] Get set configuration path
  - [ ] Store domain configuration, image, snapshots, disks, …

{{% /details %}}

### v0.3.0 - CLI

This milestone focuses on the command-line interface (CLI), providing a powerful and easy-to-use interface to interact with the hypervisor and manage domains.

{{% details title="Details" closed="true" %}}

- [ ] Expose all `xenith-vm` functionalities
- [ ] Provide multiple ways to connect to domU
  - [ ] SSH
  - [ ] WinRM
  - [ ] VNC
  - [ ] SDL

{{% /details %}}

### v0.4.0 - Virtual Machine Introspection

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone aims to provide [Virtual Machine Introspection](../reference/vmi) (VMI) capabilities to Xenith, allowing users to inspect and edit the memory and CPU state of a virtual machine with [semantic context](../reference/semantig-gap).

{{% details title="Details" closed="true" %}}

- [ ] To be defined

{{% /details %}}

### v0.5.0 - Debugger

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone focuses on the [debugger](../reference/debugger) [VMI](../reference/vmi)  interface, allowing users to debug their [domains](../reference/domain) with their favorite debugger, such as GDB, LLDB and WinDbg.

{{% details title="Details" closed="true" %}}

- [ ] To be defined

{{% /details %}}

### v0.6.0 - Stealth

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone focuses on making Xenith stealthy and hard to detect, allowing users to perform their research and development without being detected by the target system. This will include creating [redpills](../reference/redpill) to detect virtual machines and implementing countermeasures to avoid detection.

{{% details title="Details" closed="true" %}}

- [ ] Modify `cpuid` to edit common requested values
- [ ] Hook MSR reads and writes
- [ ] Hook `rdtsc` instruction
- [ ] Automated code testing in guest domains (for redpills)
- [ ] Others to be defined

{{% /details %}}

### v0.7.0 - Scripting

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone provides a scripting interface, allowing users to automate tasks and workflows with Rust and Python. This will allow users to easily integrate the project with their existing tools and workflows.

{{% details title="Details" closed="true" %}}

- [ ] To be defined

{{% /details %}}

### v0.8.0 - Plugin system

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone focuses on the extensibility of Xenith, allowing users to easily integrate new features and functionality into the project. This will allow users to customize the project to suit their needs and requirements.

{{% details title="Details" closed="true" %}}

- [ ] To be defined

{{% /details %}}

### v0.9.0 - Automated tasks and workflows

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone focuses on automating tasks and workflows within Xenith, allowing users to easily perform common tasks and workflows with minimal effort. This will allow users to focus on their research and development, rather than the administrative tasks associated with managing and debugging virtual machines.

This will notably include creating plugins for common tasks and workflows, specific to supported operating systems.

{{% details title="Details" closed="true" %}}

- [ ] To be defined

{{% /details %}}

### v0.10.0 - Graphical User Interface

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone focuses on providing a graphical user interface (GUI), allowing users to interact with the hypervisor and manage domains with a visual interface. This will provide an alternative to the command-line interface, and make it easier for users to perform common tasks and workflows.

{{% details title="Details" closed="true" %}}

- [ ] To be defined

{{% /details %}}

{{% /steps %}}
