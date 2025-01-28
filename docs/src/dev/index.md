# Setup development environment

install ansible
install vagrant
install libvirt

install vagrant-libvirt plugin, on arch linux

```shell
export VAGRANT_DISABLE_STRICT_DEPENDENCY_ENFORCEMENT=1
vagrant plugin install vagrant-libvirt vagrant-reload
```

start service

```shell
sudo systemctl start libvirtd
```

ansible-galaxy collection install ansible.posix
vagrant up
