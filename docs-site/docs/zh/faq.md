# 常见问题

## 密码正确但无法登录

- **生产：** 确认在 Worker `custom-mail` 上执行了 `wrangler secret put ADMIN_PASSWORD`。
- **本地：** 检查 `.dev.vars` 并重启 `npm run dev`。
- 尝试次数过多会临时锁定，稍后再试。

## 发送失败 / 服务商报错

- 确认 Worker 上有与 `plugins.provider`（或 `MAIL_PROVIDER`）对应的 API 密钥。
- `mail.json` 中的 `fromEmail` 须在该服务商处授权。
- 在服务商控制台查看限额或域名验证状态。

## CI 部署失败

- `check` 任务须通过（typecheck）。
- Deploy workflow 需要 `CLOUDFLARE_API_TOKEN` 与 `CLOUDFLARE_ACCOUNT_ID`。
- 本地使用 Node 22+ 与 CI 对齐。

## 发送记录为空

仅成功发送会写入 KV。失败发送可能以错误状态部分记录。

## 移动端布局

窄屏上发送记录为「列表 → 全屏详情」；桌面为左右分栏。

## 能否换成非 Brevo 的邮件服务？

可以。在 `config/mail.json` 里设置 `plugins.provider`（或环境变量 `MAIL_PROVIDER`）为 `plugins/providers/` 下的 id（内置：`brevo`、`resend`、`sendgrid`、`mailgun`、`postmark`、`mailersend`、`smtp2go`、`sparkpost`），并在 Worker 上配置对应密钥（见[配置说明](./config)）。`fromEmail` 须在该服务商处授权。

## 如何增加主题、版式或 Logo？

在 `plugins/themes/` 或 `plugins/layouts/` 放入 JSON，或在 `plugins/logos/` 放入图片，再设置 `plugins.theme` / `layout` / `logo`（也可用 `MAIL_THEME`、`MAIL_LAYOUT`、`MAIL_LOGO`）。详见[配置说明](./config)。

## 支持哪些 Markdown？

正文由 Rust（`src/markdown.rs`）按 CommonMark + GitHub Flavored Markdown 转成邮件 HTML：

- 标题、段落、斜体、粗体、删除线
- 有序 / 无序 / 任务列表（含嵌套）
- 链接（`http`/`https`/`mailto`）与图片（`http`/`https`）
- 围栏/缩进代码块、表格、引用、分隔线
- GFM 提示块（`> [!NOTE]`、`> [!WARNING]` 等）

源文中的原始 HTML 会被忽略。`javascript:` 与 `data:` 不会变成链接。

## 怎样用 Docker 跑？

见 [Docker](./docker)：`docker pull xuxuclassmate/custom-mail:1.0.0`，设置 `ADMIN_PASSWORD`，再 `docker run -p 8787:8787`。Compose 与环境变量也在那一页。

## 怎样用 OpenClaw / Claw 技能？

```bash
clawhub install custom-mail-skill
```

或把 `skills/custom-mail/` 拷到 `~/.openclaw/workspace/skills/`。技能需要 Docker。详见 [OpenClaw 技能](./openclaw)。

## 安全提示

- 不要在公网暴露弱密码控制台。
- 会话 Cookie 为 HttpOnly、Secure、SameSite=Strict。
- 不要提交 `.dev.vars` 或 API Token。

## 获取帮助

- [GitHub Discussions](https://github.com/InnoNestX/Custom-Mail/discussions)
- [InnoNestX Discussions](https://github.com/InnoNestX/.github/discussions)
- 安全问题：[私密 Advisory](https://github.com/InnoNestX/Custom-Mail/security/advisories/new)
