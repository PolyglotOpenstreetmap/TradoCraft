#!/usr/bin/env python3
"""Generate the tradocraft app icon for every platform from one glyph definition.

Concept: four module bars rising as one platform, each plugged into a single
connected flow line (nodes + arrow) -- modules working as one, getting faster.

Usage:  python3 -I scripts/gen_app_icons.py        (needs inkscape + imagemagick)
Outputs are written into app/ (android, web, windows) and app/assets/icons/src/.
"""
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
APP = ROOT / "app"
SRC = APP / "assets" / "icons" / "src"

BG_TOP, BG_MID, BG_BOT = "#2BA8A3", "#1F7FA0", "#1A4F85"
WHITE, FLOW, NODE_FILL = "#FFFFFF", "#FFD166", "#1B6A93"

# Glyph centre offset so the artwork sits in the optical middle of the canvas.
CX, CY = 8, 9


def glyph(mode: str) -> str:
    """mode: full | small | mono. Coordinates are around (0,0), fits radius ~265."""
    mono = mode == "mono"
    small = mode == "small"
    flow = WHITE if mono else FLOW
    node_fill = "none" if mono else NODE_FILL
    bar_w, pitch = (72, 96) if small else (64, 92)
    x0 = -(pitch * 3) // 2
    heights = [80, 130, 180, 230]
    base = 160
    parts = []
    if not small:
        parts.append(f'<rect x="-190" y="156" width="380" height="28" rx="14" fill="{WHITE}"/>')
    centres = []
    for i, h in enumerate(heights):
        left = x0 + i * pitch - bar_w // 2
        cx = x0 + i * pitch
        top = base - h
        parts.append(
            f'<rect x="{left}" y="{top}" width="{bar_w}" height="{h + 14}" rx="14" fill="{WHITE}"/>'
        )
        centres.append((cx, top - 46))
        if not small:
            parts.append(
                f'<path d="M{cx} {top - 4}V{top - 46 + 22}" stroke="{flow}" stroke-width="9" '
                f'stroke-linecap="round" fill="none"/>'
            )
    sw = 20 if small else 14
    pts = " ".join(f"{x},{y}" for x, y in centres)
    lx, ly = centres[-1]
    ex, ey = lx + 62, ly - 33
    parts.append(
        f'<polyline points="{pts} {ex},{ey}" fill="none" stroke="{flow}" stroke-width="{sw}" '
        f'stroke-linecap="round" stroke-linejoin="round"/>'
    )
    parts.append(
        f'<path d="M{ex - 43} {ey - 9}L{ex} {ey}L{ex - 16} {ey + 41}" fill="none" stroke="{flow}" '
        f'stroke-width="{sw}" stroke-linecap="round" stroke-linejoin="round"/>'
    )
    r, nsw = (24, 11) if small else (20, 10)
    for x, y in centres:
        parts.append(
            f'<circle cx="{x}" cy="{y}" r="{r}" fill="{node_fill}" stroke="{flow}" stroke-width="{nsw}"/>'
        )
    return "".join(parts)


def svg(size: int, *, shape: str, scale: float, mode: str = "full", bg: bool = True) -> str:
    """shape: square | rounded.  Canvas is always laid out on a 1024 viewBox."""
    defs = (
        f'<defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1">'
        f'<stop offset="0" stop-color="{BG_TOP}"/><stop offset=".55" stop-color="{BG_MID}"/>'
        f'<stop offset="1" stop-color="{BG_BOT}"/></linearGradient>'
        f'<radialGradient id="h" cx=".25" cy=".2" r=".8"><stop offset="0" stop-color="#fff" stop-opacity=".16"/>'
        f'<stop offset="1" stop-color="#fff" stop-opacity="0"/></radialGradient></defs>'
    )
    body = ""
    if bg:
        rx = 229 if shape == "rounded" else 0
        body += f'<rect width="1024" height="1024" rx="{rx}" fill="url(#g)"/>'
        body += f'<rect width="1024" height="1024" rx="{rx}" fill="url(#h)"/>'
    body += f'<g transform="translate(512 512) scale({scale}) translate({-CX} {-CY})">{glyph(mode)}</g>'
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 1024 1024">'
        f'{defs}{body}</svg>'
    )


def render(svg_text: str, out: Path, px: int, tmp: Path) -> None:
    out.parent.mkdir(parents=True, exist_ok=True)
    f = tmp / f"{out.stem}_{px}.svg"
    f.write_text(svg_text)
    subprocess.run(
        ["inkscape", str(f), "-w", str(px), "-h", str(px), "-o", str(out)],
        check=True, capture_output=True,
    )


def main() -> int:
    for tool in ("inkscape", "convert"):
        if not shutil.which(tool):
            print(f"missing tool: {tool}", file=sys.stderr)
            return 1
    SRC.mkdir(parents=True, exist_ok=True)
    tmp = Path(tempfile.mkdtemp())
    res = APP / "android/app/src/main/res"

    # --- Source SVGs kept in the repo ---------------------------------------
    rounded = lambda px, mode="full": svg(px, shape="rounded", scale=1.3, mode=mode)  # noqa: E731
    (SRC / "app_icon.svg").write_text(rounded(1024))
    (SRC / "app_icon_square.svg").write_text(svg(1024, shape="square", scale=1.3))
    (SRC / "app_icon_foreground.svg").write_text(svg(1024, shape="square", scale=1.05, bg=False))
    (SRC / "app_icon_monochrome.svg").write_text(
        svg(1024, shape="square", scale=1.05, mode="mono", bg=False)
    )

    # --- Master PNGs ----------------------------------------------------------
    render(rounded(1024), APP / "assets/icons/app_icon.png", 256, tmp)
    # iOS / macOS marketing icon: opaque, square, the OS applies the mask.
    render(svg(1024, shape="square", scale=1.3), SRC / "app_icon_1024_opaque.png", 1024, tmp)

    # --- Android: legacy + adaptive + themed (monochrome) ---------------------
    legacy = {"mdpi": 48, "hdpi": 72, "xhdpi": 96, "xxhdpi": 144, "xxxhdpi": 192}
    layer = {"mdpi": 108, "hdpi": 162, "xhdpi": 216, "xxhdpi": 324, "xxxhdpi": 432}
    fg = svg(1024, shape="square", scale=1.05, bg=False)
    mono = svg(1024, shape="square", scale=1.05, mode="mono", bg=False)
    for d, px in legacy.items():
        render(rounded(1024), res / f"mipmap-{d}/ic_launcher.png", px, tmp)
    for d, px in layer.items():
        render(fg, res / f"mipmap-{d}/ic_launcher_foreground.png", px, tmp)
        render(mono, res / f"mipmap-{d}/ic_launcher_monochrome.png", px, tmp)
    (res / "mipmap-anydpi-v26").mkdir(exist_ok=True)
    (res / "mipmap-anydpi-v26/ic_launcher.xml").write_text(
        '<?xml version="1.0" encoding="utf-8"?>\n'
        '<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">\n'
        '    <background android:drawable="@drawable/ic_launcher_background"/>\n'
        '    <foreground android:drawable="@mipmap/ic_launcher_foreground"/>\n'
        '    <monochrome android:drawable="@mipmap/ic_launcher_monochrome"/>\n'
        '</adaptive-icon>\n'
    )
    (res / "drawable/ic_launcher_background.xml").write_text(
        '<?xml version="1.0" encoding="utf-8"?>\n'
        '<shape xmlns:android="http://schemas.android.com/apk/res/android" android:shape="rectangle">\n'
        f'    <gradient android:type="linear" android:angle="315" android:startColor="{BG_TOP}" '
        f'android:centerColor="{BG_MID}" android:endColor="{BG_BOT}"/>\n'
        '</shape>\n'
    )

    # --- Web / PWA -------------------------------------------------------------
    web = APP / "web"
    for px in (192, 512):
        render(rounded(1024), web / f"icons/Icon-{px}.png", px, tmp)
        render(svg(1024, shape="square", scale=1.3), web / f"icons/Icon-maskable-{px}.png", px, tmp)
    render(svg(1024, shape="rounded", scale=1.4, mode="small"), web / "favicon.png", 32, tmp)

    # --- Windows .ico (small sizes use the simplified glyph) -------------------
    sizes = [16, 24, 32, 48, 64, 128, 256]
    pngs = []
    for px in sizes:
        mode, sc = ("small", 1.4) if px <= 48 else ("full", 1.3)
        p = tmp / f"ico_{px}.png"
        render(svg(1024, shape="rounded", scale=sc, mode=mode), p, px, tmp)
        pngs.append(str(p))
    subprocess.run(
        ["convert", *pngs, str(APP / "windows/runner/resources/app_icon.ico")], check=True
    )
    shutil.rmtree(tmp)
    print("icons generated")
    return 0


if __name__ == "__main__":
    sys.exit(main())
