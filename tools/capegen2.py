"""Second batch of animated capes. Reuses the helpers of capegen.py without re-baking the first batch."""
import pathlib
HERE = pathlib.Path(__file__).parent
__file__ = str(HERE / 'capegen.py')
exec((HERE / 'capegen.py').read_text(encoding='utf-8').split('ANIM = {')[0])

U, V = X - 19.5, Y - 31.5
R0, TH = np.hypot(U, V), np.arctan2(V, U)


def stars(seed, n=40):
    r = random.Random(seed)
    a = np.zeros((H, W))
    for _ in range(n):
        a[r.randrange(H), r.randrange(W)] = r.uniform(.3, .7)
    return a


def disc(mask_img, cx, cy, rad, col):
    mask_img[np.hypot(X - cx, Y - cy) < rad] = col


def supernova(t):
    pal = hexes('03020a', '140a3a', '4a1078', 'b01e78', 'f0501e', 'ffb62e', 'fff4c0', 'ffffff')
    val = stars(8)
    if t < .4:
        rad = 11 * (1 - (t / .4) ** 5) + 1.2
        flare = 1 + .28 * np.sin(TH * 7 + TAU * 5 * t) * np.sin(TH * 3 - TAU * 3 * t)
        val = np.maximum(val, np.exp(-(R0 / (rad * flare)) ** 2) * (1.35 + .5 * t))
        val = np.maximum(val, np.exp(-((R0 - rad * 1.5) / 2) ** 2) * .4 * (.5 + .5 * np.sin(TH * 11 - TAU * 6 * t)))
        val *= min(1, .2 + t / .07)
    elif t < .47:
        val = np.full((H, W), 1.0)
    else:
        u = (t - .47) / .53
        rad = u ** .7 * 56
        ring = np.exp(-((R0 - rad) / (2 + 5 * u)) ** 2) * (1 - u * .55) * (.72 + .28 * np.sin(TH * 13 + R0 * .4))
        neb = (wave(t, 12, 6, 1.2, 1.2) * .5 + .5) * (R0 < rad) * (1 - u) * .78
        val = np.maximum.reduce([val, ring, neb])
        r = random.Random(31)
        a = np.zeros((H, W))
        for i in range(40):
            an, sp = r.uniform(0, TAU), r.uniform(.5, 1.1)
            d = rad * sp
            line(a, 19.5 + d * math.cos(an), 31.5 + d * math.sin(an), 19.5 + (d + 4) * math.cos(an), 31.5 + (d + 4) * math.sin(an), 1 - u * .6)
        val = np.maximum(val, a)
    return ramp(val, pal)


def sky_dragon(t):
    sky = ramp((wave(t, 41, 6, .9, .9) * .5 + .5) * .75 + .05, hexes('04181e', '0a2e36', '115058', '1f7a78'))
    cl = np.sin(X * .5 + np.sin(Y * .25) * 2 + TAU * t) + np.sin(Y * .4 - TAU * 2 * t)
    sky[cl > 1.6] = hexes('2a9a90')[0]
    img = sky
    gold, red, dark = hexes('ffcf40')[0], hexes('e0281e')[0], hexes('70100e')[0]
    L, step = 46, 1.75
    path = []
    for i in range(L):
        y = ((i * step - 96 * t) % 96) - 16
        x = 19.5 + 12 * math.sin(TAU * 2 * y / 96 + TAU * t * 2)
        path.append((x, y))
    for i in range(L - 1, 0, -1):
        x, y = path[i]
        if abs(path[i - 1][1] - y) > 8:
            continue
        px, py = path[i - 1]
        nx, ny = (y - py) / step, -(x - px) / step
        rad = 4.4 * (1 - i / L) ** .6 + .6
        if i % 3 == 0:
            a = np.zeros((H, W))
            for s_ in (-1, 1):
                line(a, x + s_ * nx * rad, y + s_ * ny * rad, x + s_ * nx * (rad + 3.2) + (x - px), y + s_ * ny * (rad + 3.2) + 3, 1)
            img[a > 0] = hexes('58e0d0')[0]
        disc(img, x, y, rad + .9, hexes('1a0408')[0])
        disc(img, x, y, rad, hexes('b01420')[0] if i % 2 else hexes('d8261e')[0])
        disc(img, x + nx * rad * .4, y + ny * rad * .4, rad * .5, gold)
        disc(img, x - nx * rad * .45, y - ny * rad * .45, rad * .2, dark)
    hx, hy = path[0]
    if -10 < hy < H + 10:
        for k in range(5):  # fire ahead of the snout
            q = (t * 8 + k / 5) % 1
            disc(img, hx + 2.2 * math.sin(k * 2.4 + TAU * t * 4), hy - 10 - q * 11, 2.7 * (1 - q) + .5, hexes('ffe060')[0] if q < .4 else hexes('ff7a14')[0])
        a = np.zeros((H, W))
        for s_ in (-1, 1):
            line(a, hx + s_ * 3, hy + 3, hx + s_ * 8, hy + 11, 1)
            line(a, hx + s_ * 6, hy + 8, hx + s_ * 10, hy + 6, 1)
            line(a, hx + s_ * 3, hy - 6, hx + s_ * 10, hy - 8 + 3 * math.sin(TAU * 4 * t + s_), 1)
            line(a, hx + s_ * 10, hy - 8 + 3 * math.sin(TAU * 4 * t + s_), hx + s_ * 14, hy - 3 + 3 * math.sin(TAU * 4 * t), 1)
        img[a > 0] = hexes('fff0a0')[0]
        hd = [(0, -10), (3, -7), (6, -1), (5, 5), (0, 7), (-5, 5), (-6, -1), (-3, -7)]
        poly(img, [(hx + a_ * 1.15, hy + b_ * 1.12) for a_, b_ in hd], hexes('1a0408')[0])
        poly(img, [(hx + a_, hy + b_) for a_, b_ in hd], hexes('d8261e')[0])
        poly(img, [(hx - 1, hy - 8), (hx + 1, hy - 8), (hx + 2, hy + 5), (hx - 2, hy + 5)], gold)
        for s_ in (-1, 1):
            disc(img, hx + s_ * 3.6, hy - 1, 1.3, hexes('58ffe8')[0])
            disc(img, hx + s_ * 1.3, hy - 8, .6, hexes('1a0408')[0])
    return frame_edge(img, hexes('ffcf40')[0] * .55)


def maelstrom(t):
    pal = hexes('020612', '062044', '0c4a86', '1f8fc8', '7fe0ea', 'ffffff')
    cx, cy = 19.5 + 2 * math.sin(TAU * t), 30 + 2 * math.cos(TAU * t)
    u, v = X - cx, (Y - cy) * 1.15
    r, th = np.hypot(u, v) + .1, np.arctan2(v, u)
    s = np.sin(4 * th + 9 * np.log(r) + TAU * 5 * t)
    f = np.sin(9 * th + 15 * np.log(r) + TAU * 8 * t)
    val = .42 + .3 * s + .16 * f + .25 * np.exp(-((r - 13) / 7) ** 2)
    val = np.where(s + f * .5 > 1.25, 1, val)
    val = val * np.clip(r / 8, 0, 1) ** 2
    img = ramp(val, pal)
    for i in range(3):  # wreckage dragged round
        p = (t * 2 + i / 3) % 1
        rad, ang = 30 * (1 - p) + 5, TAU * (t * 4) + i * 2.1 + p * 7
        x, y = cx + rad * math.cos(ang), cy + rad * math.sin(ang) / 1.15
        xi, yi = int(x), int(y)
        if 3 < xi < W - 4 and 3 < yi < H - 4:
            img[yi, xi - 3:xi + 4] = hexes('3a2010')[0]
            img[yi - 3:yi, xi] = hexes('6a4020')[0]
            img[yi - 3:yi - 1, xi + 1:xi + 3] = hexes('f0e8d0')[0]
    return img


def northern_lights(t):
    img = ramp(np.clip(Y / 42, 0, 1) * .55, hexes('02040e', '061030', '0c1e4a', '123060'), .8)
    s = stars(14, 60)
    img[s > 0] = (s[s > 0, None] * 330).clip(0, 255)
    pal = hexes('08304a', '0a6a58', '12c080', '4cf0a0', 'c8ffd8')
    acc = np.zeros((H, W))
    top = np.zeros((H, W))
    for k, (base, amp, q) in enumerate(((5, 4, 1), (13, 5, -1))):
        c = base + amp * np.sin(X * .17 + TAU * t * q + k * 2) + 2 * np.sin(X * .39 - TAU * t * 2)
        fall = np.where(Y >= c, np.exp(-(Y - c) / 13), np.exp(-((Y - c) / 1.2) ** 2))
        stri = .78 + .22 * np.sin(X * 1.3 + 1.5 * np.sin(X * .23 + TAU * t * 2) + TAU * t * 3 * q)
        acc = np.maximum(acc, fall * stri)
        top = np.maximum(top, np.exp(-((Y - c + 4) / 3) ** 2) * stri)
    glow = ramp(acc * 1.1, pal, .9)
    m = (acc > .07) & (Y < 42)
    img[m] = glow[m]
    pink = ramp(top * .9, hexes('0a1030', '5a1a90', 'c040e0'), .45)
    m = (top > .55) & (Y < 42)
    img[m] = pink[m]
    p = t * 2 % 1  # shooting star
    if p < .22:
        a = np.zeros((H, W))
        x, y = 44 - p * 220, 2 + p * 60
        line(a, x, y, x + 8, y - 2.5, 1)
        img[a > 0] = 255
    ridge = 41 - np.abs(np.sin(X * .23 + 1)) * 8 - np.abs(np.sin(X * .71)) * 3
    land = Y > ridge
    img[land] = hexes('0b1226')[0]
    img[land & (Y < ridge + 2.5)] = hexes('b8d0f0')[0]
    img[Y >= 44] = hexes('0b1226')[0]
    lake = ramp(np.clip(acc[np.clip(87 - Y, 0, H - 1).astype(int), X.astype(int)] * .62, 0, 1), hexes('060c20', '0a2a3a', '0c5a50', '10a070'), .4)
    wv = np.sin(Y * 1.9 + X * .35 - TAU * t * 3) > .7
    lake[wv] = lake[wv] * .6 + 18
    img[Y >= 46] = lake[Y >= 46]
    img[44:46] = hexes('0b1226')[0]
    return img


def soul_reaper(t):
    mist = ramp((wave(t, 17, 6, 1, 1.4) * .5 + .5) * .55 + (Y / H) * .28, hexes('020806', '06201a', '0d3f2e', '17705a', '2aa878'))
    img = mist
    cloak, dark = hexes('0a0c10')[0], hexes('000000')[0]
    sway = 1.5 * math.sin(TAU * t * 2)
    r = random.Random(7)
    souls = []
    for i in range(14):
        ph, rad, h = r.random(), r.uniform(12, 20), r.uniform(10, 54)
        ang = TAU * (ph + 2 * t)
        souls.append((19.5 + rad * math.cos(ang), h + 4 * math.sin(ang * 2) + (-1 if i % 2 else 1) * 3 * math.sin(TAU * t), math.sin(ang)))

    def draw_souls(front):
        for x, y, z in souls:
            if (z > 0) == front:
                c = hexes('b4ffd8')[0] if front else hexes('48d8a0')[0]
                disc(img, x, y, 1.9 if front else 1.3, c)
                a = np.zeros((H, W))
                line(a, x, y, x - 2.5 * (1 if z > 0 else -1), y + 4, 1)
                img[a > 0] = c * .7

    draw_souls(False)
    poly(img, [(19 + sway, 7), (27 + sway, 11), (30 + sway, 23), (33, 44), (37, 64), (3, 64), (7, 44), (10 + sway, 23), (12 + sway, 11)], cloak)
    fold = np.sin(X * 1.5 + Y * .1 + TAU * t * 2) > .6
    img[fold & (Y > 30) & (np.abs(X - 20) < 15 + (Y - 30) * .3)] = hexes('181c24')[0]
    disc(img, 19.5 + sway, 19, 6.8, dark)
    glow = .55 + .45 * math.sin(TAU * t * 4)
    for s in (-3, 3):
        img[18:20, int(19.5 + sway + s - 1):int(19.5 + sway + s + 1)] = hexes('58ffa0')[0] * (.5 + glow * .5) + 60 * glow
    a = np.zeros((H, W))  # scythe
    ang = .22 * math.sin(TAU * t * 2)
    bx, by = 30, 62
    tx, ty = bx + 46 * math.sin(ang - .1), by - 52 * math.cos(ang)
    line(a, bx, by, tx, ty, 1)
    img[a > 0] = hexes('4a3020')[0]
    for k in range(20):
        q = k / 19
        x = tx - 24 * math.sin(q * 2) * math.cos(ang) * .9
        y = ty - 2 + 14 * q ** 2 + 2 * math.sin(q * 3)
        for w in range(int(3 * (1 - q)) + 1):
            xi, yi = int(x), int(y + w)
            if 0 <= xi < W and 0 <= yi < H:
                img[yi, xi] = hexes('e0f8f0')[0] if w == 0 else hexes('8ab8a8')[0]
    draw_souls(True)
    return img


def magma_core(t):
    pal = hexes('0c0606', '2a0a06', '7a1206', 'd8360a', 'ff8a10', 'ffe040', 'fffff0')
    f = np.zeros((H, W))
    r = random.Random(23)
    for i in range(8):
        ax, ay = r.uniform(7, 15), r.uniform(14, 30)
        p, q, a, b = r.choice([1, 2]), r.choice([1, 2, 3]), r.uniform(0, TAU), r.uniform(0, TAU)
        bx, by = 19.5 + ax * math.sin(TAU * p * t + a), 31.5 + ay * math.sin(TAU * q * t + b)
        f += r.uniform(30, 58) / ((X - bx) ** 2 + (Y - by) ** 2 + 4)
    crust = wave(0, 6, 7, 2.2, 2.2)
    val = np.clip((f - .55) / 4.2, 0, 1) ** .75
    val = np.where(f < .55, .08 + .1 * (crust > .2) + .22 * np.exp(-((f - .55) / .12) ** 2), .34 + val * .62 + .07 * np.sin(f * 9 - TAU * 3 * t))
    return ramp(val, pal)


def hypnosis(t):
    u, v = X - 19.5 - 3 * math.sin(TAU * t), Y - 31.5 - 4 * math.cos(TAU * t)
    r, th = np.hypot(u, v) + .1, np.arctan2(v, u)
    s = np.sin(2 * th + r * .8 + 7 * np.sin(r * .06 - TAU * t) - TAU * 5 * t)
    img = np.where((s > 0)[..., None], hexes('f4f0ff')[0], hexes('0a0614')[0])
    band = (r + 80 * t) % 20 < 4
    hue = np.full((H, W), t)
    col = hsv((hue + r / 50) % 1, np.full((H, W), 1.0), np.ones((H, W)))
    m = band & (s > 0)
    img[m] = col[m]
    disc(img, 19.5 + 3 * math.sin(TAU * t), 31.5 + 4 * math.cos(TAU * t), 3 + 1.4 * math.sin(TAU * 4 * t), hexes('ff1a50')[0])
    return img


def gear(img, cx, cy, teeth, rad, rot, col, dark):
    u, v = X - cx, Y - cy
    r, th = np.hypot(u, v), np.arctan2(v, u)
    outer = rad + 1.6 * np.sign(np.sin(teeth * (th - rot)))
    m = r < outer
    img[m] = col
    img[m & (np.sin(teeth * (th - rot)) > 0) & (r > rad - .6)] = col * 1.25
    img[(r < rad * .78) & (r > rad * .68)] = col * .62
    holes = (r < rad * .6) & (r > rad * .26) & (np.sin(5 * (th - rot * 1)) > .15)
    img[holes] = dark
    img[r < rad * .26] = col * 1.2
    img[r < rad * .12] = dark


def clockwork(t):
    img = ramp(.2 + .2 * np.sin(X * .3) * np.sin(Y * .2), hexes('07100e', '0c1a18', '122624'))
    brass, copper, steel = hexes('c89a3a')[0], hexes('b8603a')[0], hexes('8aa0a8')[0]
    dark = img.copy()
    m = 6 * t * TAU
    gear(img, 33, 12, 8, 9, -m / 8, copper, hexes('07100e')[0])
    gear(img, 7, 19, 10, 11, m / 10 + .3, steel, hexes('07100e')[0])
    gear(img, 33, 54, 9, 10, m / 9 + .2, steel, hexes('07100e')[0])
    gear(img, 6, 52, 8, 9, -m / 8, copper, hexes('07100e')[0])
    gear(img, 20, 33, 14, 14.5, -m / 14 + .1, brass, hexes('07100e')[0])
    face = np.hypot(X - 20, Y - 33) < 9.5
    img[face] = hexes('efe6c8')[0]
    for i in range(12):
        a = TAU * i / 12
        img[int(33 + 8 * math.sin(a)), int(20 + 8 * math.cos(a))] = hexes('2a1a10')[0]
    a = np.zeros((H, W))
    line(a, 20, 33, 20 + 7 * math.sin(TAU * t * 8), 33 - 7 * math.cos(TAU * t * 8), 1)
    img[a > 0] = hexes('d8201e')[0]
    a = np.zeros((H, W))
    line(a, 20, 33, 20 + 5 * math.sin(TAU * t), 33 - 5 * math.cos(TAU * t), 1)
    img[a > 0] = hexes('1a100a')[0]
    r = random.Random(int(t * T) // 2 + 3)
    for i in range(5):  # sparks where the teeth meet
        x, y = r.choice([(27, 22), (12, 26), (27, 44), (13, 44)])
        xi, yi = x + r.randint(-3, 3), y + r.randint(-3, 3)
        img[yi, xi] = hexes('ffe080')[0]
    return frame_edge(img, brass * .7, t, hexes('fff0b0')[0])


def smooth(a, b, x):
    q = min(1, max(0, (x - a) / (b - a)))
    return q * q * (3 - 2 * q)


def swarm(t):
    r = random.Random(4)
    m = kmask(4)
    ys, xs = np.nonzero(m)
    pts = [(xs[i], ys[i], r.uniform(0, TAU), r.uniform(8, 34), r.uniform(.8, 1.3)) for i in r.sample(range(len(xs)), min(len(xs), 560))]
    a = np.zeros((H, W, 3))

    def spread(q):
        q %= 1
        return smooth(.24, .42, q) * (1 - smooth(.8, .98, q))

    for x, y, ph, rad, sp in pts:
        for k, br in ((2, .25), (1, .5), (0, 1)):
            q = t - k * .012
            e = spread(q)
            ang = ph + TAU * 2 * q * (1 if sp > 1 else -1) + rad * .1
            sx, sy = 19.5 + rad * .55 * math.cos(ang), 31.5 + rad * math.sin(ang)
            px, py = x * (1 - e) + sx * e + (1 - e) * math.sin(ph + TAU * 6 * q) * .5, y * (1 - e) + sy * e
            xi, yi = int(px), int(py)
            if 0 <= xi < W and 0 <= yi < H:
                c = np.array([40 + 215 * e, 240 - 170 * e, 255.]) if br == 1 else np.array([255 * e, 120 * (1 - e) + 20, 220.])
                a[yi, xi] = np.maximum(a[yi, xi], c * br)
    bg = ramp(.25 + wave(t, 2, 3) * .18, hexes('02020a', '070a22', '0e1238'))
    return np.maximum(a, bg)


def rift(t):
    stone = ramp(.36 + wave(0, 15, 9, 2.4, 2.4) * .55 + .1 * np.sin(X * .6 + Y * .9), hexes('08080e', '14141e', '22222e', '34343f', '4a4a58'))
    jag = np.array([random.Random(int(y // 3) + 5).uniform(-3.4, 3.4) for y in range(H)])
    c = 20 + 8 * np.sin(Y[:, 0] * .13 + 1) + jag
    w = (4.2 + 2.6 * np.sin(TAU * 2 * t + Y[:, 0] * .17)) * np.clip(1.15 - np.abs(Y[:, 0] - 32) / 33, .22, 1)
    d = np.abs(X - c[:, None]) - w[:, None]
    img = stone
    edge = np.exp(-np.clip(d, 0, None) / 3.5) * (.65 + .35 * np.sin(Y * .7 + TAU * 5 * t))
    tint = ramp(edge, hexes('14141e', '3a1078', '8a1ad8', 'e040ff'), .8)
    m = (d >= 0) & (edge > .22)
    img[m] = tint[m]
    void = np.zeros((H, W))
    r = random.Random(1)
    for i in range(30):
        x, p, sp = r.randrange(W), r.random(), r.choice([2, 3, 4])
        y = ((p + sp * t) % 1) * (H + 6)
        line(void, x, y, x, y - 4, 1)
    swirl = np.sin(X * .9 + Y * .23 - TAU * 3 * t) + np.sin(Y * .4 + TAU * 4 * t)
    inner = ramp(.14 + .2 * (swirl > .9) + void * .85, hexes('04010e', '220060', '7a10e0', 'ffffff'), .5)
    img[d < 0] = inner[d < 0]
    img[(d >= 0) & (d < 1.1)] = hexes('ffb8ff')[0]
    r = random.Random(50)
    for i in range(11):  # shards drifting out of the crack
        y0, s, p = r.uniform(4, 60), r.choice([-1, 1]), r.random()
        q = (p + t * r.choice([1, 2])) % 1
        x = 20 + 8 * math.sin(y0 * .13 + 1) + s * (5 + q * 22)
        y = y0 - q * 6
        z = int(3 * (1 - q)) + 1
        xi, yi = int(x), int(y)
        if 1 < xi < W - 3 and 1 < yi < H - 3:
            img[yi:yi + z, xi:xi + z] = hexes('5a5a6a')[0]
            img[yi, xi:xi + z] = hexes('e8b0ff')[0]
    f = int(t * T)
    if f % 8 in (0, 1):
        a = np.zeros((H, W))
        y = (f * 11) % 44 + 10
        bolt(a, random.Random(f), c[y], y, c[y] + (18 if f % 16 else -18), y + 18, 1, False)
        img[a > 0] = hexes('ffffff')[0]
    return img


NEW = {'supernova': supernova, 'sky_dragon': sky_dragon, 'maelstrom': maelstrom, 'northern_lights': northern_lights, 'soul_reaper': soul_reaper, 'magma_core': magma_core, 'hypnosis': hypnosis, 'clockwork': clockwork, 'swarm': swarm, 'rift': rift}
only = sys.argv[1:] or list(NEW)
for name in only:
    strip = np.concatenate([np.clip(NEW[name](f / T), 0, 255).astype(np.uint8) for f in range(T)])
    Image.fromarray(strip, 'RGB').save(OUT / f'{name}.png', optimize=True)

S = 3
sheet = Image.new('RGB', (5 * 2 * (W * S + 6), 2 * (H * S + 10)), (40, 40, 40))
for i, n in enumerate(NEW):
    im = Image.open(OUT / f'{n}.png')
    for j, f in enumerate((6, 21)):
        sheet.paste(im.crop((0, f * H, W, (f + 1) * H)).resize((W * S, H * S), Image.NEAREST), ((i % 5) * 2 * (W * S + 6) + j * (W * S + 3), (i // 5) * (H * S + 10)))
sheet.save(HERE / 'sheet.png')
print('ok')
