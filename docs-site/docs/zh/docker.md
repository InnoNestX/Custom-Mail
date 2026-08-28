# Docker

用容器在本地跑同一套 Rust Worker，主机上不需要 Rust 或 `wasm32-unknown-unknown`。发布镜像用来试用[控制台](./console)；生产仍走 [Cloudflare](./deploy)。

镜像：

- Docker Hub / GHCR：`xuxuclassmate/custom-mail:1.0.0` 与 `ghcr.io/innonestx/custom-mail:1.0.0`
- 滚动标签：`:latest`（同一次发版）

镜像内已设置 `ALLOW_ANY_HOST=1`，`localhost` 不会被 Host 检查拦住。

## 快速开始

### 1. 拉取

```bash
docker pull xuxuclassmate/custom-mail:1.0.0
```

GHCR：

```bash
docker pull ghcr.io/innonestx/custom-mail:1.0.0
```

### 2. 密钥

必须有 `ADMIN_PASSWORD`。要发信时再配当前服务商的 API 密钥（镜像里 `config/mail.json` 的 `plugins.provider`，默认 `brevo`）。没有密钥也能打开 UI。

```bash
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'   # 或其他服务商密钥
export PORT=8787                      # 可选，默认 8787
```

**不重建镜像** 切换服务商 / 主题 / 版式 / Logo：

```bash
export MAIL_PROVIDER=resend
export MAIL_THEME=nord
export MAIL_LAYOUT=compact
export MAIL_LOGO=monogram
export RESEND_API_KEY='re_...'
```

`MAIL_CONFIG_JSON='{"site":{"brandName":"Desk"}}'` 会在运行时深度合并。**新增**主题 JSON 或 Logo 文件仍需 `docker compose build`。

### 3. 运行

前台：

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:1.0.0
```

后台：

```bash
docker run -d \
  --name custom-mail \
  -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:1.0.0
```

打开 [http://localhost:8787](http://localhost:8787)，用 `ADMIN_PASSWORD` 登录。撰写、预览、附件、历史见[控制台](./console)。

```bash
curl -s http://localhost:8787/api/health
```

停止：`docker stop custom-mail`；删除：`docker rm custom-mail`。

## Docker Compose

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'
docker compose up -d
```

Compose 映射 **8787**，传入同样的环境变量，Wrangler 状态放在 `mail-data` 卷。

```bash
docker compose down
```

改品牌或增加插件 JSON / Logo 时，编辑 `config/mail.json` / `plugins/` 后：

```bash
docker compose build
docker compose up -d
```

槽位环境变量（`MAIL_PROVIDER`、`MAIL_THEME` 等）仍可在不重建的情况下切换**当前**插件。

## 环境变量

| 变量 | 默认 | 说明 |
| --- | --- | --- |
| `ADMIN_PASSWORD` | *（必填）* | 控制台登录密码 |
| `MAIL_PROVIDER` | 空 | 覆盖 `plugins.provider` |
| `MAIL_THEME` | 空 | 覆盖 `plugins.theme` |
| `MAIL_LAYOUT` | 空 | 覆盖 `plugins.layout` |
| `MAIL_LOGO` | 空 | 覆盖 `plugins.logo`（`auto` / `image` / `monogram` / `none`） |
| `MAIL_CONFIG_JSON` | 空 | 运行时深度合并进 `mail.json` |
| `BREVO_API_KEY` | 空 | 默认服务商；无密钥可打开 UI，发信需要密钥 |
| `RESEND_API_KEY` | 空 | Resend |
| `SENDGRID_API_KEY` | 空 | SendGrid |
| `MAILGUN_API_KEY` / `MAILGUN_DOMAIN` | 空 | Mailgun |
| `POSTMARK_SERVER_TOKEN` | 空 | Postmark |
| `MAILERSEND_API_KEY` | 空 | MailerSend |
| `SMTP2GO_API_KEY` | 空 | SMTP2GO |
| `SPARKPOST_API_KEY` | 空 | SparkPost |
| `MAIL_API_KEY` | 空 | 服务商专用密钥为空时的回退 |
| `PORT` | `8787` | 容器内监听端口 |

服务商 id：`brevo`、`resend`、`sendgrid`、`mailgun`、`postmark`、`mailersend`、`smtp2go`、`sparkpost`。  
主题 id：`forest`、`midnight`、`ocean`、`paper`、`rose`、`slate`、`aurora`、`sunset`、`nord` 等。  
版式 id：`card`、`minimal`、`banner`、`digest`、`compact`。

## 入口脚本

`docker/entrypoint.sh` 根据环境写出 `.dev.vars`（含 `ALLOW_ANY_HOST=1` 以及上面非空的键），然后执行 `wrangler dev --ip 0.0.0.0 --port $PORT --local`。

## 排障

- **容器立刻退出** — 没设 `ADMIN_PASSWORD`，入口脚本会拒绝启动。
- **发不出去** — 配上与 `MAIL_PROVIDER` / `plugins.provider` 对应的密钥，并授权 `fromEmail`。
- **新主题文件不出现** — 目录在**构建镜像**时编译。往 `plugins/` 加文件后需要重建。
- **端口占用** — 改主机映射，例如 `-p 8788:8787`。

## 下一步

- [控制台](./console)
- [OpenClaw 技能](./openclaw) — `clawhub install custom-mail-skill`
- [插件](./plugins) · [部署](./deploy)
