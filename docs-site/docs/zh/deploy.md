# 部署

Custom Mail 以单个 **Cloudflare Worker** 运行，配合 **KV** 与 **Workers Assets**。

## 前置条件

- 已开通 Workers 的 Cloudflare 账号
- [Brevo](https://www.brevo.com/)（默认）或其他已支持的发信服务商 — 见[配置说明](./config)
- Rust stable（`wasm32-unknown-unknown`）与 `worker-build` 0.8.5
- Node.js 22+ 与 `npm`

## 1. 配置产品

编辑 `config/mail.json`：

- 设置 `host` 为邮件子域名
- 设置 `mail.fromEmail` 为所选服务商已授权地址
- 自定义 `plugins`、`app`、`brand`、`site`、`i18n`、`addressBook`

编辑 `wrangler.jsonc`：

- `name` — Worker 名称（如 `custom-mail`）
- `routes` — 与 `mail.json` 的 host 一致
- `kv_namespaces` — KV 命名空间 ID

```bash
npx wrangler kv namespace create MAIL_LOG_KV
```

将返回的 `id` 写入 `wrangler.jsonc`。

## 2. 密钥

**本地** — `.dev.vars`：

```bash
cp .dev.vars.example .dev.vars
```

**生产**：

```bash
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY   # 或与 plugins.provider 对应的密钥
```

## 3. 发布

```bash
npm run typecheck
npm run deploy
```

访问 `https://<host>` 并登录。

## Docker（本地）

发布的镜像用于本地试用。传入 `ADMIN_PASSWORD` 与当前服务商的 API 密钥。`MAIL_PROVIDER` 可覆盖 `plugins.provider` 而无需重建。品牌、主题、版式与 Logo 在构建镜像时从 `config/mail.json` 写入。

详见仓库中的 `docker/DOCKERHUB.md` 与 `skills/custom-mail/SKILL.md`。

## 自定义域名

```jsonc
"routes": [
  { "pattern": "mail.example.com", "custom_domain": true }
]
```

## GitHub Actions

| Workflow | 触发 | 用途 |
|----------|------|------|
| CI | push / PR → `main` | typecheck |
| CodeQL | push / PR / 每周 | 安全扫描 |
| Docs | 变更 `docs-site/**` | GitHub Pages |
| Deploy to Cloudflare Workers | **仅手动** | `wrangler deploy` |

Cloudflare 部署所需 GitHub Secrets：`CLOUDFLARE_API_TOKEN`、`CLOUDFLARE_ACCOUNT_ID`。Worker 密钥只放在 Cloudflare。

## 检查清单

- [ ] `mail.json` 的 `host` 与 `wrangler.jsonc` 路由一致
- [ ] KV ID 为真实值
- [ ] Worker 已设置 `ADMIN_PASSWORD` 与 `plugins.provider` 对应的 API 密钥
- [ ] 服务商已验证 `fromEmail`
- [ ] `main` 上 CI `check` 通过
