# OpenClaw 技能

仓库里带一份 [OpenClaw](https://docs.openclaw.ai/tools/skills) 技能，让 agent 拉取 Docker 镜像、写入密钥并启动[控制台](./console)。文件在 `skills/custom-mail/SKILL.md`（ClawHub 名称：`custom-mail`）。

Agent 所在机器需要有 **`docker`**。技能不会部署到 Cloudflare，生产仍看[部署](./deploy)。

## 安装

### ClawHub

```bash
clawhub install custom-mail
```

技能目录：[clawhub.ai](https://clawhub.ai)。CLI 说明：[ClawHub](https://docs.openclaw.ai/tools/clawhub)。

### 从本仓库复制

拷到 OpenClaw 工作区技能目录（优先级最高）或共享技能目录：

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cp -R Custom-Mail/skills/custom-mail ~/.openclaw/workspace/skills/custom-mail
```

该实例上所有 agent 都能用：

```bash
cp -R Custom-Mail/skills/custom-mail ~/.openclaw/skills/custom-mail
```

OpenClaw 会从这些目录加载 `SKILL.md`。详见 [Skills](https://docs.openclaw.ai/tools/skills)。

## 技能会做什么

当你说要在本地跑 Custom Mail 时，agent 应：

1. 确认 Docker 可用。
2. 设置必填的 `ADMIN_PASSWORD`，以及你提供的服务商 API 密钥。
3. 按需设置 `MAIL_PROVIDER`、`MAIL_THEME`、`MAIL_LAYOUT`、`MAIL_LOGO`。
4. `docker pull` `xuxuclassmate/custom-mail:1.0.0`（或 `:latest` / GHCR `ghcr.io/innonestx/custom-mail:1.0.0`）。
5. 在 **8787**（或 `PORT`）上 `docker run`。
6. 告诉你打开 http://localhost:8787 并登录。

健康检查：`curl -s http://localhost:8787/api/health`。

完整 Docker 参数与 Compose 见 [Docker](./docker)。

## 技能识别的环境变量

| 变量 | 必填 | 说明 |
| --- | --- | --- |
| `ADMIN_PASSWORD` | 是 | 控制台登录 |
| `MAIL_PROVIDER` | 否 | `brevo`、`resend`、`sendgrid`、`mailgun`、`postmark`、`mailersend`、`smtp2go`、`sparkpost` |
| `MAIL_THEME` | 否 | `forest`、`midnight`、`ocean`、`paper`、`rose`、`slate`、`aurora`、`sunset`、`nord` 等 |
| `MAIL_LAYOUT` | 否 | `card`、`minimal`、`banner`、`digest`、`compact` |
| `MAIL_LOGO` | 否 | `auto`、`image`、`monogram`、`none` |
| `BREVO_API_KEY` | 否 | 默认服务商；要发信才需要（除非配了别的密钥） |
| `MAIL_API_KEY` | 否 | 回退密钥 |
| `RESEND_API_KEY` | 否 | 配合 `MAIL_PROVIDER=resend` |
| `SENDGRID_API_KEY` | 否 | 配合 `MAIL_PROVIDER=sendgrid` |
| `PORT` | 否 | 容器端口，默认 `8787` |

同样的键写在技能元数据 `metadata.openclaw.envVars` 里。若密钥放在 OpenClaw 配置而不是对话里，在 `~/.openclaw/openclaw.json` 中映射即可。

## 示例提示词

```
Pull and run Custom Mail on port 8787 with ADMIN_PASSWORD=dev-secret and my Brevo key.
```

```
Start the custom-mail Docker container with MAIL_PROVIDER=resend, MAIL_THEME=nord, and RESEND_API_KEY.
```

```
Clone InnoNestX/Custom-Mail and bring it up with docker compose.
```

技能里的中文触发包括：「帮我本地跑一下 Custom Mail」「用 Docker 启动发信控制台」。

## 启动之后

用[控制台](./console)撰写 Markdown、预览、附件、发送、查看历史。

要改镜像里编译进去的 `mail.json` / `plugins/`，请克隆仓库并按 [Compose 重建](./docker#docker-compose)。槽位环境变量仍可在不重建时覆盖当前插件。

## 生产部署

技能可以提醒 Cloudflare 路径，但生产密钥只放在 Worker 上：

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
npm run deploy
```

见[部署](./deploy)。
