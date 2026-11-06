# Cape tools

These make the capes and menu logos in `mixins/kyro/data/kyro_capes.hmod`. Run them in this order:

```bash
uv run --no-project --with pillow --with numpy python capegen.py
uv run --no-project --with pillow --with numpy python capegen2.py
uv run --no-project --with pillow --with pymupdf python packcapes.py
```

The first two write frames to `capes2/`. The last one rebuilds the pack, so build the mod after it.
