# List of image names
IMAGES ?= debian12

# Default target
.PHONY: all
all: build

# Build all Packer images if NAME is not specified
.PHONY: build
build: $(if $(NAME),$(NAME),$(IMAGES))

# Build a single Packer image
.PHONY: $(IMAGES)
$(IMAGES): %:
	@$(MAKE) build-silver NAME=$@
	@$(MAKE) build-golden NAME=$@

# Build the specified Packer iso
.PHONY: build-silver
build-silver:
	@echo "Building iso: $(NAME)-silver"
	@if [ -f $(NAME)/$(NAME)-silver.pkr.hcl ] && [ -f $(NAME)/variables.hcl ]; then \
		cd $(NAME) && packer build -var-file=variables.hcl $(NAME)-silver.pkr.hcl; \
	else \
		echo "Error: $(NAME)-silver.pkr.hcl or variables.hcl not found in $(NAME) directory."; \
	fi

# Build the specified Packer image
.PHONY: build-golden
build-golden:
	@echo "Building image: $(NAME)-golden"
	@if [ -f $(NAME)/$(NAME)-golden.pkr.hcl ] && [ -f $(NAME)/variables.hcl ]; then \
		cd $(NAME) && packer build -var-file=variables.hcl $(NAME)-golden.pkr.hcl; \
	else \
		echo "Error: $(NAME)-golden.pkr.hcl or variables.hcl not found in $(NAME) directory."; \
	fi

# Run the specified Packer iso with QEMU
.PHONY: run-silver
run-silver:
	@echo "Running iso: $(NAME)-silver"
	@if [ -f $(NAME)/build/silver/$(NAME)-silver.qcow2 ]; then \
		qemu-system-x86_64 \
			-drive file=$(NAME)/build/silver/$(NAME)-silver.qcow2,format=qcow2 \
			-m 4G \
			-netdev user,id=net0 \
			-device virtio-net-pci,netdev=net0 \
			-smp sockets=1,cores=4,threads=4; \
	else \
		echo "Error: $(NAME)-silver.qcow2 not found in $(NAME)/build/silver directory."; \
	fi

# Run the specified Packer image with QEMU
.PHONY: run-golden
run-golden:
	@echo "Running image: $(NAME)-golden"
	@if [ -f $(NAME)/build/golden/$(NAME)-golden.qcow2 ]; then \
		qemu-system-x86_64 \
			-drive file=$(NAME)/build/golden/$(NAME)-golden.qcow2,format=qcow2 \
			-enable-kvm \
			-m 4G \
			-netdev user,id=net0 \
			-device virtio-net-pci,netdev=net0 \
			-smp sockets=1,cores=4,threads=4; \
	else \
		echo "Error: $(NAME)-golden.qcow2 not found in $(NAME)/build/golden directory."; \
	fi


# Clean up the output directories for all images
.PHONY: clean
clean:
	@for image in $(IMAGES); do \
		rm -rf $$image/build/; \
	done

# Display help message
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make                 Build all Packer images"
	@echo "  make clean           Remove build files for all images"
	@echo "  make help            Display this help message"
	@echo "  make NAME=image_name Build a specific Packer image"
