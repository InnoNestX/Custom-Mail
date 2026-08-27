/** XuXu Mail brand mark — envelope on green tile */

export const BRAND_TILE = "#15624f";
export const BRAND_TILE_EDGE = "#1a7a62";
export const BRAND_ACCENT = "#2f9e7b";
export const BRAND_CREAM = "#f7f4ee";
/** Main-site geometric logo blue */
export const SITE_BRAND_BLUE = "#4E89FE";

const TILE = BRAND_TILE;
const TILE_EDGE = BRAND_TILE_EDGE;

function envelopeGraphic(): string {
  return `
    <rect x="11" y="19" width="26" height="17" rx="1.5" fill="rgba(255,255,255,.18)"/>
    <path d="M11 19h26a1 1 0 0 1 1 1v15a1 1 0 0 1-1 1H11a1 1 0 0 1-1-1V20a1 1 0 0 1 1-1Z" stroke="#fff" stroke-width="2.5" stroke-linejoin="round"/>
    <path d="M11 19l13 9.5L37 19" stroke="#fff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M23 27.5l4 4 4-4" stroke="#fff" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"/>
  `;
}

/** Inline logo for UI. `gradKey` avoids duplicate SVG gradient ids when multiple marks appear on one page. */
export function logoMarkSvg(size = 48, gradKey = "0"): string {
  const gradId = `xxm-g-${gradKey}`;
  return `<svg class="logo-mark" width="${size}" height="${size}" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
  <defs>
    <linearGradient id="${gradId}" x1="6" y1="4" x2="42" y2="44" gradientUnits="userSpaceOnUse">
      <stop stop-color="${TILE}"/>
      <stop offset="1" stop-color="${TILE_EDGE}"/>
    </linearGradient>
  </defs>
  <rect width="48" height="48" rx="14" fill="url(#${gradId})"/>
  ${envelopeGraphic()}
</svg>`;
}

/** Favicon / touch icon */
export function faviconSvg(size = 32): string {
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" width="${size}" height="${size}">
  <defs>
    <linearGradient id="xxm-grad-fav" x1="6" y1="4" x2="42" y2="44" gradientUnits="userSpaceOnUse">
      <stop stop-color="${TILE}"/>
      <stop offset="1" stop-color="${TILE_EDGE}"/>
    </linearGradient>
  </defs>
  <rect width="48" height="48" rx="14" fill="url(#xxm-grad-fav)"/>
  ${envelopeGraphic()}
</svg>`;
}
