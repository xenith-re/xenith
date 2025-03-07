# List of image names
IMAGES ?= debian12-bios

# Default target
.PHONY: all
all: build

# Build all Packer images if NAME is not specified
.PHONY: build
build: $(if $(NAME),$(NAME),$(IMAGES))

# Build a single Packer image
.PHONY: $(IMAGES)
$(IMAGES): %:
	@$(MAKE) build-image NAME=$@

# Build the specified Packer image
.PHONY: build-image
build-image:
	@echo "Building image: $(NAME)"
	@if [ -f $(NAME)/$(NAME).pkr.hcl ] && [ -f $(NAME)/variables.hcl ]; then \
		cd $(NAME) && packer build -var-file=variables.hcl $(NAME).pkr.hcl; \
	else \
		echo "Error: $(NAME).pkr.hcl or variables.hcl not found in $(NAME) directory."; \
	fi

# Clean up the output directories for all images
.PHONY: clean
clean:
	@for image in $(IMAGES); do \
		rm -rf $$image/output/; \
	done

# Display help message
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make                 Build all Packer images"
	@echo "  make clean           Remove output files for all images"
	@echo "  make help            Display this help message"
	@echo "  make NAME=image_name Build a specific Packer image"
