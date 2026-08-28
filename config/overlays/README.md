# Config overlays

Optional JSON files in this directory are **deep-merged** onto `config/mail.json` at compile time (later filenames sort after earlier ones; nested objects merge; arrays and scalars replace; JSON `null` deletes a key so defaults apply).

Example `brand-local.json` (do not commit secrets):

```json
{
  "plugins": { "theme": "nord", "logo": "monogram" },
  "site": { "logoPath": null }
}
```

Runtime (no rebuild): set `MAIL_CONFIG_JSON` to a JSON object in `.dev.vars` or Docker. Slot env vars `MAIL_PROVIDER`, `MAIL_THEME`, `MAIL_LAYOUT`, and `MAIL_LOGO` are applied after that overlay.

Omit any section you do not need in `mail.json` itself — empty strings and `false` flags hide that chrome.
