# Logo plugins

Drop an SVG, PNG, or WebP here. `build.rs` copies it to `public/plugins/logos/` so Wrangler serves it at `/plugins/logos/<filename>`.

Then either:

- Set `site.logoPath` to that path, e.g. `"/plugins/logos/mark.svg"`, and `plugins.logo` to `"image"` or `"auto"`, or
- Leave `site.logoPath` / `logoUrl` empty — `auto` / `image` will use the first file in this folder.

`plugins.logo`:

| Value | Console / email |
|-------|-----------------|
| `auto` | Image if a file or URL is configured; otherwise a monogram from `site.brandName`; otherwise omit |
| `image` | Configured (or first bundled) image; monogram if the file is missing |
| `monogram` | Letter mark only (ignores image files) |
| `none` | No mark |

Do not commit a third-party or stock envelope icon as “your” logo. Replace `public/images/logo.svg` when you fork, or drop your mark in this folder and point `logoPath` at it.
