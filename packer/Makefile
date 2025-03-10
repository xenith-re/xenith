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
