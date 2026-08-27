import type { AddressBookEntry } from "./email";
import { logoMarkSvg } from "./brand";

export function renderAppHtml(opts: {
  fromName: string;
  fromEmail: string;
  addressBook: AddressBookEntry[];
}): string {
  const bookJson = JSON.stringify(opts.addressBook).replace(/</g, "\\u003c");
  const fromNameEsc = escapeHtml(opts.fromName);
  const fromEmailEsc = escapeHtml(opts.fromEmail);
  const logoApp = logoMarkSvg(44, "app");
  const logoHeader = logoMarkSvg(44, "hdr");
  const logoHero = logoMarkSvg(56, "hero");
  const mdInlineCode = "`js:行内代码`";
  const mdFencedCode = "```shell\n\n```";
  const mdLink = "[链接文字](https://example.com)";

  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover">
<meta name="theme-color" content="#f7f4ee">
<title>XuXu Mail</title>
<link rel="icon" href="/favicon.svg" type="image/svg+xml">
<link rel="apple-touch-icon" href="/apple-touch-icon.svg">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
<style>
  :root {
    --ink: #1a1c19;
    --ink-soft: #3f463d;
    --muted: #6f776c;
    --line: #dde3d8;
    --panel: rgba(255, 252, 247, .9);
    --panel-solid: #fffbf5;
    --chip: #eef3ea;
    --accent: #1f6f5b;
    --accent-2: #2f9e7b;
    --accent-soft: #e4f4ee;
    --danger: #b42318;
    --ok: #1b7a4a;
    --shadow: 0 24px 60px rgba(26, 28, 25, .10);
    --radius: 18px;
    --font: "Manrope", "PingFang SC", "Hiragino Sans GB", "Noto Sans SC", sans-serif;
    --mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, monospace;
  }
  /* aliases used in rules */
  .locked, button.secondary, .chip, .book button { color: var(--ink-soft); }
  * { box-sizing: border-box; }
  html, body { margin: 0; min-height: 100%; }
  body {
    font-family: var(--font);
    color: var(--ink);
    background:
      radial-gradient(900px 520px at 12% -8%, #d9efe6 0%, transparent 55%),
      radial-gradient(760px 480px at 100% 0%, #f3e7d4 0%, transparent 50%),
      linear-gradient(180deg, #f7f4ee 0%, #eef1ea 100%);
  }
  body::before {
    content: "";
    position: fixed; inset: 0; pointer-events: none; opacity: .28;
    background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='2' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)' opacity='0.5'/%3E%3C/svg%3E");
    mix-blend-mode: soft-light;
  }
  .shell { position: relative; max-width: 920px; margin: 0 auto; padding: 28px 18px 72px; }
  .shell:not(.login-mode) {
    max-height: 100dvh;
    overflow: hidden;
    padding: 10px 12px 10px;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  #appView {
    flex: 1;
    min-height: 0;
    max-height: none;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  #appView .app-header { flex-shrink: 0; }
  .shell.login-mode {
    max-width: none;
    margin: 0;
    padding: 0;
  }
  .mark { display: flex; align-items: center; gap: 12px; margin-bottom: 28px; }
  .mark-badge {
    width: 42px; height: 42px;
    flex-shrink: 0; line-height: 0;
  }
  .mark-badge svg {
    display: block; width: 100%; height: 100%;
    filter: drop-shadow(0 6px 16px rgba(21,98,79,.35));
  }
  .mark h1 { margin: 0; font-size: 22px; letter-spacing: -.04em; font-weight: 800; }
  .mark p { margin: 2px 0 0; color: var(--muted); font-size: 13px; }
  .card {
    background: var(--panel);
    backdrop-filter: blur(14px);
    border: 1px solid rgba(255,255,255,.65);
    border-radius: var(--radius);
    box-shadow: var(--shadow);
    overflow: hidden;
  }
  .login-scene {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: stretch;
    min-height: 100dvh;
    width: 100%;
    padding: max(20px, env(safe-area-inset-top, 0px)) 0 max(24px, env(safe-area-inset-bottom, 0px));
    box-sizing: border-box;
  }
  .login-hero {
    position: relative;
    flex: 0 0 auto;
    padding: 0 22px 16px;
    background:
      radial-gradient(720px 480px at 8% -10%, rgba(47,158,123,.14) 0%, transparent 58%),
      radial-gradient(520px 360px at 92% 100%, rgba(243,214,168,.28) 0%, transparent 62%),
      linear-gradient(180deg, rgba(228,244,238,.42) 0%, rgba(247,244,238,0) 100%);
  }
  .login-hero-inner {
    max-width: 520px;
  }
  .login-brand-row {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 28px;
  }
  .login-brand-row .mark-badge { width: 56px; height: 56px; }
  .login-brand-row h1 {
    margin: 0;
    font-size: 22px;
    letter-spacing: -.03em;
    font-weight: 800;
  }
  .login-brand-row p {
    margin: 3px 0 0;
    color: var(--muted);
    font-size: 13px;
  }
  .login-headline {
    margin: 0 0 12px;
    font-size: clamp(32px, 6vw, 48px);
    line-height: 1.06;
    letter-spacing: -.045em;
    font-weight: 800;
    max-width: 22ch;
  }
  .login-headline em {
    font-style: normal;
    color: var(--accent);
  }
  .login-lead {
    margin: 0;
    color: var(--ink-soft);
    font-size: 15px;
    line-height: 1.6;
    max-width: 36ch;
  }
  .login-points {
    margin: 28px 0 0;
    padding: 0;
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .login-points li {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 14px;
    color: var(--ink-soft);
    line-height: 1.45;
  }
  .login-points li::before {
    content: "";
    flex: 0 0 7px;
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent-2);
    box-shadow: 0 0 0 4px rgba(47,158,123,.14);
  }
  .login-panel {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 0 0 auto;
    padding: 8px 20px 0;
    background: transparent;
    border-top: 0;
  }
  .login-form-card {
    width: min(400px, 100%);
    padding: 28px 26px 26px;
  }
  .login-form-title {
    margin: 0 0 4px;
    font-size: 24px;
    letter-spacing: -.03em;
    font-weight: 800;
  }
  .login-form-sub {
    margin: 0 0 22px;
    font-size: 14px;
    color: var(--muted);
    line-height: 1.5;
  }
  .login-form .actions {
    margin-top: 16px;
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }
  .login-form .actions .status {
    order: 2;
    text-align: center;
    min-height: 0;
  }
  .login-form .actions .primary {
    order: 1;
    width: 100%;
    padding: 13px 16px;
    font-size: 15px;
  }
  .login-alert {
    margin-top: 12px;
    padding: 10px 12px;
    border-radius: 10px;
    font-size: 13px;
    line-height: 1.45;
    border: 1px solid transparent;
  }
  .login-alert.err {
    color: var(--danger);
    background: #fef3f2;
    border-color: #fecdca;
  }
  .login-alert:empty { display: none; }
  label {
    display: block; font-size: 12px; font-weight: 700; color: var(--muted);
    margin-bottom: 8px; letter-spacing: .02em;
  }
  input[type=password], input[type=text], input[type=email], textarea {
    width: 100%;
    border: 1px solid var(--line);
    background: var(--panel-solid);
    border-radius: 12px;
    padding: 12px 14px;
    font: inherit;
    font-size: 16px;
    color: var(--ink);
    outline: none;
    transition: border-color .15s, box-shadow .15s;
  }
  input:focus, textarea:focus {
    border-color: #7bbbb0;
    box-shadow: 0 0 0 4px rgba(31,111,91,.12);
  }
  textarea { line-height: 1.6; }
  .hint { margin-top: 8px; font-size: 12px; color: var(--muted); }
  .md-syntax {
    margin-top: 10px;
    padding: 10px 12px;
    border-radius: 10px;
    background: rgba(238,243,234,.45);
    border: 1px solid rgba(221,227,216,.55);
    font-size: 11px;
    line-height: 1.5;
    color: #9aa89f;
  }
  .md-syntax-head {
    font-size: 10px;
    font-weight: 800;
    letter-spacing: .05em;
    color: #b0bab2;
    margin-bottom: 6px;
  }
  .md-syntax-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px 8px;
    align-items: center;
  }
  .md-syntax-row + .md-syntax-row { margin-top: 6px; }
  .md-syntax code {
    font-family: var(--mono);
    font-size: 10.5px;
    color: #8a9690;
    background: rgba(255,255,255,.55);
    border: 1px solid rgba(221,227,216,.5);
    padding: 2px 6px;
    border-radius: 5px;
    white-space: nowrap;
  }
  .md-syntax code.md-copy {
    cursor: pointer;
    transition: background .15s, border-color .15s, color .15s;
  }
  .md-syntax code.md-copy:hover {
    background: rgba(255,255,255,.92);
    border-color: rgba(31,111,91,.35);
    color: var(--ink-soft);
  }
  .md-syntax code.md-copy.copied {
    border-color: var(--ok);
    color: var(--ok);
    background: rgba(228,244,238,.75);
  }
  .md-syntax-hint {
    font-size: 10px;
    color: #b5bdb6;
  }
  .actions {
    display: flex; align-items: center; justify-content: space-between;
    gap: 12px; margin-top: 18px; flex-wrap: wrap;
  }
  button {
    font: inherit; cursor: pointer; border-radius: 12px;
    border: 1px solid transparent; padding: 11px 16px; font-weight: 700;
  }
  button.primary {
    background: var(--ink); color: #fff;
    box-shadow: 0 10px 22px rgba(26,28,25,.18);
  }
  button.primary:hover { background: #2a2e27; }
  button.primary:disabled { opacity: .55; cursor: not-allowed; }
  button.secondary {
    background: #fff; border-color: var(--line); color: var(--ink-soft);
  }
  button.ghost {
    background: transparent; color: var(--muted); padding-left: 8px; padding-right: 8px;
  }
  .status { font-size: 13px; min-height: 1.2em; }
  .status.ok { color: var(--ok); }
  .status.err { color: var(--danger); }
  .hidden { display: none !important; }
  .app-header {
    display: flex; align-items: center; gap: 10px; flex-wrap: wrap;
    margin-bottom: 8px;
    padding: 8px 12px;
    border-radius: 14px;
    border: 1px solid rgba(255,255,255,.7);
    background: rgba(255,252,247,.78);
    backdrop-filter: blur(12px);
    box-shadow: 0 6px 20px rgba(26,28,25,.04);
  }
  .app-brand { display: flex; align-items: center; gap: 10px; min-width: 0; }
  .app-brand .mark-badge { width: 36px; height: 36px; }
  .app-brand h1 { margin: 0; font-size: 16px; letter-spacing: -.03em; font-weight: 800; line-height: 1.2; }
  .app-brand p { margin: 1px 0 0; color: var(--muted); font-size: 11px; line-height: 1.3; }
  .app-tabs {
    display: inline-flex; gap: 2px; padding: 3px;
    margin-left: auto;
    border-radius: 10px; border: 1px solid var(--line);
    background: rgba(255,255,255,.7);
  }
  .tab {
    background: transparent; border: 0; color: var(--ink-soft);
    padding: 6px 12px; font-size: 12px; border-radius: 8px;
  }
  .tab.active {
    background: var(--accent-soft); color: var(--accent);
    box-shadow: inset 0 0 0 1px #8dcfb8;
  }
  .app-header .ghost {
    border: 1px solid var(--line); background: #fff;
    border-radius: 8px; padding: 6px 10px; font-size: 12px; color: var(--ink-soft);
  }
  .app-header .ghost:hover { border-color: #c5d0c2; color: var(--ink); }
  .workspace { display: flex; flex-direction: column; min-height: 0; }
  #composePanel.workspace,
  #historyPanel.workspace {
    flex: 1;
    min-height: 0;
    max-height: 100%;
  }
  .workspace-head {
    display: flex; align-items: flex-start; justify-content: space-between;
    gap: 16px; padding: 22px 24px 18px;
    border-bottom: 1px solid var(--line);
    background: linear-gradient(180deg, rgba(228,244,238,.45) 0%, rgba(255,252,247,0) 100%);
    flex-shrink: 0;
  }
  .workspace-head h2 {
    margin: 0 0 4px; font-size: 20px; letter-spacing: -.03em; font-weight: 800;
  }
  .workspace-head p { margin: 0; color: var(--muted); font-size: 13px; line-height: 1.5; }
  .workspace-body { padding: 8px 24px 8px; flex: 1; min-height: 0; }
  #composePanel .workspace-body.compose-body {
    padding: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .compose-layout {
    display: grid;
    grid-template-columns: minmax(260px, 36%) minmax(0, 64%);
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }
  .compose-col-side {
    grid-column: 1;
    display: flex;
    flex-direction: column;
    gap: 0;
    min-height: 0;
    overflow-y: auto;
    overscroll-behavior: contain;
    padding: 12px 14px;
    border-right: 1px solid var(--line);
    background: rgba(255,252,247,.5);
  }
  .compose-col-main {
    grid-column: 2;
    display: flex;
    flex-direction: column;
    min-height: 0;
    overflow: hidden;
    padding: 12px 16px 10px;
  }
  .compose-col-side .compose-field,
  .compose-col-main .compose-field {
    padding: 0;
    border: 0;
    background: transparent;
    position: static;
  }
  .compose-field-subject {
    flex-shrink: 0;
    padding-bottom: 0;
  }
  @media (min-width: 861px) {
    .compose-field-body {
      flex: 1;
      min-height: 0;
      padding-top: 8px;
      display: flex;
      flex-direction: column;
    }
    .compose-field-body > .section-label {
      flex-shrink: 0;
    }
    .compose-field-body .compose-field-inner {
      flex: 1 1 0;
      min-height: 0;
      display: flex;
      flex-direction: column;
    }
    .compose-field-body .field-input-wrap {
      flex: 1 1 0;
      min-height: 0;
      display: flex;
      flex-direction: column;
    }
    #body {
      flex: 1 1 0;
      width: 100%;
      min-height: 0;
      height: auto;
      resize: none;
      overflow-y: auto;
      overscroll-behavior: contain;
      box-sizing: border-box;
    }
  }
  .compose-focus-hint {
    flex-shrink: 0;
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 700;
    color: var(--accent);
    background: rgba(228,244,238,.65);
    border-bottom: 1px solid rgba(31,111,91,.15);
  }
  .compose-focus-hint:empty,
  .compose-focus-hint.hidden {
    display: none !important;
  }
  .compose-field.is-focused .section-label {
    color: var(--accent);
  }
  .compose-field.is-focused .section-label .dot {
    background: var(--accent);
    box-shadow: 0 0 0 3px rgba(31,111,91,.18);
  }
  .compose-side-attach .section-label {
    margin-bottom: 6px;
  }
  .compose-side-attach .attach-drop {
    padding: 12px 14px;
    text-align: left;
  }
  .compose-side-attach .attach-drop-title {
    margin: 0 0 4px;
    font-size: 13px;
  }
  .compose-side-attach .attach-drop-sub {
    margin: 0 0 10px;
    font-size: 11px;
  }
  .compose-side-attach #addFilesBtn {
    width: 100%;
  }
  .compose-col-side .from-grid {
    grid-template-columns: 1fr;
  }
  .compose-col-side .md-syntax { margin-top: 0; }
  .compose-field-body .section-label { margin-bottom: 8px; }
  .compose-field {
    min-width: 0;
  }
  .compose-field-inner {
    display: block;
  }
  .compose-section.compact { padding: 8px 0; }
  .compose-section.compact:first-child { padding-top: 0; }
  .field-label {
    display: none;
  }
  @media (max-width: 860px) {
    #composePanel .workspace-body.compose-body {
      overflow: hidden;
    }
    .compose-layout {
      display: flex !important;
      flex-direction: column !important;
      overflow-x: hidden;
      overflow-y: auto;
      flex: 1;
      min-height: 0;
      height: auto !important;
      -webkit-overflow-scrolling: touch;
      overscroll-behavior: contain;
      grid-template-columns: none !important;
    }
    .compose-col-main {
      order: 1;
      display: flex !important;
      flex-direction: column !important;
      width: 100% !important;
      max-width: 100% !important;
      grid-column: unset !important;
      grid-row: unset !important;
      flex: 0 0 auto !important;
      min-height: 0 !important;
      height: auto !important;
      overflow: visible !important;
      padding: 0 !important;
      border: 0 !important;
      background: transparent !important;
    }
    .compose-col-side {
      order: 2;
      display: flex !important;
      flex-direction: column !important;
      width: 100% !important;
      max-width: 100% !important;
      grid-column: unset !important;
      grid-row: unset !important;
      flex: 0 0 auto !important;
      min-height: 0 !important;
      height: auto !important;
      overflow: visible !important;
      padding: 0 !important;
      border: 0 !important;
      background: transparent !important;
    }
    .compose-col-side .compose-field-to { order: 1; }
    .compose-col-side .compose-field-from { order: 2; }
    .compose-col-side .compose-field-syntax { order: 3; }
    .compose-col-side .compose-field-attach { order: 4; }
    .compose-field {
      flex: 0 0 auto !important;
      width: 100% !important;
      box-sizing: border-box;
      padding: 10px 14px !important;
      border-right: 0 !important;
      border-bottom: 1px solid var(--line) !important;
      background: transparent;
      height: auto !important;
      min-height: 0 !important;
      max-height: none !important;
      overflow: visible !important;
      position: relative !important;
      display: block !important;
      clear: both !important;
    }
    .compose-field.is-focused {
      background: rgba(228,244,238,.45);
    }
    .compose-field .section-label {
      display: none !important;
    }
    .compose-field-inner {
      display: flex !important;
      align-items: flex-start;
      gap: 8px;
      padding: 2px 0;
      min-height: 44px;
      height: auto !important;
      flex: none !important;
      width: 100% !important;
    }
    .field-label {
      display: block;
      flex: 0 0 auto;
      width: auto;
      min-width: 52px;
      max-width: none;
      white-space: nowrap;
      font-size: 14px;
      font-weight: 700;
      color: var(--muted);
      padding: 12px 0 0 2px;
      line-height: 1.3;
    }
    .compose-field.is-focused .field-label {
      color: var(--accent);
    }
    .field-input-wrap {
      flex: 1 1 auto;
      min-width: 0;
      width: auto;
      padding: 2px 0;
      display: block !important;
      height: auto !important;
      min-height: 0 !important;
      max-height: none !important;
    }
    .compose-field input[type=text],
    .compose-field input[type=email],
    .compose-field textarea {
      border: 0;
      box-shadow: none;
      padding: 10px 8px;
      border-radius: 10px;
      background: transparent;
      font-size: 16px;
      width: 100%;
      box-sizing: border-box;
    }
    .compose-field.is-focused input[type=text],
    .compose-field.is-focused input[type=email],
    .compose-field.is-focused textarea {
      background: rgba(255,255,255,.92);
      border: 1px solid rgba(31,111,91,.35);
    }
    .compose-field-to .to-box {
      border: 0;
      padding: 4px 0;
      background: transparent;
      box-shadow: none;
    }
    .compose-field-to.is-focused .to-box {
      background: rgba(255,255,255,.92);
      border: 1px solid rgba(31,111,91,.35);
      border-radius: 10px;
      padding: 6px 8px;
    }
    .compose-field-to .hint,
    .compose-field-to .book {
      margin-top: 4px;
    }
    .compose-field-from .from-grid {
      gap: 6px;
    }
    .compose-field-from .locked {
      padding: 8px 10px;
    }
    .compose-field-subject {
      flex: none !important;
      display: block !important;
      height: auto !important;
      min-height: 0 !important;
      padding: 10px 14px !important;
    }
    .compose-field-body {
      flex: none !important;
      display: block !important;
      height: auto !important;
      min-height: 0 !important;
      max-height: none !important;
      padding: 10px 14px !important;
    }
    .compose-field-body .compose-field-inner {
      flex-direction: column;
      align-items: stretch;
      flex: none !important;
      min-height: 0 !important;
      height: auto !important;
    }
    .compose-field-body .field-label {
      padding: 4px 2px 0;
    }
    .compose-field-body .field-input-wrap {
      flex: none !important;
      display: block !important;
      height: auto !important;
      min-height: 0 !important;
      max-height: none !important;
    }
    .compose-field-body #body,
    #body {
      flex: none !important;
      display: block !important;
      width: 100% !important;
      min-height: 160px !important;
      height: 160px !important;
      max-height: none !important;
      resize: none;
    }
    .compose-field-syntax .compose-field-inner,
    .compose-field-attach .compose-field-inner {
      flex-direction: column;
      align-items: stretch;
      width: 100% !important;
    }
    .compose-field-syntax .field-label,
    .compose-field-attach .field-label {
      padding-top: 4px;
    }
    .compose-field-attach {
      border-bottom: 0 !important;
      padding-bottom: 12px !important;
    }
    .compose-side-attach .attach-drop {
      min-height: 88px;
      width: 100%;
      box-sizing: border-box;
    }
    .compose-side-attach .attach-drop-title {
      font-size: 13px;
    }
    .compose-side-attach .attach-drop-sub {
      font-size: 11px;
      margin-bottom: 10px;
    }
  }
  #composePanel .workspace-foot {
    padding: 8px 12px 10px;
    gap: 8px;
  }
  #composePanel .workspace-foot button {
    padding: 8px 14px;
    font-size: 13px;
    border-radius: 10px;
  }
  #composePanel .workspace-foot .status {
    font-size: 12px;
    min-height: 1em;
  }
  .workspace-foot {
    display: flex; align-items: center; justify-content: space-between;
    gap: 12px; flex-wrap: wrap;
    flex-shrink: 0;
    padding: 12px 18px 14px;
    border-top: 1px solid var(--line);
    background: rgba(255,252,247,.92);
  }
  .workspace-foot .btn-group { display: flex; gap: 8px; flex-wrap: wrap; }
  .compose-section {
    padding: 16px 0;
    border-bottom: 1px solid rgba(221,227,216,.75);
  }
  .compose-section:last-child { border-bottom: 0; }
  .compose-section > label, .compose-section .section-label {
    display: flex; align-items: center; gap: 8px;
    font-size: 12px; font-weight: 800; color: var(--muted);
    margin-bottom: 10px; letter-spacing: .04em; text-transform: uppercase;
  }
  .section-label .dot {
    width: 6px; height: 6px; border-radius: 50%; background: var(--accent-2);
  }
  .row { margin-bottom: 16px; }
  .from-grid {
    display: grid;
    grid-template-columns: 1fr minmax(0, 38%);
    gap: 10px;
    align-items: start;
  }
  @media (max-width: 700px) {
    .from-grid { grid-template-columns: 1fr; }
    .shell:not(.login-mode) { padding: 8px 10px 8px; }
    .login-scene {
      min-height: 100dvh;
      justify-content: center;
      padding: max(16px, env(safe-area-inset-top, 0px)) 0 max(20px, env(safe-area-inset-bottom, 0px));
    }
    .login-hero { padding: 0 18px 12px; }
    .login-brand-row { margin-bottom: 16px; }
    .login-headline { max-width: none; font-size: clamp(28px, 8vw, 36px); margin-bottom: 8px; }
    .login-lead { font-size: 14px; }
    .login-points { margin-top: 16px; gap: 8px; }
    .login-panel {
      padding: 0 16px;
      align-items: stretch;
    }
    .login-form-card { padding: 22px 20px 20px; }
    .app-header {
      position: static;
      flex-direction: row;
      align-items: center;
      flex-wrap: wrap;
      gap: 8px;
      padding: 8px 10px;
      margin-bottom: 6px;
    }
    .app-brand { width: auto; flex: 1; min-width: 0; }
    .app-brand .mark-badge { width: 32px; height: 32px; }
    .app-brand h1 { font-size: 15px; }
    .app-brand p { display: none; }
    .app-tabs { margin-left: 0; width: auto; flex: 1; min-width: 140px; }
    .app-tabs .tab { flex: 1; text-align: center; padding: 8px 10px; font-size: 12px; }
    .app-header .ghost {
      width: auto;
      flex: 0 0 auto;
      text-align: center;
      padding: 8px 12px;
      font-size: 12px;
    }
    .workspace { min-height: 0; }
    .workspace-head, .workspace-body, .workspace-foot { padding-left: 16px; padding-right: 16px; }
    .workspace-head { flex-direction: column; gap: 10px; }
    .workspace-head .secondary { width: 100%; }
    .workspace-foot {
      flex-direction: column;
      align-items: stretch;
      gap: 10px;
    }
    .workspace-foot .btn-group {
      width: 100%;
      flex-direction: column;
    }
    #composePanel .workspace-foot .btn-group {
      flex-direction: row;
      width: auto;
    }
    #composePanel .workspace-foot .btn-group button {
      width: auto;
      padding: 8px 14px;
    }
    .hist-layout {
      grid-template-rows: minmax(160px, 32vh) minmax(0, 1fr);
      height: min(520px, calc(100dvh - 200px));
    }
    .hist-list { padding: 12px 14px; }
    .hist-detail-pane { padding: 14px 16px 18px; }
    .hist-detail-pane .kv div { grid-template-columns: 52px 1fr; }
    textarea { min-height: 120px; }
    .app-brand p { display: none; }
    #composePanel .workspace-foot {
      padding: 8px 12px max(10px, env(safe-area-inset-bottom, 0px));
    }
    .preview-overlay { padding: 10px; }
    .preview-foot { flex-direction: column; }
    .preview-foot button { width: 100%; }
    button { min-height: 44px; }
  }
  @media (min-width: 900px) {
    .login-scene {
      display: grid;
      grid-template-columns: 1.08fr 0.92fr;
      align-content: stretch;
      justify-content: stretch;
      min-height: 100dvh;
      padding: 0;
    }
    .login-hero {
      display: flex;
      align-items: center;
      padding: 48px 56px;
      min-height: 100dvh;
    }
    .login-hero-inner { max-width: 480px; }
    .login-panel {
      min-height: 100dvh;
      padding: 48px 40px;
      border-top: 0;
      border-left: 1px solid rgba(221,227,216,.75);
      background: rgba(255,252,247,.88);
    }
  }
  .locked {
    display: flex;
    align-items: center;
    gap: 6px;
    min-height: 0;
    padding: 10px 10px;
    border-radius: 10px;
    background: #f3f6f1;
    border: 1px dashed #c5d8cc;
    font-family: var(--mono);
    font-size: 11px;
    color: var(--muted);
    max-width: 100%;
  }
  .locked > span:not(.pill) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }
  .pill {
    flex: 0 0 auto;
    font-family: var(--font);
    font-size: 9px;
    font-weight: 800;
    letter-spacing: .05em;
    color: var(--accent);
    background: #fff;
    border: 1px solid #b8dccb;
    border-radius: 999px;
    padding: 2px 6px;
  }
  .to-box {
    border: 1px solid var(--line); border-radius: 12px; padding: 10px;
    background: var(--panel-solid);
    transition: border-color .15s, box-shadow .15s;
  }
  .to-box:focus-within {
    border-color: #7bbbb0;
    box-shadow: 0 0 0 4px rgba(31,111,91,.12);
  }
  .chips { display: flex; flex-wrap: wrap; gap: 6px; min-height: 0; margin-bottom: 6px; }
  .chips:empty { display: none; }
  .chip {
    display: inline-flex; align-items: center; gap: 6px;
    background: var(--chip); border: 1px solid var(--line);
    border-radius: 999px; padding: 5px 8px 5px 10px; font-size: 13px;
  }
  .chip button {
    border: 0; background: transparent; color: var(--muted);
    padding: 0 2px; font-size: 15px; line-height: 1;
  }
  .chip button:hover { color: var(--danger); }
  .to-input {
    border: 0 !important; box-shadow: none !important;
    padding: 8px 6px !important; background: transparent !important;
  }
  .book { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 10px; }
  .book:empty { display: none; }
  .book button {
    border: 1px solid var(--line); background: #fff; color: var(--ink-soft);
    border-radius: 999px; padding: 7px 12px; font-size: 12px; font-weight: 600;
  }
  .book button:hover {
    border-color: var(--accent-2); color: var(--accent); background: var(--accent-soft);
  }
  .book button.active {
    background: var(--accent-soft); border-color: #8dcfb8; color: var(--accent);
  }
  .attach-drop {
    border: 1.5px dashed #b8cfc0; border-radius: 14px;
    background: linear-gradient(180deg, rgba(228,244,238,.35), rgba(255,252,247,.5));
    padding: 18px 16px; text-align: center;
    transition: border-color .15s, background .15s, transform .15s;
    cursor: pointer;
  }
  .attach-drop.dragover {
    border-color: var(--accent);
    background: var(--accent-soft);
    transform: translateY(-1px);
  }
  .attach-drop-title {
    font-size: 14px; font-weight: 700; color: var(--ink-soft); margin: 0 0 4px;
  }
  .attach-drop-sub { margin: 0 0 12px; font-size: 12px; color: var(--muted); }
  .attach-list { display: flex; flex-wrap: wrap; gap: 10px; margin-top: 12px; }
  .attach-list:empty { display: none; margin: 0; }
  .attach-item {
    display: flex; align-items: center; gap: 10px;
    padding: 8px 10px; border: 1px solid var(--line); border-radius: 12px;
    background: #fff; max-width: 100%;
    box-shadow: 0 4px 12px rgba(26,28,25,.04);
  }
  .attach-thumb {
    width: 44px; height: 44px; object-fit: cover; border-radius: 8px; flex-shrink: 0;
  }
  .attach-icon {
    width: 44px; height: 44px; display: grid; place-items: center;
    background: var(--chip); border-radius: 8px; color: var(--accent);
  }
  .attach-meta { flex: 1; min-width: 0; }
  .attach-name {
    font-size: 13px; font-weight: 600; white-space: nowrap;
    overflow: hidden; text-overflow: ellipsis;
  }
  .attach-size { font-size: 11px; color: var(--muted); margin-top: 2px; }
  .attach-item button {
    border: 0; background: transparent; color: var(--muted); padding: 4px;
    font-size: 16px; line-height: 1;
  }
  .attach-item button:hover { color: var(--danger); }
  .hist-layout {
    display: grid;
    gap: 0;
    flex: 1;
    min-height: 0;
    height: min(560px, calc(100dvh - 220px));
    max-height: 100%;
    overflow: hidden;
    grid-template-rows: minmax(0, 1fr);
  }
  @media (min-width: 860px) {
    .hist-layout {
      grid-template-columns: minmax(260px, 38%) minmax(0, 62%);
    }
    .hist-layout .hist-list {
      border-right: 1px solid var(--line);
    }
    .hist-detail-pane {
      border-top: 0 !important;
    }
  }
  @media (max-width: 859px) {
    .hist-layout {
      grid-template-rows: minmax(160px, 34vh) minmax(0, 1fr);
    }
    .hist-detail-pane {
      border-top: 1px solid var(--line);
    }
  }
  .hist-list {
    display: flex; flex-direction: column; gap: 8px;
    padding: 16px 18px 20px;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
    overscroll-behavior: contain;
    -webkit-overflow-scrolling: touch;
    scrollbar-gutter: stable;
  }
  .hist-row {
    border: 1px solid var(--line); border-radius: 12px; padding: 12px 14px;
    background: #fff; cursor: pointer; transition: border-color .15s, background .15s, transform .12s;
  }
  .hist-row:hover { border-color: #8dcfb8; transform: translateY(-1px); }
  .hist-row.active {
    border-color: #8dcfb8; background: var(--accent-soft);
    box-shadow: 0 0 0 3px rgba(31,111,91,.08);
  }
  .hist-meta { font-size: 11px; color: var(--muted); margin-bottom: 4px; }
  .hist-meta .ok { color: var(--ok); font-weight: 700; }
  .hist-meta .fail { color: var(--danger); font-weight: 700; }
  .hist-subject { font-size: 14px; font-weight: 700; margin-bottom: 4px; }
  .hist-to { font-size: 12px; color: var(--ink-soft); }
  .hist-detail-pane {
    padding: 18px 20px 22px;
    border-top: 1px solid var(--line);
    background: rgba(255,252,247,.55);
    font-size: 13px; line-height: 1.55;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
    overscroll-behavior: contain;
    -webkit-overflow-scrolling: touch;
    scrollbar-gutter: stable;
  }
  .hist-detail-pane.hidden { display: none !important; }
  .hist-detail-pane h4 {
    margin: 0 0 12px; font-size: 12px; font-weight: 800;
    letter-spacing: .06em; text-transform: uppercase; color: var(--muted);
  }
  .hist-detail-pane .kv {
    display: grid; gap: 8px; margin-bottom: 16px;
  }
  .hist-detail-pane .kv div {
    display: grid; grid-template-columns: 64px 1fr; gap: 8px;
  }
  .hist-detail-pane .kv strong {
    color: var(--muted); font-weight: 700; font-size: 12px;
  }
  .hist-detail-pane pre {
    margin: 0; white-space: pre-wrap; word-break: break-word;
    font-family: var(--mono); font-size: 12px; line-height: 1.55;
    padding: 14px; border-radius: 12px;
    background: #fff; border: 1px solid var(--line);
    max-height: none;
  }
  .hist-empty, .hist-detail-empty {
    color: var(--muted); font-size: 13px; padding: 48px 16px;
    text-align: center; line-height: 1.6;
  }
  .hist-empty strong, .hist-detail-empty strong {
    display: block; color: var(--ink-soft); font-size: 14px; margin-bottom: 4px;
  }
  .preview-overlay {
    position: fixed; inset: 0; z-index: 200;
    background: rgba(26, 28, 25, .42);
    backdrop-filter: blur(5px);
    display: grid; place-items: center; padding: 16px;
  }
  .preview-panel {
    width: min(760px, 100%); max-height: 92vh;
    background: var(--panel-solid);
    border: 1px solid var(--line);
    border-radius: 16px;
    box-shadow: var(--shadow);
    display: flex; flex-direction: column;
    overflow: hidden;
  }
  .preview-head {
    padding: 16px 18px 12px;
    border-bottom: 1px solid var(--line);
  }
  .preview-head h3 {
    margin: 0 0 10px; font-size: 16px; font-weight: 800; letter-spacing: -.02em;
  }
  .preview-meta {
    display: grid; gap: 6px; font-size: 13px; color: var(--ink-soft);
  }
  .preview-meta strong { color: var(--muted); font-weight: 700; margin-right: 6px; }
  .preview-attach-chips { display: flex; flex-wrap: wrap; gap: 6px; margin-top: 8px; }
  .preview-attach-chip {
    font-size: 11px; padding: 4px 8px; border-radius: 999px;
    background: var(--chip); border: 1px solid var(--line);
  }
  .preview-frame-wrap {
    flex: 1; min-height: 0; overflow: hidden; background: #f4f1ec;
    border-top: 1px solid var(--line); border-bottom: 1px solid var(--line);
  }
  .preview-frame {
    display: block; width: 100%; height: 100%; min-height: 360px; border: 0; background: #fff;
  }
  .preview-foot {
    padding: 12px 18px; display: flex; justify-content: flex-end; gap: 10px;
  }
  .result-panel {
    width: min(380px, 100%);
    background: var(--panel-solid);
    border: 1px solid var(--line);
    border-radius: 16px;
    box-shadow: var(--shadow);
    padding: 26px 22px 20px;
    text-align: center;
  }
  .result-panel .result-mark {
    width: 48px; height: 48px; margin: 0 auto 14px;
    border-radius: 50%; display: grid; place-items: center;
    font-size: 22px; font-weight: 800;
  }
  .result-panel.ok .result-mark {
    background: rgba(228,244,238,.95); color: var(--ok); border: 1px solid #b8e0cc;
  }
  .result-panel.err .result-mark {
    background: rgba(254,243,242,.95); color: var(--danger); border: 1px solid #f0c9c6;
  }
  .result-panel h3 {
    margin: 0 0 8px; font-size: 18px; font-weight: 800; letter-spacing: -.02em;
  }
  .result-panel p {
    margin: 0 0 18px; font-size: 13px; line-height: 1.55; color: var(--ink-soft);
    word-break: break-word;
  }
  .result-panel button { min-width: 120px; }

  .attach-icon {
    font-size: 10px; font-weight: 800; letter-spacing: .04em;
    color: var(--accent);
  }
  .workspace-foot .status { flex: 1; min-width: 140px; }
  #historyPanel .workspace-head { flex-shrink: 0; }
  .hist-layout { align-items: stretch; }
  .app-header { position: sticky; top: 6px; z-index: 20; }
</style>
</head>
<body>
  <div class="shell login-mode" id="shell">
    <div class="mark hidden" id="topMark">
      <div class="mark-badge">${logoHeader}</div>
      <div>
        <h1>XuXu Mail</h1>
        <p id="markSubApp">私有网页发信 · 仅本域可用</p>
      </div>
    </div>

    <div id="loginView" class="login-scene">
      <section class="login-hero">
        <div class="login-hero-inner">
          <div class="login-brand-row">
            <div class="mark-badge">${logoHero}</div>
            <div>
              <h1>XuXu Mail</h1>
              <p>Private web mail</p>
            </div>
          </div>
          <h2 class="login-headline">A quiet place to <em>send mail</em></h2>
          <p class="login-lead">Compose, preview, and send — a simple console for outbound notices.</p>
          <ul class="login-points">
            <li>Preview before you send</li>
            <li>Attachments and delivery log</li>
            <li>Secure session access</li>
          </ul>
        </div>
      </section>
      <section class="login-panel">
        <div class="card login-form-card">
          <h3 class="login-form-title">Sign in</h3>
          <p class="login-form-sub">Enter your password to continue.</p>
          <form id="loginForm" class="login-form">
            <label for="password">Password</label>
            <input id="password" type="password" placeholder="Enter your password" autocomplete="current-password">
            <div class="login-alert err" id="loginAlert" role="alert"></div>
            <div class="actions">
              <div class="status" id="loginStatus"></div>
              <button class="primary" id="loginBtn" type="submit">Sign in</button>
            </div>
          </form>
        </div>
      </section>
    </div>

    <div id="appView" class="hidden">
      <header class="app-header">
        <div class="app-brand">
          <div class="mark-badge">${logoApp}</div>
          <div>
            <h1>XuXu Mail</h1>
            <p>私有网页发信 · 仅本域可用</p>
          </div>
        </div>
        <div class="app-tabs">
          <button class="tab active" id="tabCompose" type="button">写信</button>
          <button class="tab" id="tabHistory" type="button">发送记录</button>
        </div>
        <button class="ghost" id="logoutBtn" type="button">退出</button>
      </header>

      <div id="composePanel" class="card workspace">
        <div class="workspace-body compose-body">
          <div id="composeFocusHint" class="compose-focus-hint hidden" aria-live="polite"></div>
          <div class="compose-layout">
            <div class="compose-col-side">
              <div class="compose-field compose-field-from compose-section compact" data-field="from" data-label="发件人">
                <div class="section-label"><span class="dot"></span>发件人</div>
                <div class="compose-field-inner">
                  <span class="field-label">发件人</span>
                  <div class="field-input-wrap">
                    <div class="from-grid">
                      <div>
                        <input id="fromName" type="text" value="${fromNameEsc}" placeholder="显示名称" maxlength="80">
                      </div>
                      <div class="locked"><span class="pill">LOCKED</span><span>${fromEmailEsc}</span></div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="compose-field compose-field-to compose-section compact" data-field="to" data-label="收件人">
                <div class="section-label"><span class="dot"></span>收件人</div>
                <div class="compose-field-inner">
                  <span class="field-label">收件人</span>
                  <div class="field-input-wrap">
                    <div class="to-box">
                      <div class="chips" id="chips"></div>
                      <input class="to-input" id="toInput" type="email" placeholder="输入邮箱后回车，或从下方点选" autocomplete="off">
                    </div>
                    <div class="book" id="addressBook"></div>
                    <div class="hint">可点选地址簿，也可手动填写任意邮箱</div>
                  </div>
                </div>
              </div>

              <div class="compose-field compose-field-syntax compose-section compact" data-field="syntax" data-label="语法参考">
                <div class="section-label"><span class="dot"></span>语法参考</div>
                <div class="compose-field-inner">
                  <span class="field-label">语法参考</span>
                  <div class="field-input-wrap">
                    <div class="md-syntax" id="mdSyntaxRef">
                      <div class="md-syntax-head">点击复制</div>
                      <div class="md-syntax-row">
                        <code class="md-copy" data-copy="# 一级标题" title="点击复制"># 一级</code>
                        <code class="md-copy" data-copy="## 二级标题" title="点击复制">## 二级</code>
                        <code class="md-copy" data-copy="### 三级标题" title="点击复制">### 三级</code>
                      </div>
                      <div class="md-syntax-row">
                        <code class="md-copy" data-copy="**粗体**" title="点击复制">**粗体**</code>
                        <code class="md-copy" data-copy="${escapeHtml(mdInlineCode)}" title="点击复制">&#96;js:行内&#96;</code>
                        <code class="md-copy" data-copy="${escapeHtml(mdFencedCode)}" title="点击复制">&#96;&#96;&#96;shell 块&#96;&#96;&#96;</code>
                      </div>
                      <div class="md-syntax-row">
                        <code class="md-copy" data-copy="- 列表项" title="点击复制">- 列表</code>
                        <code class="md-copy" data-copy="${escapeHtml(mdLink)}" title="点击复制">[链接](url)</code>
                        <span class="md-syntax-hint">空行分段</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="compose-field compose-field-attach compose-section compact compose-side-attach" data-field="attach" data-label="附件">
                <div class="section-label"><span class="dot"></span>附件</div>
                <div class="compose-field-inner">
                  <span class="field-label">附件</span>
                  <div class="field-input-wrap">
                    <div class="attach-drop" id="attachDrop">
                      <p class="attach-drop-title">拖拽或点击添加</p>
                      <p class="attach-drop-sub">最多 8 个 · 单文件 ≤ 8MB · 合计 ≤ 15MB</p>
                      <input id="fileInput" type="file" multiple hidden>
                      <button class="secondary" id="addFilesBtn" type="button">添加文件</button>
                      <div class="attach-list" id="attachList"></div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="compose-col-main">
              <div class="compose-field compose-field-subject compose-section compose-main-subject" data-field="subject" data-label="主题">
                <div class="section-label"><span class="dot"></span>主题</div>
                <div class="compose-field-inner">
                  <span class="field-label">主题</span>
                  <div class="field-input-wrap">
                    <input id="subject" type="text" placeholder="邮件主题">
                  </div>
                </div>
              </div>

              <div class="compose-field compose-field-body compose-section compose-body-wrap" data-field="body" data-label="正文">
                <div class="section-label"><span class="dot"></span>正文</div>
                <div class="compose-field-inner">
                  <span class="field-label">正文</span>
                  <div class="field-input-wrap">
                    <textarea id="body" placeholder="在此输入正文…" aria-describedby="mdSyntaxRef"></textarea>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="workspace-foot">
          <div class="status" id="sendStatus"></div>
          <div class="btn-group">
            <button class="secondary" id="clearBtn" type="button">清空</button>
            <button class="secondary" id="previewBtn" type="button">预览</button>
            <button class="primary" id="sendBtn" type="button">发送</button>
          </div>
        </div>
      </div>

      <div id="historyPanel" class="card workspace hidden">
        <div class="workspace-head">
          <div>
            <h2>发送记录</h2>
            <p>最近 10 条 · 点击条目查看详情</p>
          </div>
          <button class="secondary" id="refreshHistoryBtn" type="button">刷新</button>
        </div>
        <div class="hist-layout">
          <div id="historyList" class="hist-list"></div>
          <div id="historyDetail" class="hist-detail-pane">
            <div class="hist-detail-empty"><strong>选择一条记录</strong>在左侧列表中点击，查看收件人、正文与附件</div>
          </div>
        </div>
      </div>
    </div>

    <div id="resultOverlay" class="preview-overlay hidden" role="dialog" aria-modal="true" aria-labelledby="resultTitle">
      <div class="result-panel" id="resultPanel">
        <div class="result-mark" id="resultMark">✓</div>
        <h3 id="resultTitle">发送成功</h3>
        <p id="resultMsg"></p>
        <button class="primary" id="resultCloseBtn" type="button">确定</button>
      </div>
    </div>

    <div id="previewOverlay" class="preview-overlay hidden" role="dialog" aria-modal="true" aria-labelledby="previewTitle">
      <div class="preview-panel">
        <div class="preview-head">
          <h3 id="previewTitle">邮件预览</h3>
          <div class="preview-meta" id="previewMeta"></div>
        </div>
        <div class="preview-frame-wrap">
          <iframe id="previewFrame" class="preview-frame" title="邮件预览"></iframe>
        </div>
        <div class="preview-foot">
          <button class="secondary" id="previewCloseBtn" type="button">返回修改</button>
          <button class="primary" id="previewSendBtn" type="button">确认发送</button>
        </div>
      </div>
    </div>
  </div>

<script>
(() => {
  const ADDRESS_BOOK = ${bookJson};
  const recipients = new Set();
  const pendingFiles = [];
  const MAX_FILES = 8;
  const MAX_FILE_BYTES = 8 * 1024 * 1024;
  const MAX_TOTAL_BYTES = 15 * 1024 * 1024;

  const $ = (id) => document.getElementById(id);
  const loginView = $("loginView");
  const appView = $("appView");
  const composePanel = $("composePanel");
  const historyPanel = $("historyPanel");
  const chipsEl = $("chips");
  const bookEl = $("addressBook");
  const toInput = $("toInput");
  const loginStatus = $("loginStatus");
  const loginAlert = $("loginAlert");
  const shellEl = $("shell");
  const topMark = $("topMark");
  const sendStatus = $("sendStatus");
  const attachListEl = $("attachList");
  const fileInput = $("fileInput");
  const attachDrop = $("attachDrop");
  const historyListEl = $("historyList");
  const historyDetailEl = $("historyDetail");
  const previewOverlay = $("previewOverlay");
  const previewMeta = $("previewMeta");
  const previewFrame = $("previewFrame");
  const resultOverlay = $("resultOverlay");
  const resultPanel = $("resultPanel");
  const resultMark = $("resultMark");
  const resultTitle = $("resultTitle");
  const resultMsg = $("resultMsg");
  const loginBtn = $("loginBtn");
  const passwordInput = $("password");
  let loginLockTimer = null;

  function clearLoginLockout() {
    if (loginLockTimer) {
      clearInterval(loginLockTimer);
      loginLockTimer = null;
    }
    loginBtn.disabled = false;
    passwordInput.disabled = false;
  }

  function startLoginLockout(retryAfterSec) {
    clearLoginLockout();
    let sec = Math.max(1, Math.floor(Number(retryAfterSec) || 900));
    loginBtn.disabled = true;
    passwordInput.disabled = true;

    const tick = () => {
      if (sec <= 0) {
        clearLoginLockout();
        setLoginMessage("");
        return;
      }
      const m = Math.floor(sec / 60);
      const s = sec % 60;
      const timeTxt = m > 0 ? m + "m " + String(s).padStart(2, "0") + "s" : s + "s";
      setLoginMessage("Too many attempts. Try again in " + timeTxt + ".", "err");
      sec -= 1;
    };
    tick();
    loginLockTimer = setInterval(tick, 1000);
  }

  function clearLegacyAuth() {
    try {
      localStorage.removeItem("xuxu_mail_token");
      localStorage.removeItem("pulse_mail_token");
    } catch {}
  }

  function setLoginMessage(text, kind) {
    const msg = text || "";
    loginAlert.textContent = kind === "err" ? msg : "";
    loginAlert.className = "login-alert" + (kind === "err" && msg ? " err" : "");
    if (kind !== "err") {
      setStatus(loginStatus, msg, kind);
    } else {
      setStatus(loginStatus, "");
    }
  }

  function showApp(on) {
    loginView.classList.toggle("hidden", on);
    appView.classList.toggle("hidden", !on);
    topMark.classList.add("hidden");
    shellEl.classList.toggle("login-mode", !on);
    document.documentElement.lang = on ? "zh-CN" : "en";
    if (on) setLoginMessage("");
  }

  function resetHistoryDetail() {
    historyDetailEl.classList.remove("hidden");
    historyDetailEl.innerHTML = '<div class="hist-detail-empty"><strong>选择一条记录</strong>在左侧列表中点击，查看收件人、正文与附件</div>';
  }
  function setStatus(el, text, kind) {
    el.textContent = text || "";
    el.className = "status" + (kind ? " " + kind : "");
  }
  function formatSize(n) {
    if (n < 1024) return n + " B";
    if (n < 1024 * 1024) return (n / 1024).toFixed(1) + " KB";
    return (n / (1024 * 1024)).toFixed(1) + " MB";
  }
  function formatTime(iso) {
    try {
      return new Date(iso).toLocaleString("zh-CN", { hour12: false });
    } catch { return iso; }
  }
  function escapeText(s) {
    return String(s).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;");
  }

  function switchTab(which) {
    const compose = which === "compose";
    $("tabCompose").classList.toggle("active", compose);
    $("tabHistory").classList.toggle("active", !compose);
    composePanel.classList.toggle("hidden", !compose);
    historyPanel.classList.toggle("hidden", compose);
    if (!compose) {
      resetHistoryDetail();
      loadHistory();
    }
  }

  async function fileToAttachment(file) {
    const buf = await file.arrayBuffer();
    const bytes = new Uint8Array(buf);
    let binary = "";
    const chunk = 0x8000;
    for (let i = 0; i < bytes.length; i += chunk) {
      binary += String.fromCharCode.apply(null, bytes.subarray(i, i + chunk));
    }
    return {
      name: file.name,
      content: btoa(binary),
      size: file.size,
      type: file.type || "application/octet-stream",
    };
  }

  function totalAttachBytes() {
    return pendingFiles.reduce((sum, f) => sum + f.size, 0);
  }

  function renderAttachments() {
    attachListEl.innerHTML = "";
    for (let i = 0; i < pendingFiles.length; i++) {
      const f = pendingFiles[i];
      const row = document.createElement("div");
      row.className = "attach-item";
      const isImg = f.type && f.type.startsWith("image/");
      const visual = isImg
        ? '<img class="attach-thumb" alt="" src="data:' + f.type + ';base64,' + f.content + '">'
        : '<span class="attach-icon" aria-hidden="true">DOC</span>';
      row.innerHTML = visual +
        '<div class="attach-meta"><div class="attach-name">' + escapeText(f.name) + '</div><div class="attach-size">' + formatSize(f.size) + '</div></div>' +
        '<button type="button" aria-label="remove">&times;</button>';
      row.querySelector("button").onclick = () => {
        pendingFiles.splice(i, 1);
        renderAttachments();
      };
      attachListEl.appendChild(row);
    }
  }

  async function addFiles(fileList) {
    for (const file of fileList) {
      if (pendingFiles.length >= MAX_FILES) {
        setStatus(sendStatus, "最多 " + MAX_FILES + " 个附件", "err");
        break;
      }
      if (file.size > MAX_FILE_BYTES) {
        setStatus(sendStatus, file.name + " 超过 8MB", "err");
        continue;
      }
      if (totalAttachBytes() + file.size > MAX_TOTAL_BYTES) {
        setStatus(sendStatus, "附件总大小超过 15MB", "err");
        break;
      }
      try {
        pendingFiles.push(await fileToAttachment(file));
      } catch (err) {
        setStatus(sendStatus, "读取文件失败: " + file.name, "err");
      }
    }
    renderAttachments();
  }

  function renderChips() {
    chipsEl.innerHTML = "";
    for (const addr of recipients) {
      const chip = document.createElement("span");
      chip.className = "chip";
      chip.innerHTML = "<span></span><button type='button' aria-label='remove'>&times;</button>";
      chip.querySelector("span").textContent = addr;
      chip.querySelector("button").onclick = () => {
        recipients.delete(addr);
        renderChips();
        renderBook();
      };
      chipsEl.appendChild(chip);
    }
  }

  function renderBook() {
    bookEl.innerHTML = "";
    for (const entry of ADDRESS_BOOK) {
      const btn = document.createElement("button");
      btn.type = "button";
      const active = recipients.has(entry.address.toLowerCase());
      btn.classList.toggle("active", active);
      btn.textContent = entry.note ? entry.note + " · " + entry.address : entry.address;
      btn.onclick = () => {
        const a = entry.address.toLowerCase();
        if (recipients.has(a)) recipients.delete(a);
        else recipients.add(a);
        renderChips();
        renderBook();
      };
      bookEl.appendChild(btn);
    }
  }

  function addTypedRecipient() {
    const raw = toInput.value.trim().replace(/[,;]+$/, "");
    if (!raw) return;
    const parts = raw.split(/[,;\\s]+/).map((s) => s.trim().toLowerCase()).filter(Boolean);
    for (const p of parts) {
      if (!p.includes("@")) continue;
      recipients.add(p);
    }
    toInput.value = "";
    renderChips();
    renderBook();
  }

  toInput.addEventListener("keydown", (e) => {
    if (e.key === "Enter" || e.key === "," || e.key === ";") {
      e.preventDefault();
      addTypedRecipient();
    }
    if (e.key === "Backspace" && !toInput.value && recipients.size) {
      const arr = [...recipients];
      recipients.delete(arr[arr.length - 1]);
      renderChips();
      renderBook();
    }
  });
  toInput.addEventListener("blur", addTypedRecipient);

  async function api(path, body) {
    const res = await fetch(path, {
      method: "POST",
      credentials: "same-origin",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body || {}),
    });
    const data = await res.json().catch(() => ({}));
    return { res, data };
  }

  async function handleLogin() {
    if (loginBtn.disabled) return;
    const password = passwordInput.value;
    setLoginMessage("Verifying…");
    const { res, data } = await api("/api/login", { password });
    if (!res.ok) {
      if (res.status === 429 && data.retryAfterSec) {
        startLoginLockout(data.retryAfterSec);
        return;
      }
      setLoginMessage(data.error || "Sign-in failed", "err");
      return;
    }
    clearLoginLockout();
    passwordInput.value = "";
    setLoginMessage("");
    showApp(true);
    switchTab("compose");
  }

  $("loginForm").addEventListener("submit", (e) => {
    e.preventDefault();
    handleLogin();
  });

  $("tabCompose").onclick = () => switchTab("compose");
  $("tabHistory").onclick = () => switchTab("history");
  $("refreshHistoryBtn").onclick = () => loadHistory();

  $("logoutBtn").onclick = async () => {
    await api("/api/logout", {});
    passwordInput.value = "";
    showApp(false);
    resetHistoryDetail();
  };

  $("addFilesBtn").onclick = (e) => {
    e.stopPropagation();
    fileInput.click();
  };
  if (attachDrop) {
    attachDrop.addEventListener("click", (e) => {
      if (e.target.closest("button")) return;
      fileInput.click();
    });
    ["dragenter", "dragover"].forEach((evt) => {
      attachDrop.addEventListener(evt, (e) => {
        e.preventDefault();
        e.stopPropagation();
        attachDrop.classList.add("dragover");
      });
    });
    attachDrop.addEventListener("dragleave", (e) => {
      e.preventDefault();
      if (!attachDrop.contains(e.relatedTarget)) attachDrop.classList.remove("dragover");
    });
    attachDrop.addEventListener("drop", async (e) => {
      e.preventDefault();
      e.stopPropagation();
      attachDrop.classList.remove("dragover");
      const files = e.dataTransfer && e.dataTransfer.files;
      if (files && files.length) await addFiles(files);
    });
  }
  fileInput.onchange = async () => {
    if (fileInput.files?.length) await addFiles(fileInput.files);
    fileInput.value = "";
  };

  const composeFocusHint = $("composeFocusHint");
  const composeFields = document.querySelectorAll(".compose-field[data-field]");
  const mobileComposeMq = window.matchMedia("(max-width: 860px)");

  function updateComposeFocusHint(label) {
    if (!composeFocusHint) return;
    if (!mobileComposeMq.matches || !label) {
      composeFocusHint.textContent = "";
      composeFocusHint.classList.add("hidden");
      return;
    }
    composeFocusHint.textContent = "正在编辑：" + label;
    composeFocusHint.classList.remove("hidden");
  }

  composeFields.forEach((field) => {
    field.addEventListener("focusin", () => {
      composeFields.forEach((f) => f.classList.remove("is-focused"));
      field.classList.add("is-focused");
      updateComposeFocusHint(field.getAttribute("data-label") || "");
      if (mobileComposeMq.matches) {
        window.setTimeout(() => {
          const layout = field.closest(".compose-layout");
          if (layout && layout.scrollHeight > layout.clientHeight) {
            const top = field.offsetTop - layout.offsetTop - 8;
            layout.scrollTo({ top: Math.max(0, top), behavior: "smooth" });
          } else {
            field.scrollIntoView({ block: "nearest", behavior: "smooth" });
          }
        }, 280);
      }
    });
    field.addEventListener("focusout", (e) => {
      if (!field.contains(e.relatedTarget)) {
        field.classList.remove("is-focused");
        const next = e.relatedTarget && composePanel.contains(e.relatedTarget)
          ? e.relatedTarget.closest(".compose-field[data-field]")
          : null;
        if (!next) updateComposeFocusHint("");
      }
    });
  });

  mobileComposeMq.addEventListener("change", () => {
    const active = document.activeElement;
    const focused = active && composePanel.contains(active)
      ? active.closest(".compose-field[data-field]")
      : null;
    updateComposeFocusHint(focused ? focused.getAttribute("data-label") || "" : "");
  });

  function clearComposeForm() {
    recipients.clear();
    pendingFiles.length = 0;
    renderChips();
    renderBook();
    renderAttachments();
    $("subject").value = "";
    $("body").value = "";
    setStatus(sendStatus, "");
  }

  function openResultModal(ok, title, message) {
    resultPanel.classList.toggle("ok", ok);
    resultPanel.classList.toggle("err", !ok);
    resultMark.textContent = ok ? "✓" : "×";
    resultTitle.textContent = title;
    resultMsg.textContent = message || "";
    resultOverlay.classList.remove("hidden");
  }

  function closeResultModal() {
    resultOverlay.classList.add("hidden");
  }

  $("resultCloseBtn").onclick = () => closeResultModal();
  resultOverlay.addEventListener("click", (e) => {
    if (e.target === resultOverlay) closeResultModal();
  });

  $("clearBtn").onclick = () => clearComposeForm();

  async function loadHistory() {
    historyListEl.innerHTML = '<div class="hist-empty"><strong>加载中</strong>正在读取发送记录…</div>';
    resetHistoryDetail();
    const { res, data } = await api("/api/history", { limit: 10 });
    if (!res.ok) {
      historyListEl.innerHTML = '<div class="hist-empty"><strong>加载失败</strong>' + escapeText(data.error || "请稍后重试") + '</div>';
      return;
    }
    const items = data.items || [];
    if (!items.length) {
      historyListEl.innerHTML = '<div class="hist-empty"><strong>暂无发送记录</strong>发送成功后会显示在这里</div>';
      return;
    }
    historyListEl.innerHTML = "";
    for (const item of items) {
      const row = document.createElement("div");
      row.className = "hist-row";
      row.dataset.id = item.id;
      const statusCls = item.ok ? "ok" : "fail";
      const statusTxt = item.ok ? "成功" : "失败";
      const attach = item.attachmentNames?.length
        ? " · " + item.attachmentNames.length + " 个附件"
        : "";
      row.innerHTML =
        '<div class="hist-meta">' + formatTime(item.createdAt) +
        ' · <span class="' + statusCls + '">' + statusTxt + '</span>' + attach + '</div>' +
        '<div class="hist-subject">' + escapeText(item.subject) + '</div>' +
        '<div class="hist-to">' + escapeText((item.to || []).join(", ")) + '</div>';
      row.onclick = () => showHistoryDetail(item.id, row);
      historyListEl.appendChild(row);
    }
  }

  async function showHistoryDetail(id, rowEl) {
    for (const el of historyListEl.querySelectorAll(".hist-row")) el.classList.remove("active");
    rowEl.classList.add("active");
    historyDetailEl.classList.remove("hidden");
    historyDetailEl.innerHTML = '<div class="hist-detail-empty">加载详情…</div>';
    const { res, data } = await api("/api/history/detail", { id });
    if (!res.ok) {
      historyDetailEl.innerHTML = '<div class="hist-detail-empty"><strong>加载失败</strong>' + escapeText(data.error || "请稍后重试") + '</div>';
      return;
    }
    const e = data.entry;
    const attachLine = e.attachmentNames?.length
      ? e.attachmentNames.map((n, i) => n + " (" + formatSize(e.attachmentSizes?.[i] || 0) + ")").join(", ")
      : "无";
    historyDetailEl.innerHTML =
      "<h4>详情</h4>" +
      '<div class="kv">' +
      "<div><strong>时间</strong><span>" + escapeText(formatTime(e.createdAt)) + "</span></div>" +
      "<div><strong>From</strong><span>" + escapeText(e.fromName) + " &lt;" + escapeText(e.fromEmail) + "&gt;</span></div>" +
      "<div><strong>To</strong><span>" + escapeText((e.to || []).join(", ")) + "</span></div>" +
      "<div><strong>状态</strong><span>" + (e.ok ? "成功" : ("失败" + (e.error ? " · " + escapeText(e.error) : ""))) + "</span></div>" +
      (e.messageId ? "<div><strong>ID</strong><span>" + escapeText(e.messageId) + "</span></div>" : "") +
      "<div><strong>附件</strong><span>" + escapeText(attachLine) + "</span></div>" +
      "</div>" +
      "<h4>正文</h4><pre>" + escapeText(e.body || "(无正文)") + "</pre>";
  }

  function collectComposePayload() {
    addTypedRecipient();
    return {
      to: [...recipients],
      subject: $("subject").value.trim(),
      body: $("body").value,
      fromName: $("fromName").value.trim(),
      attachments: pendingFiles.map((f) => ({ name: f.name, content: f.content, size: f.size })),
    };
  }

  function validateCompose(payload, el) {
    if (!payload.to.length) {
      setStatus(el, "请至少添加一个收件人", "err");
      return false;
    }
    if (!payload.subject) {
      setStatus(el, "请填写主题", "err");
      return false;
    }
    if (!payload.body.trim() && !payload.attachments.length) {
      setStatus(el, "正文或附件至少一项", "err");
      return false;
    }
    return true;
  }

  function buildLocalAttachPreviewHtml() {
    if (!pendingFiles.length) return "";
    const chips = pendingFiles.map((f) => {
      const label = escapeText(f.name) + " · " + formatSize(f.size);
      return '<span class="preview-attach-chip">' + label + "</span>";
    }).join("");
    return '<div class="preview-attach-chips">' + chips + "</div>";
  }

  function openPreviewModal(data) {
    previewMeta.innerHTML =
      "<div><strong>From</strong>" + escapeText(data.fromName) + " &lt;" + escapeText(data.fromEmail) + "&gt;</div>" +
      "<div><strong>To</strong>" + escapeText((data.to || []).join(", ")) + "</div>" +
      "<div><strong>Subject</strong>" + escapeText(data.subject) + "</div>" +
      (pendingFiles.length ? "<div><strong>附件</strong></div>" + buildLocalAttachPreviewHtml() : "");
    previewFrame.srcdoc = data.html || "";
    previewOverlay.classList.remove("hidden");
  }

  function closePreviewModal() {
    previewOverlay.classList.add("hidden");
    previewFrame.srcdoc = "";
  }

  $("previewBtn").onclick = async () => {
    const payload = collectComposePayload();
    if (!validateCompose(payload, sendStatus)) return;
    setStatus(sendStatus, "生成预览…");
    const { res, data } = await api("/api/preview", payload);
    if (!res.ok) {
      setStatus(sendStatus, data.error || "预览失败", "err");
      if (res.status === 401) showApp(false);
      return;
    }
    setStatus(sendStatus, "");
    openPreviewModal(data);
  };

  $("previewCloseBtn").onclick = () => closePreviewModal();
  previewOverlay.addEventListener("click", (e) => {
    if (e.target === previewOverlay) closePreviewModal();
  });
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      if (!previewOverlay.classList.contains("hidden")) closePreviewModal();
      if (!resultOverlay.classList.contains("hidden")) closeResultModal();
    }
  });

  $("previewSendBtn").onclick = async () => {
    closePreviewModal();
    $("sendBtn").click();
  };

  $("sendBtn").onclick = async () => {
    const payload = collectComposePayload();
    if (!validateCompose(payload, sendStatus)) return;

    const btn = $("sendBtn");
    btn.disabled = true;
    setStatus(sendStatus, "发送中…");
    try {
      const { res, data } = await api("/api/send", payload);
      if (!res.ok) {
        setStatus(sendStatus, "");
        openResultModal(false, "发送失败", data.error || "请稍后重试");
        if (res.status === 401) showApp(false);
        return;
      }
      clearComposeForm();
      const detail = data.messageId ? "Message ID: " + data.messageId : "邮件已成功发出。";
      openResultModal(true, "发送成功", detail);
    } catch (err) {
      setStatus(sendStatus, "");
      openResultModal(false, "发送失败", String(err));
    } finally {
      btn.disabled = false;
    }
  };

  function initMdCopy() {
    const root = $("mdSyntaxRef");
    if (!root) return;
    root.querySelectorAll("code.md-copy").forEach((el) => {
      el.addEventListener("click", async () => {
        const text = el.getAttribute("data-copy") || el.textContent || "";
        try {
          await navigator.clipboard.writeText(text);
        } catch {
          const ta = document.createElement("textarea");
          ta.value = text;
          ta.setAttribute("readonly", "");
          ta.style.position = "fixed";
          ta.style.left = "-9999px";
          document.body.appendChild(ta);
          ta.select();
          document.execCommand("copy");
          document.body.removeChild(ta);
        }
        el.classList.add("copied");
        const prevTitle = el.getAttribute("title");
        el.setAttribute("title", "已复制");
        setTimeout(() => {
          el.classList.remove("copied");
          if (prevTitle) el.setAttribute("title", prevTitle);
        }, 1200);
      });
    });
  }

  (async () => {
    clearLegacyAuth();
    renderChips();
    renderBook();
    renderAttachments();
    initMdCopy();
    const { res } = await api("/api/session", {});
    showApp(res.ok);
  })();
})();
</script>
</body>
</html>`;
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
