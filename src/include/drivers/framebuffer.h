#pragma once

#include "kernel/types.h"
#include "kernel/boot.h"

#define FB_PURPLE 0xAA00AA
#define FB_WHITE  0xFFFFFF

void fb_init(struct boot_framebuffer *fb);
void fb_clear(u32 color);
void fb_draw_char(char ch, u64 x, u64 y, u32 color);
void fb_write(const char *str, u64 x, u64 y, u32 color);
