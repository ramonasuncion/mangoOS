#include "drivers/framebuffer.h"
#include "drivers/font8x8_basic.h"

static struct boot_framebuffer *screen;

static void fb_put_pixel(u64 x, u64 y, u32 color)
{
  u32 *row = (u32 *)(screen->address + y * screen->pitch);
  row[x] = color;
}

void fb_init(struct boot_framebuffer *fb)
{
  screen = fb;
}

void fb_clear(u32 color)
{
  for (u64 y = 0; y < screen->height; y++) {
    for (u64 x = 0; x < screen->width; x++) {
      fb_put_pixel(x, y, color);
    }
  }
}

void fb_draw_char(char ch, u64 x, u64 y, u32 color)
{
  const u8 *glyph = font8x8_basic[ch & 0x7F];

  for (u64 row = 0; row < 8; row++) {
    for (u64 col = 0; col < 8; col++) {
      if (glyph[row] & (1 << col)) {
        fb_put_pixel(x + col, y + row, color);
      }
    }
  }
}

void fb_write(const char *str, u64 x, u64 y, u32 color)
{
  for (u64 i = 0; str[i] != '\0'; i++) {
    fb_draw_char(str[i], x + i * 8, y, color);
  }
}
