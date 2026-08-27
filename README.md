# Custom Mail

独立的网页发信服务：**Cloudflare Workers** + **Brevo**，部署在 `mail.xuxuclassmate.com`。

## 功能

- 固定发件域、可配置品牌与文案
- 收件人地址簿、Markdown 正文、附件、预览、发送记录
- 会话登录（`ADMIN_PASSWORD`）

## 客制化配置

编辑 **`config/mail.json`**（品牌、域名、发件人、地址簿、登录页文案等）。

注意：`config/mail.json` 里的 `host` 需与 `wrangler.jsonc` 中 `routes` 的域名一致。

## 开发

```bash
npm install
cp .dev.vars.example .dev.vars   # ADMIN_PASSWORD、BREVO_API_KEY
npm run dev
```

本地：<http://localhost:8790>

## 部署

### 手动

```bash
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
npm run deploy
```

### GitHub → Cloudflare 自动部署

推送到 `main` 分支后，GitHub Actions 会自动 `wrangler deploy`。

在仓库 **Settings → Secrets and variables → Actions** 配置：

| Secret | 说明 |
|--------|------|
| `CLOUDFLARE_API_TOKEN` | Cloudflare API Token（需 Workers Scripts 写权限） |
| `CLOUDFLARE_ACCOUNT_ID` | Cloudflare Account ID |

Worker 上的 Secrets（**不在 GitHub 里**，在 Cloudflare Dashboard 或 wrangler 设置一次即可）：

| Secret | 说明 |
|--------|------|
| `ADMIN_PASSWORD` | 登录密码 |
| `BREVO_API_KEY` | Brevo 发信 API Key |

创建 API Token：<https://dash.cloudflare.com/profile/api-tokens> → Create Token → Edit Cloudflare Workers 模板。

## 技术栈

TypeScript · Cloudflare Workers · KV · Workers Assets · Brevo
