# XuXu Mail (pulse-mail)

独立的网页发信服务：**Cloudflare Workers** + **Brevo**，部署在 `mail.xuxuclassmate.com`。

## 功能

- 固定发件域：`noreply@xuxuclassmate.com`（显示名可改）
- 收件人：地址簿点选 + 手动输入
- Markdown 正文、附件、预览、发送记录
- 会话登录（`ADMIN_PASSWORD`）

## 开发

```bash
npm install
cp .dev.vars.example .dev.vars   # 填写 ADMIN_PASSWORD、BREVO_API_KEY
npm run dev
```

本地默认：<http://localhost:8790>

## 部署

```bash
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
npm run deploy
```

## 环境变量

| 名称 | 说明 |
|------|------|
| `ADMIN_PASSWORD` | 登录密码（Secret） |
| `BREVO_API_KEY` | Brevo 发信 API Key（Secret） |
| `EMAIL_FROM_NAME` | 发件显示名（wrangler vars） |
| `ADDRESS_BOOK` | 地址簿 JSON（wrangler vars） |

## 技术栈

- TypeScript, Cloudflare Workers, KV, Workers Assets
- Brevo Transactional Email API
