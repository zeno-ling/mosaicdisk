"""从 assets/icon.svg 的设计参数生成 Windows 多尺寸图标 assets/app_icon.ico。"""
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "assets" / "app_icon.ico"

SCALE = 8  # 512x512 超采样后缩小
SIZES = [16, 24, 32, 48, 64, 128, 256]

# icon.svg 中基于 64x64 的几何参数（坐标按比例缩放）
BG = (0x2A, 0x2A, 0x34)
PURPLE = (0x7C, 0x4D, 0xFF)
PINK = (0xFF, 0x4D, 0x8A)
TEAL = (0x00, 0xD4, 0xAA)


def draw_icon(size: int) -> Image.Image:
    s = size * SCALE
    img = Image.new("RGBA", (s, s), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    box = lambda x0, y0, x1, y1: (x0 * s // 64, y0 * s // 64, x1 * s // 64, y1 * s // 64)
    radius = lambda r: r * s // 64
    d.rounded_rectangle(box(0, 0, 64, 64), radius=radius(14), fill=BG)
    d.rounded_rectangle(box(9, 9, 37, 55), radius=radius(4), fill=PURPLE)
    d.rounded_rectangle(box(39, 9, 55, 33), radius=radius(4), fill=PINK)
    d.rounded_rectangle(box(39, 35, 55, 55), radius=radius(4), fill=TEAL)
    return img.resize((size, size), Image.LANCZOS)


frames = [draw_icon(sz) for sz in SIZES]
frames[0].save(
    OUT,
    format="ICO",
    sizes=[(sz, sz) for sz in SIZES],
    append_images=frames[1:],
)
print(f"生成 {OUT} ({OUT.stat().st_size} 字节, 尺寸 {SIZES})")
