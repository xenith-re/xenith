---
title: FAQ
type: docs
weight: 100
---

This document is a collection of decisions made during the development of Xenith and the reasoning behind them.

## Why use Xen ?

Xen is a high-performance, open-source hypervisor that is widely used in the industry. It is a mature project with a large community and a lot of documentation. Xen is also the hypervisor used by Amazon Web Services for their EC2 instances, that proves its reliability and scalability.

It has to this day one of the best support of virtual machine introspection, which was the primary goal of this project.

Also, its architecture is very modular and allows for a lot of customization, which is a good fit for a project like Xenith. For example we can choose the type of virtualization to use (HVM, PV, PVH), the type of device model to use (QEMU, MiniOS, etc.), the type of backend to use (QEMU, MiniOS, etc.), etc.

Its dom0 architecture is also very interesting, as it allows to run a minimalistic Linux distribution in a VM, which is very useful and powerful to us so we can build our introspection tools on top of it, and its even possible to have a dedicated GUI !
