# 快速开始

## 环境要求

- Rust **stable**（含 `wasm32-unknown-unknown`）与 `worker-build` 0.8.5
- Node.js **22+**（Wrangler）
- Cloudflare 账号（生产部署）
- 所选发信服务商的 API Key（默认 Brevo；见[插件](./plugins)与[配置说明](./config)）

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
# 可选：MAIL_THEME=nord  MAIL_LAYOUT=compact  MAIL_LOGO=monogram
ALLOW_ANY_HOST=1
```

启动本地 Worker：

```bash
npm run dev
```

打开 **http://localhost:8790**，用 `ADMIN_PASSWORD` 登录。撰写、预览、附件、历史见[控制台](./console)。

## 其他运行方式

| 路径 | 适用 |
|------|------|
| [Docker](./docker) | 不装 Rust。`docker run` 或 Compose，端口 8787。 |
| [OpenClaw 技能](./openclaw) | 让 agent 拉起容器：`clawhub install custom-mail-skill`。 |
| [部署](./deploy) | 生产环境 Cloudflare Workers。 |

## 选择插件

把文件放到 `plugins/`（主题、版式、Logo、服务商元数据），再在 `config/mail.json` 里选中 id。未配置的区块不会出现。详见[插件](./plugins)。

## 文档站点（可选）

```bash
cd docs-site
npm install
npm run docs:dev
```

## 下一步

- [配置说明](./config)
- [部署](./deploy)
