# 快速开始

## 环境要求

- Rust **stable**（含 `wasm32-unknown-unknown`）与 `worker-build` 0.8.5
- Node.js **22+**（Wrangler）
- Cloudflare 账号（生产部署）
- 所选发信服务商的 API Key（默认 Brevo；见[配置说明](./config)）

## 安装并运行

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
npm install
cp .dev.vars.example .dev.vars
```

编辑 `.dev.vars`：

```bash
ADMIN_PASSWORD=你的强密码
BREVO_API_KEY=你的-brevo-key
# 或 MAIL_PROVIDER=resend 与 RESEND_API_KEY=...
ALLOW_ANY_HOST=1
```

启动本地 Worker：

```bash
npm run dev
```

打开 **http://localhost:8790**，用 `ADMIN_PASSWORD` 登录。

## 文档站点（可选）

```bash
cd docs-site
npm install
npm run docs:dev
```

## 下一步

- [配置说明](./config)
- [部署](./deploy)
