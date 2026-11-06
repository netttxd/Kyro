"""Rebuilds kyro_capes.hmod: one sheet per cape (88x68 cells, 8 across) and the menu emblems."""
import io, math, pathlib, zipfile
import fitz
from PIL import Image

HERE = pathlib.Path(__file__).parent
HMOD = HERE.parent / 'mixins' / 'kyro' / 'data' / 'kyro_capes.hmod'
ANIM = ['black_hole', 'hyperspace', 'reactor', 'hellfire', 'glitch', 'thunderhead', 'the_watcher', 'kaleidoscope', 'blood_code', 'overdrive',
        'supernova', 'sky_dragon', 'maelstrom', 'northern_lights', 'soul_reaper', 'magma_core', 'hypnosis', 'clockwork', 'swarm', 'rift']
STILL = ['crest', 'red_crane', 'first_snow', 'kyro_sigil', 'kintsugi']
CW, CH = 88, 68


def cell(face):
    """One frame in the cape's box layout, at 4x: top, bottom, sides, outside and inside."""
    c = Image.new('RGBA', (CW, CH), (0, 0, 0, 0))
    c.paste(face, (4, 4))
    inside = face.transpose(Image.FLIP_LEFT_RIGHT).point(lambda v: int(v * .42))
    c.paste(inside, (48, 4))
    c.paste(face.crop((0, 0, 40, 1)).resize((40, 4), Image.NEAREST), (4, 0))
    c.paste(face.crop((0, 63, 40, 64)).resize((40, 4), Image.NEAREST), (44, 0))
    c.paste(face.crop((0, 0, 1, 64)).resize((4, 64), Image.NEAREST), (0, 4))
    c.paste(face.crop((39, 0, 40, 64)).resize((4, 64), Image.NEAREST), (44, 4))
    return c


def sheet(name, frames):
    strip = Image.open(HERE / 'capes2' / f'{name}.png').convert('RGB')
    cols = min(frames, 8)
    out = Image.new('RGBA', (cols * CW, (frames + cols - 1) // cols * CH), (0, 0, 0, 0))
    for f in range(frames):
        out.paste(cell(strip.crop((0, f * 64, 40, f * 64 + 64))), (f % cols * CW, f // cols * CH))
    b = io.BytesIO()
    out.save(b, 'PNG', optimize=True)
    return b.getvalue()


def pol(r, d):
    return 100 + r * math.cos(math.radians(d)), 100 + r * math.sin(math.radians(d))


def ticks(n, a, b):
    o = ''
    for i in range(n):
        d = i * 360 / n
        x, y = pol(a, d); u, v = pol(b, d)
        o += f'M{x:.1f} {y:.1f}L{u:.1f} {v:.1f}'
    return o


SWORD = '<path class="h" d="M-5 22L-5-58L0-70L5-58L5 22z"/><path d="M0-66V22"/><path class="f" d="M-17 22h34l-4 6h-26z"/><path d="M-17 22l-5-6M17 22l5-6"/><rect class="w" x="-3" y="28" width="6" height="20"/><path d="M-3 33h6M-3 38h6M-3 43h6"/><path class="f" d="M0 48l6 6-6 6-6-6z"/>'
gear = 'L'.join('L'.join(' '.join(f'{v:.1f}' for v in pol(r, i * 30 + o)) for r, o in ((35, -10), (45, -6), (45, 6), (35, 10))) for i in range(12))
CORE = {
    'hud': '<path class="h" d="M100 70l30 30-30 30-30-30z"/><path d="M100 58l42 42-42 42-42-42z"/><path d="M100 42v22M100 136v22M42 100h22M136 100h22"/><path d="M72 62H62v10M128 62h10v10M72 138H62v-10M128 138h10v-10"/><rect class="f" x="95" y="95" width="10" height="10" transform="rotate(45 100 100)"/><path d="M78 148h44M78 152h30M78 52h44M92 48h30"/>',
    'pvp': f'<g transform="translate(100 100) rotate(45) scale(.92)">{SWORD}</g><g transform="translate(100 100) rotate(-45) scale(.92)">{SWORD}</g><path class="w" d="M100 84l16 16-16 16-16-16z"/><path class="f" d="M100 92l8 8-8 8-8-8z"/>',
    'visual': f'<path class="w" d="M40 100Q100 44 160 100Q100 156 40 100z"/><circle class="h" cx="100" cy="100" r="27"/><circle cx="100" cy="100" r="27"/><path d="{ticks(36, 18, 25)}"/><circle class="f" cx="100" cy="100" r="13"/><rect class="w" x="104" y="90" width="5" height="5"/><path d="M52 82L42 70M72 66l-6-14M100 60V44M128 66l6-14M148 82l10-12M52 118l-10 12M148 118l10 12"/>',
    'client': f'<path class="h" d="M{gear}z"/><circle class="w" cx="100" cy="100" r="25"/><path d="M100 80l17 10v20l-17 10-17-10V90z"/><path class="f" d="M100 90l9 5v10l-9 5-9-5V95z"/><path d="M28 72h22l10 10M28 128h22l10-10M172 72h-22l-10 10M172 128h-22l-10-10"/><rect class="f" x="24" y="69" width="6" height="6"/><rect class="f" x="24" y="125" width="6" height="6"/><rect class="f" x="170" y="69" width="6" height="6"/><rect class="f" x="170" y="125" width="6" height="6"/>',
    'capes': '<path class="h" d="M76 62L58 150l14-7 14 9 14-9 14 9 14-9 14 7-18-88z"/><path d="M76 62L58 150l14-7 14 9 14-9 14 9 14-9 14 7-18-88"/><path d="M86 66l-8 76M100 66v76M114 66l8 76"/><path class="w" d="M72 48h56l8 10-10 6H74l-10-6z"/><path d="M78 56h44"/><path class="f" d="M100 50l7 7-7 7-7-7z"/><path class="f" d="M90 100l2 5 5 2-5 2-2 5-2-5-5-2 5-2zM112 84l2 4 4 2-4 2-2 4-2-4-4-2 4-2z"/>',
    'themes': '<path class="h" d="M100 52l44 76H56z"/><path d="M100 52l44 76H56z"/><path d="M100 76l22 40H78z"/><path d="M24 112l52-14"/><path d="M124 92l50-22M129 101l52-10M134 110l50 6M139 119l44 20"/><rect class="f" x="96" y="98" width="8" height="8"/>',
}


def emblem(core):
    core = core.replace('class="f"', 'fill="#fff" stroke="none"').replace('class="h"', 'fill="#6a6a6a" stroke="none"').replace('class="w"', 'fill="#000"')
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200"><rect width="200" height="200" fill="#000"/><g fill="none" stroke="#fff" stroke-width="1.7" stroke-linecap="square">{core}</g></svg>'
    doc = fitz.open(stream=svg.encode(), filetype='svg')
    pm = doc[0].get_pixmap(matrix=fitz.Matrix(1.6, 1.6), alpha=False)
    g = Image.frombytes('RGB', (pm.width, pm.height), pm.samples).convert('L')
    out = Image.new('RGBA', g.size, (255, 255, 255, 0))
    out.putalpha(g)
    b = io.BytesIO()
    out.save(b, 'PNG', optimize=True)
    return b.getvalue()


old = zipfile.ZipFile(HMOD)
toml = old.read('horizon-mod.toml').decode()
others = [n for n in old.namelist() if n != 'horizon-mod.toml' and not n.endswith('.png')]
print('kept', others)
kept = {n: old.read(n) for n in others}
old.close()
with zipfile.ZipFile(HMOD, 'w', zipfile.ZIP_DEFLATED) as z:
    z.writestr('horizon-mod.toml', toml)
    for n, d in kept.items():
        z.writestr(n, d)
    for n in ANIM:
        z.writestr(f'assets/kyro_capes/textures/entity/{n}.png', sheet(n, 32))
    for n in STILL:
        z.writestr(f'assets/kyro_capes/textures/entity/{n}.png', sheet(n, 1))
    for n, c in CORE.items():
        data = emblem(c)
        z.writestr(f'assets/kyro_capes/textures/entity/ui_{n}.png', data)
print(HMOD.stat().st_size // 1024, 'KB')
