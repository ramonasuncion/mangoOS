ASM       = nasm
CC        = gcc
LD        = ld
NM        = nm
OBJCOPY   = objcopy
QEMU      = qemu-system-x86_64
QEMU_FLAGS= -no-reboot -no-shutdown

BUILD     = build
OBJ       = obj
SRC       = .

BOOTLOADER_DIR ?= ../limine
LIMINE_BRANCH ?= v10.x-binary
LIMINE_URL ?= https://github.com/limine-bootloader/limine.git
LIMINE_H ?= include/limine.h
LIMINE_H_URL ?= https://raw.githubusercontent.com/limine-bootloader/limine-protocol/trunk/include/limine.h
BOOT_NAME ?= mangoos
ISO_ROOT ?= iso_root
ISO_FILE ?= $(BOOT_NAME).iso
KERNEL_ISO_NAME ?= kernel.elf

CFLAGS_COMMON = -Wall -Wextra -ffreestanding -fno-pic -fno-pie \
                -mno-red-zone -fno-stack-protector -nostdlib -nostartfiles \
                -mcmodel=kernel -Iinclude

CFLAGS_DEBUG = -O0 -g -fno-omit-frame-pointer
CFLAGS_BUILD ?= $(CFLAGS_DEBUG)
CFLAGS = $(CFLAGS_COMMON) $(CFLAGS_BUILD)

SRC_FILES = $(shell find $(SRC) -name '*.c')
ASM_FILES = $(shell find $(SRC) -name '*.asm')

OBJ_FILES = $(SRC_FILES:%.c=$(OBJ)/%.o)
ASM_OBJ_FILES = $(ASM_FILES:%.asm=$(OBJ)/%.o)
ALL_OBJ_FILES = $(OBJ_FILES) $(ASM_OBJ_FILES)

KERNEL_ELF = $(BUILD)/kernel.elf
KERNEL_BIN = $(BUILD)/kernel.bin

.PHONY: all iso run debug clean

all: $(KERNEL_ELF)

$(BUILD):
	mkdir -p $(BUILD)

$(LIMINE_H):
	curl -fsSLo $@ $(LIMINE_H_URL)

$(BOOTLOADER_DIR):
	git clone --branch=$(LIMINE_BRANCH) --depth=1 $(LIMINE_URL) $@

$(OBJ)/%.o: %.c | $(LIMINE_H)
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ)/%.o: %.asm
	@mkdir -p $(dir $@)
	$(ASM) -f elf64 $< -o $@

$(KERNEL_ELF): $(ALL_OBJ_FILES) | $(BUILD)
	$(LD) -o $@ $(ALL_OBJ_FILES) -T $(SRC)/link.ld

$(BUILD)/kernel_symbols.txt: $(KERNEL_ELF)
	@$(NM) -C $< | grep ' [Tt]' > $@

$(KERNEL_BIN): $(KERNEL_ELF)
	$(OBJCOPY) -O binary $< $@

iso: $(KERNEL_ELF) | $(BOOTLOADER_DIR)
	@BOOTLOADER_DIR="$(BOOTLOADER_DIR)" \
		BOOT_NAME="$(BOOT_NAME)" \
		ISO_ROOT="$(ISO_ROOT)" \
		ISO_FILE="$(ISO_FILE)" \
		KERNEL_ISO_NAME="$(KERNEL_ISO_NAME)" \
		KERNEL_ELF="$(KERNEL_ELF)" \
		bash ../tools/create_iso.sh

run: iso
	$(QEMU) -cdrom $(ISO_FILE) -m 512M $(QEMU_FLAGS) -serial stdio

debug: iso
	$(QEMU) -s -S -cdrom $(ISO_FILE) -m 512M $(QEMU_FLAGS) -serial stdio -d int,cpu_reset,guest_errors

clean:
	rm -rf $(BUILD) $(OBJ) $(ISO_ROOT) $(ISO_FILE)
