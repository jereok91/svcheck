# Nombre del binario
BIN_NAME := svcheck

# Ruta de salida de los binarios
OUT_DIR := dist

# Lista de targets
TARGETS := \
	x86_64-unknown-linux-gnu \
	x86_64-pc-windows-gnu \
	aarch64-unknown-linux-gnu \
	x86_64-apple-darwin \
	aarch64-apple-darwin

# DOCKER_IMAGE_X86_64 := custom-cross-openssl
# DOCKER_IMAGE_arm := custom-cross-openssl_arm
#
# .PHONY: docker
# docker:
# 	docker build -t $(DOCKER_IMAGE_X86_64):latest buils_conatiner/x86_64-unknown-linux-gnu/.
# 	docker build -t $(DOCKER_IMAGE_arm):latest buils_conatiner/aarch64-unknown-linux-gnu/.
#
build: $(TARGETS)

# .PHONY: all
all: build
	@echo "✅ Imagen docker y binarios construidos."
	@echo "✅ Todos los binarios generados en $(OUT_DIR)/"

# Regla para cada target
$(TARGETS):
	cargo clean
	rustup target add $@
	cross build --release --target $@
	mkdir -p $(OUT_DIR)/$@
	cp target/$@/release/$(BIN_NAME)$(if $(findstring windows,$@),.exe,) $(OUT_DIR)/$@/
	tar -czf $(OUT_DIR)/$@/$@.tar.gz -C $(OUT_DIR) $@/$(BIN_NAME)
	

# Limpiar binarios
clean:
	cargo clean
	rm -rf $(OUT_DIR)

.PHONY: all clean $(TARGETS)

