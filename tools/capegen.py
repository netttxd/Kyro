"""Bakes the 15 Kyro capes as vertical PNG strips (40x64 per frame). Every loop is seamless."""
import math, pathlib, random, sys
import numpy as np
from PIL import Image, ImageDraw

W, H, T = 40, 64, 32
OUT = pathlib.Path(__file__).parent / 'capes2'
OUT.mkdir(exist_ok=True)
Y, X = np.mgrid[0:H, 0:W].astype(float)
TAU = math.tau
B4 = np.array([[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]) / 16 - .47
BAYER = np.tile(B4, (H // 4, W // 4))


def hexes(*c):
    return np.array([[int(h[i:i + 2], 16) for i in (0, 2, 4)] for h in c], float)


def ramp(v, pal, dither=1.0):
    n = len(pal) - 1
    i = np.clip(np.floor(np.clip(v, 0, 1) * n + .5 + BAYER * dither), 0, n).astype(int)
    return pal[i]


def wave(t, seed, n=5, sx=1.0, sy=1.0, spin=1):
    """Smooth noise whose time term has whole cycles, so it loops."""
    r = random.Random(seed)
    o = np.zeros((H, W))
    for k in range(n):
        a, b = r.uniform(-.5, .5) * sx, r.uniform(-.5, .5) * sy
        o += np.sin(a * X + b * Y + TAU * t * r.choice([1, 2, -1, -2]) * spin + r.uniform(0, TAU))
    return o / n


def line(a, x0, y0, x1, y1, v):
    n = int(max(abs(x1 - x0), abs(y1 - y0))) + 1
    for i in range(n + 1):
        x, y = int(x0 + (x1 - x0) * i / n), int(y0 + (y1 - y0) * i / n)
        if 0 <= x < W and 0 <= y < H:
            a[y, x] = max(a[y, x], v)


def frame_edge(img, col, t=None, glow=None):
    img[[0, -1], :] = col
    img[:, [0, -1]] = col
    if t is not None:  # light runs round the edge
        path = [(x, 0) for x in range(W)] + [(W - 1, y) for y in range(H)] + [(x, H - 1) for x in range(W - 1, -1, -1)] + [(0, y) for y in range(H - 1, -1, -1)]
        for s in (0, .5):
            for k in range(14):
                x, y = path[int(((t + s) % 1) * len(path) - k) % len(path)]
                img[y, x] = glow * (1 - k / 16) + col * (k / 16)
    return img


def bolt(a, r, x, y, tx, ty, v, branch=True):
    while y < ty:
        nx, ny = x + r.randint(-3, 3) + (tx - x) * .12, y + r.randint(2, 4)
        line(a, x, y, nx, ny, v)
        if branch and r.random() < .22:
            bolt(a, r, nx, ny, nx + r.randint(-14, 14), ny + r.randint(6, 16), v * .7, False)
        x, y = nx, ny


K = ["XX....XX", "XX...XX.", "XX..XX..", "XX.XX...", "XXXX....", "XXXX....", "XX.XX...", "XX..XX..", "XX...XX.", "XX....XX"]


def kmask(scale=3, cx=20, cy=32):
    m = np.zeros((H, W), bool)
    ox, oy = cx - 4 * scale, cy - 5 * scale
    for j, row in enumerate(K):
        for i, c in enumerate(row):
            if c == 'X':
                m[oy + j * scale:oy + (j + 1) * scale, ox + i * scale:ox + (i + 1) * scale] = True
    return m


def black_hole(t):
    pal = hexes('05030c', '1a0833', '4a0f5e', 'a01a58', 'e8531f', 'ffb52e', 'fff2c0', 'ffffff')
    cx, cy = 19.5, 27
    u, v = X - cx, Y - cy
    a = -.5
    du, dv = u * math.cos(a) - v * math.sin(a), (u * math.sin(a) + v * math.cos(a)) * 3
    r, th = np.hypot(du, dv) + .01, np.arctan2(dv, du)
    spiral = np.sin(3 * th + 7 * np.log(r) + TAU * 3 * t) * .5 + .5
    disk = np.exp(-((r - 19) / 11) ** 2) * (.35 + .75 * spiral)
    doppler = 1 + .55 * np.cos(th + .6)
    rr = np.hypot(u, v)
    back = np.exp(-((rr - 12.5) / 2.3) ** 2) * (.75 + .25 * np.sin(5 * np.arctan2(v, u) - TAU * 2 * t))
    val = np.maximum(disk * doppler, back)
    val = np.where((rr < 10.5), 0, val)
    val = np.where((rr < 12) & (v > 2) & (disk > .3), disk * doppler, val)
    r0 = random.Random(5)
    s = np.zeros((H, W))
    for i in range(46):  # matter falling in on a spiral
        p, a0, sp = r0.random(), r0.uniform(0, TAU), r0.choice([1, 2])
        z = (p + sp * t) % 1
        rad, ang = 46 * (1 - z) ** 1.4 + 11, a0 + 5 * z
        x, y = cx + rad * math.cos(ang), cy + rad * math.sin(ang) * .9 + 4
        line(s, x, y, x - 2 * math.sin(ang), y + 2 * math.cos(ang), .35 + .6 * z)
    return ramp(np.maximum(val, s), pal)


def warp(t):
    pal = hexes('02030f', '0a1a55', '1553c9', '20b4ff', '9df3ff', 'ffffff')
    a = np.zeros((H, W))
    r = random.Random(9)
    cx = 19.5 + 5 * math.sin(TAU * t)
    cy = 30 + 6 * math.cos(TAU * t)
    for i in range(150):
        ang, p, sp = r.uniform(0, TAU), r.random(), r.choice([1, 2, 3])
        z = (p + sp * t) % 1
        r0, r1 = 60 * z ** 2.6, 60 * z ** 2.6 * 1.5 + 2
        line(a, cx + r0 * math.cos(ang), cy + r0 * math.sin(ang), cx + r1 * math.cos(ang), cy + r1 * math.sin(ang), .25 + z * .85)
    d = np.hypot(X - cx, Y - cy)
    tunnel = (np.sin(d * .9 - TAU * 4 * t) * .5 + .5) * .24 * np.exp(-d / 26)
    core = np.exp(-(d / (3.2 + 1.2 * math.sin(TAU * 4 * t))) ** 2) * 1.2
    return ramp(np.maximum.reduce([a, tunnel, core]), pal)


def reactor(t):
    pal = hexes('03060c', '06303a', '087a78', '19e0b0', 'b0ffe0', 'ffffff')
    hot = hexes('ff2bd0')[0]
    cx, cy = 19.5, 31.5
    u, v = X - cx, Y - cy
    r, th = np.hypot(u, v), np.arctan2(v, u)
    val = np.zeros((H, W))
    for rad, seg, sp, w in ((27, 6, 1, 1.5), (21, 10, -2, 1.3), (15, 4, 3, 1.5), (10, 12, -4, 1)):
        ring = np.abs(r - rad) < w
        val = np.where(ring & (np.sin(seg * (th - TAU * t * sp / seg * 2)) > -.25), .62 + .3 * (rad < 16), val)
    pulse = .5 + .5 * math.sin(TAU * 4 * t)
    val = np.maximum(val, np.exp(-(r / (5 + 2 * pulse)) ** 2) * 1.3)
    shock = (t * 2) % 1 * 34
    val = np.maximum(val, np.exp(-((r - shock) / 1.6) ** 2) * .55)
    img = ramp(val, pal)
    arcs = np.zeros((H, W))
    f = int(t * T)
    r0 = random.Random(f * 7 + 1)
    for i in range(3):
        an = r0.uniform(0, TAU)
        x, y = cx, cy
        for s in range(9):
            nx = cx + (s + 1) * 3 * math.cos(an) + r0.randint(-2, 2)
            ny = cy + (s + 1) * 3 * math.sin(an) + r0.randint(-2, 2)
            line(arcs, x, y, nx, ny, 1)
            x, y = nx, ny
    img[arcs > 0] = hot
    img[(arcs > 0) & (r < 8)] = 255
    return frame_edge(img, pal[1], t, pal[5])


SKULL = ["..XXXXXXXX..", ".XXXXXXXXXX.", "XXXXXXXXXXXX", "XXXXXXXXXXXX", "XX..XXXX..XX", "X....XX....X", "X....XX....X", "XX..XXXX..XX", "XXXXX..XXXXX", ".XXXX..XXXX.", "..XXXXXXXX..", "..X.XX.XX.X.", "..X.XX.XX.X."]


def hellfire(t):
    pal = hexes('0a0204', '3a0508', '8c0d0c', 'dd3a0a', 'ff8a12', 'ffd93a', 'fff8d0')
    n = np.zeros((H, W))
    r = random.Random(3)
    for k in range(7):
        kx, q = r.uniform(.15, .55) * r.choice([-1, 1]), r.choice([1, 2, 3, 4])
        n += np.sin(kx * X + TAU * (q * Y / H + q * 2 * t) + r.uniform(0, TAU) + 1.2 * np.sin(.2 * Y + TAU * t))
    n /= 7
    val = (Y / H) ** 1.2 * 1.25 + n * .62 - .12
    img = ramp(val, pal)
    s = 2
    ox, oy = 8, 12 + int(round(1.5 * math.sin(TAU * 2 * t)))
    eye = .5 + .5 * math.sin(TAU * 3 * t)
    for j, row in enumerate(SKULL):
        for i, c in enumerate(row):
            ys, xs = slice(oy + j * s, oy + (j + 1) * s), slice(ox + i * s, ox + (i + 1) * s)
            if c == 'X':
                img[ys, xs] = hexes('1a0608')[0] if (j + i) % 5 else hexes('2c0a0a')[0]
            elif 3 < j < 8:
                img[ys, xs] = hexes('ff2a08')[0] * (1 - eye) + hexes('ffffa0')[0] * eye
    return img


def glitch(t):
    f = int(t * T)
    r = random.Random(f * 13 + 2)
    m = kmask(4).astype(float)
    base = wave(t, 4, 3) * .08 + .07
    chans = []
    calm = f % 8 < 3
    rows = [0] * H
    y = 0
    while y < H:
        h, s = r.randint(1, 9), 0 if (calm and r.random() < .75) else r.randint(-13, 13)
        for k in range(y, min(H, y + h)):
            rows[k] = s
        y += h
    for c, off in enumerate((-3, 0, 3)):
        ch = np.zeros((H, W))
        o = off * (1 if calm else r.randint(1, 3))
        for y in range(H):
            ch[y] = np.roll(m[y], rows[y] + o)
        chans.append(ch)
    img = np.stack([chans[0], chans[1] * .9, chans[2]], -1) * np.array([255, 240, 255]) + base[..., None] * 90
    for i in range(r.randint(4, 12)):  # broken blocks
        x, y, w, h = r.randrange(W), r.randrange(H), r.randint(2, 16), r.randint(1, 4)
        img[y:y + h, x:x + w] = r.choice([(255, 20, 140), (20, 255, 230), (255, 255, 255), (0, 0, 0), (120, 40, 255)])
    bar = int(t * 2 % 1 * H)
    img[bar:bar + 2] = 255 - img[bar:bar + 2]
    img[::2] *= .78
    if f % 16 == 11:
        img = 255 - img
    return np.clip(img, 0, 255)


def storm(t):
    f = int(t * T)
    pal = hexes('050512', '0d1030', '1c2358', '37408c', '7380d0')
    cloud = (wave(t, 11, 7, 1.1, 1.8) * .5 + .5) * np.clip(1.3 - Y / 30, 0, 1)
    flashes = {1: 1, 2: .5, 3: .9, 4: .35, 9: .8, 10: .4, 16: 1, 17: .6, 18: .9, 19: .3, 25: .85, 26: .4}
    fl = flashes.get(f, 0)
    img = ramp(np.maximum(cloud * 1.05, (Y / H) ** 2 * .75) + fl * .5, pal)
    rr = random.Random(6)
    for i in range(34):
        x0, p, sp = rr.randrange(-10, W), rr.random(), rr.choice([2, 3])
        y = ((p + sp * t) % 1) * (H + 8) - 4
        x = x0 + y * .3
        for k in range(4):
            xi, yi = int(x - k * .3), int(y - k)
            if 0 <= xi < W and 0 <= yi < H:
                img[yi, xi] = hexes('b4c6ff')[0]
    ground = 55 + 2.5 * np.sin(X * .35) + 1.4 * np.sin(X * .9 + 1)
    img[Y > ground] = hexes('030309')[0] + fl * 34
    img[34:57, 17:23] = hexes('030309')[0] + fl * 28  # lone tower
    img[30:34, 15:25] = hexes('030309')[0] + fl * 28
    img[28:30, [15, 17, 19, 21, 23]] = hexes('030309')[0] + fl * 28
    img[37:41, 19:21] = hexes('ffd25a')[0]
    if fl >= .45:
        a = np.zeros((H, W))
        seed = 1 if f < 6 else 3 if f < 12 else 2 if f < 22 else 3
        r = random.Random(seed + (f % 2) * 10)
        bolt(a, r, {1: 10, 2: 30, 3: 8}[seed], 0, 20 if seed < 3 else 34, 30 if seed < 3 else 56, 1)
        img[a > .8] = 255
        img[(a > 0) & (a <= .8)] = hexes('c9a4ff')[0]
    return img


def eye(t):
    cx, cy = 19.5, 31.5
    lookx, looky = 4.5 * math.sin(TAU * 2 * t), 3.2 * math.sin(TAU * 3 * t)
    u, v = X - cx, Y - cy
    pal = hexes('0a0204', '2a0610', '5a0c1c', '8e1626')
    bg = wave(t, 21, 5, 1.3, 1.3) * .5 + .5 - np.hypot(u, v) / 90
    img = ramp(bg * .9, pal)
    tent = np.sin(np.arctan2(v, u) * 7 + .22 * np.hypot(u, v) - TAU * 2 * t)  # veins crawling inward
    img[(tent > .9)] = hexes('d81e3e')[0]
    bl = t * 4 % 1
    f = int(t * T)
    open_ = {12: .5, 13: .08, 14: .08, 15: .55}.get(f, 1)
    half = (15 + .0) * open_
    lid = half * (1 - (u / 19) ** 2)
    inside = (np.abs(v) < lid) & (np.abs(u) < 19)
    sclera = ramp(.86 - np.hypot(u, v) / 60, hexes('5a4a3a', 'b8a890', 'efe6cc', 'fff8e6'), .8)
    img[inside] = sclera[inside]
    iu, iv = u - lookx, v - looky
    ir, ith = np.hypot(iu, iv), np.arctan2(iv, iu)
    striae = np.sin(ith * 13 + TAU * 2 * t) * .5 + .5
    iris = ramp(.25 + .55 * striae * (1 - ir / 16) + .35 * (ir < 8), hexes('3b0a00', 'a81e00', 'ff6a00', 'ffc400', 'fff3a0'))
    m = inside & (ir < 13)
    img[m] = iris[m]
    pupil = (2.4 + 1.8 * (.5 + .5 * math.sin(TAU * 4 * t))) * (1 + .0)
    m = inside & (np.abs(iu) < pupil) & (np.abs(iv) < 9.5)
    img[m] = 0
    m = inside & (np.abs(iu + 4) < 1.6) & (np.abs(iv + 5) < 1.6)
    img[m] = 255
    edge = (np.abs(np.abs(v) - lid) < 1.1) & (np.abs(u) < 19.5)
    img[edge] = hexes('1a0408')[0]
    return frame_edge(img, hexes('3a0810')[0], t, hexes('ff4a20')[0])


def hsv(h, s, v):
    i = (h * 6).astype(int) % 6
    f = h * 6 - np.floor(h * 6)
    p, q, u = v * (1 - s), v * (1 - s * f), v * (1 - s * (1 - f))
    return np.stack([np.choose(i, [v, q, p, p, u, v]), np.choose(i, [u, v, v, q, p, p]), np.choose(i, [p, p, u, v, v, q])], -1) * 255


def kaleido(t):
    u, v = X - 19.5, Y - 31.5
    r, th = np.hypot(u, v), np.arctan2(v, u) + TAU * t / 2
    sect = TAU / 8
    a = np.abs((th % sect) - sect / 2)
    px, py = r * np.cos(a), r * np.sin(a)
    n = np.sin(px * .55 + TAU * 2 * t) + np.sin(py * .9 - TAU * 3 * t) + np.sin((px + py) * .33 + TAU * t) + np.sin(r * .5 - TAU * 2 * t)
    n = n / 4 * .5 + .5
    q = np.round(n * 9) / 9
    hue = (q * 1.4 + t + r / 90) % 1
    val = .18 + .82 * (np.sin(q * 26) * .5 + .5) ** .7
    img = hsv(np.round(hue * 14) / 14 % 1, np.full((H, W), .88), val)
    return frame_edge(img, np.array([10, 8, 20.]), t, np.array([255, 255, 255.]))


GLYPHS = [0b111101111, 0b010111010, 0b101010101, 0b111010111, 0b110011110, 0b100111001, 0b011110011, 0b101111101, 0b001110100, 0b111100111]


def code_rain(t):
    f = int(t * T)
    pal = hexes('060008', '2a0014', '6e0024', 'c8002e', 'ff3b4e', 'ffd0d0')
    m = kmask(4)
    val = np.zeros((H, W))
    r = random.Random(77)
    for c in range(10):
        sp, p, ln = r.choice([1, 2, 3]), r.random(), r.randint(5, 10)
        head = ((p + sp * t) % 1) * 16
        for row in range(16):
            d = (head - row) % 16
            if d >= ln:
                continue
            b = 1 - d / ln
            g = GLYPHS[(c * 7 + row * 3 + (f // 3 if d < 2 else f // 8)) % 10]
            for k in range(9):
                if g >> k & 1:
                    val[row * 4 + k // 3, c * 4 + k % 3] = b
    lit = np.where(m, np.clip(val * 1.4 + .42, 0, 1), val * .62)
    img = ramp(lit, pal, .35)
    return img


def overdrive(t):
    img = ramp(np.clip((Y - 2) / 38, 0, 1) * .95, hexes('0b0224', '230652', '5c0d80', 'c81e8e', 'ff5c7a'), .9)
    r = random.Random(2)
    for i in range(28):
        x, y = r.randrange(1, 39), r.randrange(1, 20)
        if (i + int(t * T) // 2) % 5:
            img[y, x] = (255, 240, 255)
    d = np.hypot(X - 19.5, Y - 25)
    sun = d < 15
    col = ramp(np.clip((Y - 10) / 28, 0, 1), hexes('ffe34a', 'ffa42a', 'ff4c6a', 'ff1ea0'), .8)
    slit = (((Y - t * 8 * 2) % 8) < np.clip((Y - 20) / 4.5, 0, 4)) & (Y > 21)
    m = sun & ~slit
    img[m] = col[m]
    ridge = 40 - np.abs(np.sin(X * .31 + 1)) * 7 - np.abs(np.sin(X * .73)) * 4
    img[(Y > ridge) & (Y < 41)] = hexes('12062e')[0]
    g = Y >= 41
    img[g] = hexes('080218')[0]
    zf = 1 / np.maximum(Y - 39.5, .5)
    lines_h = ((zf * 44 + t * 4) % 1) < np.clip(.05 + zf * 1.6, 0, .6)
    lines_v = (np.abs(((X - 19.5) * zf * 1.1 + 100.5) % 1 - .5) < .05 + zf * .25)
    img[g & (lines_h | lines_v)] = hexes('19f0ff')[0]
    img[41] = hexes('ff4ce0')[0]
    x = 19.5 + 11 * math.sin(TAU * t)  # racing car
    y = 56
    ix = int(x)
    img[y:y + 3, ix - 4:ix + 5] = hexes('14082c')[0]
    img[y - 2:y, ix - 2:ix + 3] = hexes('ff2bd0')[0]
    img[y + 1, [ix - 4, ix + 4]] = hexes('ff2030')[0]
    return img


def poly(img, pts, col):
    p = Image.new('1', (W, H))
    ImageDraw.Draw(p).polygon(pts, fill=1)
    img[np.array(p)] = col


def crest(_):
    img = ramp(.25 + .35 * (Y / H) + .12 * np.sin(X * 1.1 + Y * .4), hexes('0a0a14', '14162a', '20243f', '2c3358'))
    gold = hexes('ffd76a')[0]
    u = np.abs(X - 19.5)
    outer = (Y > 10) & (Y < 54) & (u < np.where(Y < 34, 15, 15 * (1 - ((Y - 34) / 21) ** 2) ** .5))
    inner = (Y > 12) & (Y < 52) & (u < np.where(Y < 34, 13, 13 * np.clip(1 - ((Y - 34) / 19) ** 2, 0, 1) ** .5))
    img[outer] = gold
    q = ((X < 20) ^ (Y < 31))
    img[inner & q] = hexes('8a1020')[0]
    img[inner & ~q] = hexes('13204e')[0]
    img[inner & ((Y.astype(int) + X.astype(int)) % 4 == 0)] *= .75
    poly(img, [(18, 7), (21, 7), (21, 40), (18, 40)], hexes('e8eef8')[0])
    poly(img, [(19, 3), (20, 3), (21, 7), (18, 7)], hexes('ffffff')[0])
    img[7:40, 20] = hexes('8fa0c0')[0]
    poly(img, [(11, 40), (28, 40), (28, 42), (11, 42)], gold)
    img[42:50, 18:22] = hexes('5a3414')[0]
    img[43:50:2, 18:22] = gold
    img[50:53, 17:23] = gold
    for i in range(9):
        for s in (-1, 1):
            a = -.2 + i * .34
            x, y = 19.5 + s * 18 * math.cos(a), 32 + 26 * math.sin(a)
            img[int(y):int(y) + 2, int(x):int(x) + 2] = gold
    return frame_edge(img, gold)


def crane(_):
    img = ramp(.82 - Y / H * .2, hexes('c9bf9e', 'e3dcc0', 'f1ecd8'), .9)
    sun = np.hypot(X - 20, Y - 23) < 16
    img[sun] = hexes('c81d25')[0]
    img[sun & (np.hypot(X - 20, Y - 23) > 14.5)] = hexes('8e0f18')[0]
    for row, y0 in enumerate(range(46, 70, 5)):  # scallop waves
        for cx in range(-4 + (row % 2) * 5, 46, 10):
            d = np.hypot(X - cx, Y - (y0 + 5))
            for rad, c in ((7, '10305e'), (5.5, 'f1ecd8'), (4, '10305e'), (2.5, 'f1ecd8')):
                img[(d < rad) & (Y >= y0 - 2)] = hexes(c)[0]
    white, ink = hexes('fdfdf6')[0], hexes('14141c')[0]
    poly(img, [(4, 12), (16, 22), (24, 25), (18, 31), (9, 26)], white)          # wing
    poly(img, [(36, 8), (27, 20), (24, 25), (18, 31), (26, 30), (33, 18)], white)  # wing
    poly(img, [(4, 12), (9, 15), (11, 21), (9, 26)], ink)
    poly(img, [(36, 8), (34, 14), (33, 18), (30, 16)], ink)
    poly(img, [(17, 25), (24, 25), (26, 35), (20, 38)], white)
    poly(img, [(21, 26), (24, 24), (15, 12), (13, 13)], white)
    poly(img, [(13, 12), (17, 12), (16, 15), (13, 15)], white)
    img[11:13, 13:16] = hexes('c81d25')[0]
    poly(img, [(13, 13), (7, 15), (13, 15)], hexes('c9a23a')[0])
    poly(img, [(20, 36), (24, 36), (29, 45), (24, 42)], ink)
    img[38:47, 21] = ink
    img[38:47, 24] = ink
    return frame_edge(img, hexes('14141c')[0])


def peak(_):
    sky = ramp(np.clip(Y / 46, 0, 1), hexes('101a3c', '2b2a66', '6c3a86', 'd8586a', 'ffb060', 'ffe6a0'), 1)
    img = sky
    moon = np.hypot(X - 29, Y - 11) < 5
    img[moon] = hexes('fff4d6')[0]
    img[np.hypot(X - 31, Y - 10) < 4.2] = sky[6, 30]
    u = np.abs(X - 17)
    main = 16 + u * 1.25 + 1.8 * np.sin(X * 1.1) + np.abs(np.sin(X * .43)) * 3
    img[Y > main] = hexes('33285c')[0]
    snow = (Y > main) & (Y < 26 + 3 * np.sin(X * 1.7))
    img[snow] = hexes('f6efe6')[0]
    img[(Y > main) & (X > 17 + (Y - 16) * .25) & (Y < 54)] = hexes('221c48')[0]
    img[snow & (X > 18 + (Y - 16) * .35)] = hexes('b8a8d8')[0]
    for base, c, a, b in ((41, '3a1f4e', .27, 2), (48, '26153c', .41, 0), (55, '140c26', .63, 5)):
        ridge = base + 3 * np.sin(X * a + b) + 1.5 * np.sin(X * a * 2.3 + b)
        img[Y > ridge] = hexes(c)[0]
    for x in range(1, 39, 3):
        h = 3 + (x * 7) % 4
        y = int(55 + 3 * math.sin(x * .63 + 5))
        for k in range(h):
            img[y - k, x - (h - k) // 3:x + (h - k) // 3 + 1] = hexes('0a0614')[0]
    for x, y in ((8, 12), (11, 10), (5, 15)):
        img[y, x - 1:x + 2] = hexes('141028')[0]
        img[y - 1, [x - 2, x + 2]] = hexes('141028')[0]
    return frame_edge(img, hexes('140c26')[0])


def sigil(_):
    pal = hexes('b5ad92', 'c4bea4', 'cdc8b0', 'dad5bd')
    img = ramp(.6 + .08 * np.sin(X * 1.7) * np.sin(Y * 1.3), pal)
    ink = hexes('2e2c26')[0]
    u, v = X - 19.5, Y - 31.5
    r, th = np.hypot(u, v), np.arctan2(v, u)
    img[(np.abs(r - 18) < .7)] = ink
    tick = (np.abs(r - 16) < 1.2) & (np.sin(th * 24) > .6)
    img[tick] = ink
    img[(np.abs(r - 13.3) < .5) & (np.sin(th * 3) > -.5)] = hexes('b8452f')[0]
    img[kmask(2, 20, 32)] = ink
    for y in (4, 59):
        img[y:y + 2, 4:36:3] = ink
    for y in (8, 55):
        img[y, 4:36] = ink
    for dy in range(0, 3):
        img[14 + dy * 3 - 11, 19:21] = ink
    return frame_edge(img, ink)


def obsidian(_):
    r = random.Random(40)
    pts = [(r.uniform(-4, 44), r.uniform(-4, 68)) for _ in range(26)]
    d = np.sort(np.stack([np.hypot(X - a, Y - b) for a, b in pts]), 0)
    cell = np.argmin(np.stack([np.hypot(X - a, Y - b) for a, b in pts]), 0)
    shade = (cell * 37 % 7) / 7
    img = ramp(.08 + shade * .42 + .1 * (X + Y) / 104, hexes('050508', '0d0d14', '171824', '26283a', '3b3e58'))
    crack = d[1] - d[0] < 1.1
    g = ramp(.5 + .5 * np.sin(X * .3 + Y * .22), hexes('8a5a12', 'd8a02a', 'ffd96a', 'fff4c0'))
    img[crack] = g[crack]
    return frame_edge(img, hexes('d8a02a')[0])


ANIM = {'black_hole': black_hole, 'hyperspace': warp, 'reactor': reactor, 'hellfire': hellfire, 'glitch': glitch, 'thunderhead': storm, 'the_watcher': eye, 'kaleidoscope': kaleido, 'blood_code': code_rain, 'overdrive': overdrive}
STILL = {'crest': crest, 'red_crane': crane, 'first_snow': peak, 'kyro_sigil': sigil, 'kintsugi': obsidian}

for name, fn in {**ANIM, **STILL}.items():
    n = T if name in ANIM else 1
    strip = np.concatenate([np.clip(fn(f / n), 0, 255).astype(np.uint8) for f in range(n)])
    Image.fromarray(strip, 'RGB').save(OUT / f'{name}.png', optimize=True)

# review sheet: four moments of every cape at 3x
names = list(ANIM) + list(STILL)
S = 3
sheet = Image.new('RGB', (5 * (W * 2 * S // 2 + 8) * 2, 3 * (H * S + 10)), (40, 40, 40))
for i, n in enumerate(names):
    im = Image.open(OUT / f'{n}.png')
    fr = [0, 11] if n in ANIM else [0]
    for j, f in enumerate(fr):
        c = im.crop((0, f * H, W, (f + 1) * H)).resize((W * S, H * S), Image.NEAREST)
        sheet.paste(c, ((i % 5) * 2 * (W * S + 8) + j * (W * S + 4), (i // 5) * (H * S + 10)))
sheet.save(OUT.parent / 'sheet.png')
print(sum(p.stat().st_size for p in OUT.glob('*.png')) // 1024, 'KB')
