# 配置说明

Custom Mail 通过仓库根目录的 **`config/mail.json`** 配置。Worker 在构建 / 部署时读取该文件，修改后需重新 `npm run deploy`。

## 文件结构

```jsonc
{
  "host": "mail.example.com",       // 须与 wrangler 路由一致
  "app": { /* 界面文案 */ },
  "mail": { /* 发件人 + Brevo */ },
  "site": { /* 页脚 / 品牌链接 */ },
  "brand": { /* Logo 色块 */ },
  "addressBook": [ /* 预设收件人 */ ]
}
```

## `host`

用户浏览器访问的公网域名，须与 `wrangler.jsonc` 的 `routes` 一致。

## `app` — 界面文案

| 字段 | 用途 |
|------|------|
| `title` | 顶栏与登录页名称 |
| `subtitle` | 顶栏副标题（桌面） |
| `loginTagline` | 登录页短标语 |
| `loginHeadlineBefore` / `loginHeadlineEm` | 登录大标题 |
| `loginLead` | 登录说明 |
| `loginPoints` | 登录页要点列表 |
| `loginFormTitle` / `loginFormSub` | 登录卡片标题 |

均为纯文本，标签中不解析 HTML。

## `mail` — 发信

| 字段 | 说明 |
|------|------|
| `fromEmail` | 固定发件地址（须在 Brevo 授权） |
| `fromNameDefault` | 默认发件显示名 |
| `contactEmail` | 联系 / 支持邮箱 |
| `brevoTag` | Brevo 分析标签 |

## `site` — 品牌外链

| 字段 | 说明 |
|------|------|
| `url` / `label` | 界面中的站点链接 |
| `brandName` | 组织名称 |
| `logoPath` | `public/` 下路径 |

## `brand` — 颜色

供 `src/brand.rs` 生成 Logo 色块：

| Key | 作用 |
|-----|------|
| `tile` / `tileEdge` | Logo 背景 |
| `accent` | 主强调色 |
| `cream` | 页面底色倾向 |
| `siteBlue` | Logo 点缀色 |

## `addressBook`

`{ "address", "note" }` 数组，在收件人栏显示快捷芯片。

## 环境密钥（不在 mail.json）

| Secret | 位置 | 用途 |
|--------|------|------|
| `ADMIN_PASSWORD` | `.dev.vars` / Worker secret | 登录密码 |
| `BREVO_API_KEY` | `.dev.vars` / Worker secret | Brevo API |

切勿将密钥提交到 Git。

## 修改配置后

```bash
npm run typecheck
npm run deploy
```
