# 控制台

Custom Mail 跑起来之后（本地 Wrangler、Docker 或 Cloudflare），用浏览器这样操作。

按启动方式打开对应地址：

| 启动方式 | 地址 |
|----------|------|
| `npm run dev` | http://localhost:8790 |
| Docker / Compose / OpenClaw | http://localhost:8787 |
| 生产 | `config/mail.json` 里的 `https://<host>` |

登录密码是 `ADMIN_PASSWORD`（`.dev.vars`、Docker `-e` 或 `wrangler secret`）。尝试次数过多会短暂锁定。

## 撰写

1. **收件人（To）** — 可多个。回车或逗号确认地址。
2. **发件人名称** — 显示名；发件邮箱是 `config/mail.json` 的 `mail.fromEmail`。
3. **主题**
4. **正文** — 开启 `features.markdown` 时为 CommonMark + GitHub Flavored Markdown。

开启 `features.addressBook` 时，`mail.json` 的 `addressBook` 会显示成芯片，一点即可填入 **To**。

可选区块关掉即隐藏：在 `features` 里关闭附件、历史或通讯录后，对应 UI 不会出现。

## 预览与发送

用 **预览** 查看转成邮件安全 HTML 的结果，再确认发送。正文里的原始 HTML 会被忽略；`javascript:` 与 `data:` 不会变成链接。

**发送** 走当前服务商（`plugins.provider` 或 `MAIL_PROVIDER`）。必须配置对应 API 密钥，且 `fromEmail` 已在该服务商处授权。

## 附件

开启 `features.attachments` 时：

- 最多 **8** 个文件
- 单个 **8 MB**
- 合计 **15 MB**

可拖到附件区，或用文件选择器。

## 发送记录

开启 `features.history` 时，成功发送写入 KV（最近 10 封）。点开一行看详情。窄屏是「列表 → 全屏详情」；桌面是左右分栏。

失败发送若已部分落库，可能带错误状态出现。

## 健康检查

```bash
curl -s http://localhost:8787/api/health
```

JSON 含 `"ok": true`、`"runtime": "rust"`、当前 `"plugins"`（provider / theme / layout / logo）以及 `"available"` 目录。

## 下一步

- [Docker](./docker) — 不装 Rust 工具链
- [OpenClaw 技能](./openclaw) — 让 agent 拉起容器
- [插件](./plugins) · [配置说明](./config) · [部署](./deploy)
