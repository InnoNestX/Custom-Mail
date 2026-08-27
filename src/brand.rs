use crate::config::MailConfig;

pub fn logo_mark_svg(cfg: &MailConfig, size: u32, grad_key: &str) -> String {
    let grad_id = format!("xxm-g-{grad_key}");
    format!(
        r##"<svg class="logo-mark" width="{size}" height="{size}" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
  <defs>
    <linearGradient id="{grad_id}" x1="6" y1="4" x2="42" y2="44" gradientUnits="userSpaceOnUse">
      <stop stop-color="{tile}"/>
      <stop offset="1" stop-color="{edge}"/>
    </linearGradient>
  </defs>
  <rect width="48" height="48" rx="14" fill="url(#{grad_id})"/>
  {envelope}
</svg>"##,
        size = size,
        grad_id = grad_id,
        tile = cfg.brand.tile,
        edge = cfg.brand.tile_edge,
        envelope = envelope_graphic()
    )
}

pub fn favicon_svg(cfg: &MailConfig, size: u32) -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" width="{size}" height="{size}">
  <defs>
    <linearGradient id="xxm-grad-fav" x1="6" y1="4" x2="42" y2="44" gradientUnits="userSpaceOnUse">
      <stop stop-color="{tile}"/>
      <stop offset="1" stop-color="{edge}"/>
    </linearGradient>
  </defs>
  <rect width="48" height="48" rx="14" fill="url(#xxm-grad-fav)"/>
  {envelope}
</svg>"##,
        size = size,
        tile = cfg.brand.tile,
        edge = cfg.brand.tile_edge,
        envelope = envelope_graphic()
    )
}

fn envelope_graphic() -> &'static str {
    r##"
    <rect x="11" y="19" width="26" height="17" rx="1.5" fill="rgba(255,255,255,.18)"/>
    <path d="M11 19h26a1 1 0 0 1 1 1v15a1 1 0 0 1-1 1H11a1 1 0 0 1-1-1V20a1 1 0 0 1 1-1Z" stroke="#fff" stroke-width="2.5" stroke-linejoin="round"/>
    <path d="M11 19l13 9.5L37 19" stroke="#fff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M23 27.5l4 4 4-4" stroke="#fff" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"/>
  "##
}
