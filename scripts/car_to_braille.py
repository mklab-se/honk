#!/usr/bin/env python3
"""Turn media/car-lineart.png into the Braille car honk draws in the terminal.

Each output character is a 2x4 grid of Braille dots, so a 60-column car gets
120x64 "pixels" of detail. Lines are thickened a little before downscaling so
they survive. Every row is padded to the same width so the animation can
redraw in place.

Usage (needs Pillow):
    python3 scripts/car_to_braille.py media/car-lineart.png 60 > crates/honk/assets/car-braille.txt
"""

import sys

from PIL import Image, ImageFilter, ImageOps

# Dot bit for each (x, y) position inside a 2x4 Braille cell (Unicode order).
DOTS = [(0, 0, 0x01), (0, 1, 0x02), (0, 2, 0x04), (1, 0, 0x08),
        (1, 1, 0x10), (1, 2, 0x20), (0, 3, 0x40), (1, 3, 0x80)]
THICKEN = 3      # MinFilter size: widens dark lines before downscaling
THRESHOLD = 150  # grey level below which a dot is set


def braille(path: str, cols: int) -> list[str]:
    img = Image.open(path).convert("L")
    bbox = ImageOps.invert(img).point(lambda p: 255 if p > 40 else 0).getbbox()
    img = img.crop(bbox).filter(ImageFilter.MinFilter(THICKEN))
    w, h = img.size
    pw = cols * 2
    ph = round(pw * h / w / 4) * 4
    small = img.resize((pw, ph), Image.LANCZOS)
    rows = []
    for cy in range(ph // 4):
        row = ""
        for cx in range(cols):
            bits = 0
            for dx, dy, bit in DOTS:
                if small.getpixel((cx * 2 + dx, cy * 4 + dy)) < THRESHOLD:
                    bits |= bit
            row += chr(0x2800 + bits) if bits else " "
        rows.append(row)
    # Drop fully blank rows at the top and bottom.
    while rows and not rows[0].strip():
        rows.pop(0)
    while rows and not rows[-1].strip():
        rows.pop()
    return rows


if __name__ == "__main__":
    print("\n".join(braille(sys.argv[1], int(sys.argv[2]))))
