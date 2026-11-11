#!/usr/bin/env python3
"""Generate the platform launcher icons from the brand SVGs in app/assets/branding/.

Sources:  app_icon.svg (rounded), app_icon_maskable.svg (full-bleed, glyph in the safe zone).
Outputs:  Android (legacy + adaptive + themed monochrome), web/PWA icons and favicon, Windows .ico.
Usage:    python3 -I scripts/gen_app_icons.py     (needs inkscape and imagemagick)
"""
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

APP = Path(__file__).resolve().parent.parent / "app"
BRAND = APP / "assets" / "branding"
RES = APP / "android/app/src/main/res"
BG_START, BG_END = "#3B4767", "#5E6784"  # same as the brand SVG background gradient


def render(svg: Path, out: Path, px: int) -> None:
    out.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(["inkscape", str(svg), "-w", str(px), "-h", str(px), "-o", str(out)],
                   check=True, capture_output=True)


def adaptive_layers(tmp: Path) -> tuple[Path, Path]:
    """Foreground (glyph only, centred, inside the 66dp safe circle) and a white monochrome copy."""
    s = 0.75
    # Glyph bounding box in its own coordinates is centred on (256, 268.5).
    tf = f'transform="translate({256 - s * 256:.2f} {256 - s * 268.5:.2f}) scale({s})"'
    src = (BRAND / "app_icon_maskable.svg").read_text()
    src = re.sub(r'<rect width="512" height="512" fill="url\(#bg\)"/>\n?', "", src)
    fg = re.sub(r'<g transform="[^"]*"', f"<g {tf}", src, count=1)
    mono = fg.replace("url(#gold)", "#FFFFFF")
    paths = []
    for name, text in (("fg", fg), ("mono", mono)):
        p = tmp / f"{name}.svg"
        p.write_text(text)
        paths.append(p)
    return paths[0], paths[1]


def main() -> int:
    for tool in ("inkscape", "convert"):
        if not shutil.which(tool):
            print(f"missing tool: {tool}", file=sys.stderr)
            return 1
    rounded, maskable = BRAND / "app_icon.svg", BRAND / "app_icon_maskable.svg"
    tmp = Path(tempfile.mkdtemp())

    # Android
    legacy = {"mdpi": 48, "hdpi": 72, "xhdpi": 96, "xxhdpi": 144, "xxxhdpi": 192}
    layer = {"mdpi": 108, "hdpi": 162, "xhdpi": 216, "xxhdpi": 324, "xxxhdpi": 432}
    fg, mono = adaptive_layers(tmp)
    for d, px in legacy.items():
        render(rounded, RES / f"mipmap-{d}/ic_launcher.png", px)
    for d, px in layer.items():
        render(fg, RES / f"mipmap-{d}/ic_launcher_foreground.png", px)
        render(mono, RES / f"mipmap-{d}/ic_launcher_monochrome.png", px)
    (RES / "mipmap-anydpi-v26").mkdir(exist_ok=True)
    (RES / "mipmap-anydpi-v26/ic_launcher.xml").write_text(
        '<?xml version="1.0" encoding="utf-8"?>\n'
        '<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">\n'
        '    <background android:drawable="@drawable/ic_launcher_background"/>\n'
        '    <foreground android:drawable="@mipmap/ic_launcher_foreground"/>\n'
        '    <monochrome android:drawable="@mipmap/ic_launcher_monochrome"/>\n'
        '</adaptive-icon>\n')
    (RES / "drawable/ic_launcher_background.xml").write_text(
        '<?xml version="1.0" encoding="utf-8"?>\n'
        '<shape xmlns:android="http://schemas.android.com/apk/res/android" android:shape="rectangle">\n'
        f'    <gradient android:type="linear" android:angle="315" android:startColor="{BG_START}" '
        f'android:endColor="{BG_END}"/>\n</shape>\n')

    # Web / PWA
    web = APP / "web"
    for px in (192, 512):
        render(rounded, web / f"icons/Icon-{px}.png", px)
        render(maskable, web / f"icons/Icon-maskable-{px}.png", px)
    render(rounded, web / "favicon.png", 32)

    # Windows
    sizes = [16, 24, 32, 48, 64, 128, 256]
    pngs = []
    for px in sizes:
        p = tmp / f"ico_{px}.png"
        render(rounded, p, px)
        pngs.append(str(p))
    subprocess.run(["convert", *pngs, str(APP / "windows/runner/resources/app_icon.ico")], check=True)
    shutil.rmtree(tmp)
    print("icons generated")
    return 0


if __name__ == "__main__":
    sys.exit(main())
