# 配置说明

Custom Mail 通过仓库根目录的 **`config/mail.json`** 配置。Worker 在构建 / 部署时读取该文件，修改后需重新 `npm run deploy`。

空字段或 `false` 的功能/版式开关会**省略**对应区块。最少需要 `host`、`app.title`、`mail.fromEmail` / `fromNameDefault`。

## `plugins` — 主题 / 版式 / 发信服务商

全部插件编译进 Worker，部署时在 `mail.json` 里选一个即可。未配置的区块（页脚联系方式、站点、Logo、附件栏等）不会渲染。

| 字段 | 可选值 |
|------|--------|
| `provider` | `brevo` `resend` `sendgrid` `mailgun` `postmark` `mailersend` `smtp2go` `sparkpost` |
| `theme` | `forest` `midnight` `ocean` `paper` `rose` `slate` |
| `layout` | `card` `minimal` `banner` `digest` |

对应密钥见下文「环境密钥」。`fromEmail` 须在所选服务商处授权。Docker 下可用环境变量 `MAIL_PROVIDER` 覆盖 `plugins.provider`，无需重建镜像。

页眉颜色用 `brand.heroFrom` / `heroTo` / `headerText`（也可继续用 `tile` / `tileEdge`）。Logo 用 `site.logoPath` 或 `logoUrl`；不配则控制台显示品牌名首字母，邮件里不放图片。浏览器标签图标用 `site.faviconPath`。

## 文件结构

```jsonc
{
  "host": "mail.example.com",
  "plugins": { "provider": "brevo", "theme": "forest", "layout": "banner" },
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
| `logoPath` / `logoUrl` | Logo。空则用品牌名首字母，不用内置信封图 |
| `faviconPath` | 浏览器标签图标。空则用 `logoPath`，再退回生成的 `/favicon.svg` |

Fork 后请替换 `public/images/logo.svg` 为自己的标志。

## `brand` — 颜色覆盖

空字符串保留主题默认：`tile`、`tileEdge`、`heroFrom`、`heroTo`、`headerText`、`accent`、`accentDeep`、`accentSoft`、`cream`、`paper`、`ink`、`muted`、`line`、`siteBlue`。

## `i18n` / `syntax` / `addressBook`

控制台文案默认英文，可在 `i18n` 下覆盖任意键。语法芯片在 `syntax.chips`。通讯录为 `{ "address", "note" }`。

## 环境密钥（不在 mail.json）

| Secret | 位置 | 用途 |
|--------|------|------|
| `ADMIN_PASSWORD` | `.dev.vars` / Worker secret | 登录密码 |
| 服务商 API Key | `.dev.vars` / Worker secret | 与 `plugins.provider` 对应，或使用 `MAIL_API_KEY` |
| `MAIL_PROVIDER` | `.dev.vars` / Worker 环境 | 运行时覆盖 `plugins.provider`（Docker 常用） |

切勿将密钥提交到 Git。

## 修改配置后

```bash
npm run typecheck
npm run deploy
```
