# Custom Mail

基于 **Cloudflare Workers** 的私有网页发信控制台。支持 Markdown 撰写、附件、HTML 预览与发送记录，出站邮件经 **Brevo** 投递。

| | |
|---|---|
| **在线演示** | [mail.xuxuclassmate.com](https://mail.xuxuclassmate.com) |
| **代码仓库** | [InnoNestX/Custom-Mail](https://github.com/InnoNestX/Custom-Mail) |
| **许可证** | MIT |

## 为什么用 Custom Mail

- **无需 VPS** — Worker + KV + 静态资源跑在边缘
- **单密码登录** — 会话 Cookie，私有发信界面
- **可品牌化** — 标题、配色、文案、通讯录写在 `config/mail.json`
- **Markdown 正文** — 发送前预览；支持附件
- **可追溯** — 近期发送记录存 KV（桌面 / 移动详情）

## 架构

```text
浏览器 ──► Cloudflare Worker (custom-mail)
              ├── Workers Assets（界面）
              ├── KV MAIL_LOG_KV（会话 + 发送日志）
              └── Brevo API（事务邮件）
```

## 下一步

1. [快速开始](./quick-start) — 本地运行
2. [配置说明](./config) — `config/mail.json`
3. [部署](./deploy) — Cloudflare Worker
4. [常见问题](./faq)
