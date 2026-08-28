# 配置说明

Custom Mail 通过仓库根目录的 **`config/mail.json`** 与 **`plugins/`** 下的可插拔文件配置。Worker 在构建 / 部署时编译这些文件，修改后需重新 `npm run deploy`。目录说明见[插件](./plugins)。

空字段或 `false` 的功能/版式开关会**省略**对应区块。最少需要 `host`、`app.title`、`mail.fromEmail` / `fromNameDefault`。

无需重建即可覆盖插件槽：`MAIL_PROVIDER`、`MAIL_THEME`、`MAIL_LAYOUT`、`MAIL_LOGO`、`MAIL_CONFIG_JSON`。

## `plugins` — 服务商 / 主题 / 版式 / Logo

把 JSON 放到 `plugins/providers|themes|layouts|features/`，把 Logo 文件放到 `plugins/logos/`。`mail.json` 里选出当前使用的 id。未配置的区块不会渲染。`GET /api/health` 的 `available` 列出已编译目录。

| 字段 | 说明 |
|------|------|
| `provider` | `plugins/providers/` 的 id。内置：`brevo` `resend` `sendgrid` `mailgun` `postmark` `mailersend` `smtp2go` `sparkpost` |
| `theme` | `plugins/themes/` 的 id。内置：`forest` `midnight` `ocean` `paper` `rose` `slate` `aurora` `sunset` `nord` |
| `layout` | `plugins/layouts/` 的 id。内置：`card` `minimal` `banner` `digest` `compact` |
| `logo` | `auto`（有图用图，否则首字母，再否则省略）· `image` · `monogram` · `none` |

对应密钥见下文「环境密钥」。`fromEmail` 须在所选服务商处授权。新增主题/版式只需 JSON；新增发信 HTTP API 还要在 Rust 里加发送适配。

页眉颜色用 `brand.heroFrom` / `heroTo` / `headerText`（也可继续用 `tile` / `tileEdge`）。`config/overlays/*.json` 会在编译时深度合并进 `mail.json`。

## 文件结构

```jsonc
{
  "host": "mail.example.com",
  "plugins": { "provider": "brevo", "theme": "forest", "layout": "banner", "logo": "image" },
  "features": { "attachments": true, "history": true, "addressBook": true, "markdown": true, "syntaxHelp": true },
  "layout": { "showHeader": true, "showLogo": true, "showSubject": true, "showFrom": true, "showFooterContact": true, "showFooterSite": true },
  "app": { /* 界面文案 */ },
  "mail": { /* 发件人 */ },
  "site": { /* Logo / favicon / 页脚站点 */ },
  "brand": { /* 覆盖主题色 */ },
  "i18n": { /* 控制台文案；缺省为英文 */ },
  "syntax": { /* Markdown 提示芯片 */ },
  "addressBook": [ /* 预设收件人 */ ]
}
```

## `host`

用户浏览器访问的公网域名，须与 `wrangler.jsonc` 的 `routes` 一致。

## `app` — 界面文案

| 字段 | 用途 |
|------|------|
| `title` | 顶栏、登录页、浏览器标签名称 |
| `subtitle` | 顶栏副标题（空则隐藏） |
| `locale` | `html lang` 与日期格式 |
| `loginTagline` | 登录页短标语 |
| `loginHeadlineBefore` / `loginHeadlineEm` | 登录大标题 |
| `loginLead` | 登录说明（空则隐藏） |
| `loginPoints` | 登录页要点（空则隐藏） |
| `loginFormTitle` / `loginFormSub` | 登录卡片标题 |

均为纯文本，标签中不解析 HTML。

## `mail` — 发信

| 字段 | 说明 |
|------|------|
| `fromEmail` | 固定发件地址（须在所选服务商授权） |
| `fromNameDefault` | 默认发件显示名 |
| `contactEmail` | 页脚联系邮箱；空则隐藏 |
| `tag` | 服务商活动/分析标签（仍接受 `brevoTag`） |
| `providerDomain` | Mailgun 发信域名 |

## `site` — 品牌

| 字段 | 说明 |
|------|------|
| `url` / `label` | 页脚站点链接；`url` 为空则隐藏 |
| `brandName` | 组织名称（缺省用 `app.title`） |
| `logoPath` / `logoUrl` | Logo。空则用 `plugins/logos/` 中的文件，再退回品牌名首字母 |
| `faviconPath` | 浏览器标签图标。空则用 `logoPath`，再退回生成的 `/favicon.svg` |

Fork 后请替换 `public/images/logo.svg` 或在 `plugins/logos/` 放入自己的标志。

## `brand` — 颜色覆盖

空字符串保留主题默认：`tile`、`tileEdge`、`heroFrom`、`heroTo`、`headerText`、`accent`、`accentDeep`、`accentSoft`、`cream`、`paper`、`ink`、`muted`、`line`、`siteBlue`。

## `i18n` / `syntax` / `addressBook`

控制台文案默认英文，可在 `i18n` 下覆盖任意键。语法芯片在 `syntax.chips`。通讯录为 `{ "address", "note" }`。

## 环境密钥（不在 mail.json）

| Secret | 位置 | 用途 |
|--------|------|------|
| `ADMIN_PASSWORD` | `.dev.vars` / Worker secret | 登录密码 |
| 服务商 API Key | `.dev.vars` / Worker secret | 与 `plugins.provider` 对应，或使用 `MAIL_API_KEY` |
| `MAIL_PROVIDER` / `MAIL_THEME` / `MAIL_LAYOUT` / `MAIL_LOGO` | `.dev.vars` / Docker | 运行时覆盖插件槽 |
| `MAIL_CONFIG_JSON` | `.dev.vars` / Docker | 运行时 JSON 覆盖 |

切勿将密钥提交到 Git。

## 修改配置后

```bash
npm run typecheck
npm run deploy
```
