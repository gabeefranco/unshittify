#!/usr/bin/env python3
"""Check the contrast of an unshittified app's palette.

Usage: python3 contrast.py path/to/ui/style.css

Reads the color variables from each `:root { ... }` block (the light block and,
if present, the dark one inside `@media (prefers-color-scheme: dark)`) and
checks text colors against their background using the WCAG 2 formula.
Exits 1 if any pair fails. Colors must be hex (#rgb or #rrggbb) or rgb().
"""

import re
import sys

# (foreground var, background var, minimum ratio, what it is)
PAIRS = [
    ("--fg", "--bg", 4.5, "body text"),
    ("--link", "--bg", 4.5, "links and [actions]"),
    ("--visited", "--bg", 4.5, "visited links"),
    ("--muted", "--bg", 3.0, "muted text / status line"),
    ("--fg", "--flash", 4.5, "flash message text"),
]


def parse_color(value):
    value = value.strip()
    m = re.fullmatch(r"#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})", value)
    if m:
        h = m.group(1)
        if len(h) == 3:
            h = "".join(c * 2 for c in h)
        return tuple(int(h[i : i + 2], 16) for i in (0, 2, 4))
    m = re.fullmatch(r"rgba?\(\s*(\d+)[\s,]+(\d+)[\s,]+(\d+).*\)", value)
    if m:
        return tuple(int(g) for g in m.groups())
    return None


def luminance(rgb):
    def channel(c):
        c /= 255
        return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4

    r, g, b = (channel(c) for c in rgb)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def ratio(a, b):
    la, lb = sorted((luminance(a), luminance(b)), reverse=True)
    return (la + 0.05) / (lb + 0.05)


def blocks(css):
    """Yield (label, vars) for each :root block, light first."""
    css = re.sub(r"/\*.*?\*/", "", css, flags=re.S)
    dark = re.search(r"@media\s*\(\s*prefers-color-scheme:\s*dark\s*\)\s*\{", css)
    for m in re.finditer(r":root\s*\{([^}]*)\}", css):
        label = "dark (system)" if dark and m.start() > dark.start() else "main"
        yield label, dict(re.findall(r"(--[\w-]+)\s*:\s*([^;]+);", m.group(1)))


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    css = open(sys.argv[1], encoding="utf-8").read()
    failed = False
    base = {}
    for label, found in blocks(css):
        palette = {**base, **found}  # the dark block only overrides some vars
        if label == "main":
            base = palette
        print(f"[{label}]")
        for fg, bg, minimum, what in PAIRS:
            a, b = parse_color(palette.get(fg, "")), parse_color(palette.get(bg, ""))
            if a is None or b is None:
                print(f"  ?     {fg} on {bg}: not a hex/rgb color, skipped ({what})")
                continue
            r = ratio(a, b)
            ok = r >= minimum
            failed |= not ok
            mark = "ok  " if ok else "FAIL"
            print(f"  {mark}  {r:5.2f}:1 (min {minimum}) {fg} {palette[fg].strip()} on {bg} {palette[bg].strip()}  {what}")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
