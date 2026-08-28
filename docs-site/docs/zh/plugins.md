# 插件

五个插槽都是 `plugins/` 下的可插拔目录。Cloudflare Worker 不能在运行时加载 `.so`，所以插件是 **编译进 Worker 的 JSON 或图片**。在 `config/mail.json` 里选出当前 id（也可用环境变量）。未配置的区块不会渲染。

| 插槽 | 放入文件 | 在配置中选择 | 运行时覆盖 |
|------|----------|--------------|------------|
| 服务商 | `plugins/providers/*.json` | `plugins.provider` | `MAIL_PROVIDER` |
| 功能 | `plugins/features/*.json` | `features.*`（`false` 即隐藏） | — |
| 主题 | `plugins/themes/*.json` | `plugins.theme` | `MAIL_THEME` |
| 版式 | `plugins/layouts/*.json` | `plugins.layout` | `MAIL_LAYOUT` |
| Logo | `plugins/logos/*` | `plugins.logo` | `MAIL_LOGO` |
| 配置 | `config/overlays/*.json` | 省略不需要的键 | `MAIL_CONFIG_JSON` |

`GET /api/health` 的 `available` 列出已编译目录，`plugins` 列出当前启用的 id。

## 服务商

内置：`brevo` · `resend` · `sendgrid` · `mailgun` · `postmark` · `mailersend` · `smtp2go` · `sparkpost`。

JSON 登记 id、名称、密钥名、是否需要发信域名。**新增 HTTP 发信 API 仍要在 Rust 里写发送适配**（`src/plugins/provider.rs`）。

## 功能

把 JSON 放到 `plugins/features/`。每个文件对应 `features.*` 里的开关，设为 `false` 即隐藏。内置：`markdown` · `history` · `attachments`。

## 主题

放入调色板 JSON 即可，**不用改 Rust**。内置：`forest` · `midnight` · `ocean` · `paper` · `rose` · `slate` · `aurora` · `sunset` · `nord`。

未知 id 回退 `forest`。`brand.*` 可覆盖单个颜色。

`plugins/themes/nord.json` 示例：

```json
{
  "id": "nord",
  "label": "Nord",
  "aliases": ["polar"],
  "accent": "#5e81ac",
  "accentDeep": "#2e3440",
  "accentSoft": "#eceff4",
  "ink": "#2e3440",
  "muted": "#4c566a",
  "paper": "#eceff4",
  "line": "#d8dee9",
  "heroFrom": "#5e81ac",
  "heroTo": "#3b4252",
  "headerText": "#eceff4"
}
```

## 版式

JSON 控制页眉样式、内边距、卡片阴影，**这些旋钮不用改 Rust**。内置：`card` · `minimal` · `banner` · `digest` · `compact`。

```json
{
  "id": "compact",
  "label": "Compact",
  "headerStyle": "plain",
  "bodyPadding": "12px 16px 12px",
  "cardShadow": "none"
}
```

`headerStyle` 为 `plain`、`gradient` 或 `none`。

## Logo

| `plugins.logo` | 效果 |
|----------------|------|
| `auto` | 有图用图；否则品牌名首字母；再否则省略 |
| `image` | 配置的图或 `plugins/logos/` 里第一个文件；缺失则用首字母 |
| `monogram` | 只用字母标 |
| `none` | 不显示 |

`plugins/logos/` 中的文件构建后位于 `/plugins/logos/<文件名>`。把 `site.logoPath` 指过去，或留空以使用该目录第一个文件。

## 配置覆盖

`config/overlays/` 里的 JSON 在**编译时**深度合并进 `mail.json`（覆盖层优先；`null` 删除键）。**运行时**不重建可用：

```bash
MAIL_CONFIG_JSON='{"plugins":{"theme":"nord","logo":"monogram"}}'
```

`MAIL_THEME` 等槽位变量在该覆盖之后生效。

## 添加文件之后

```bash
cargo test --lib
npm run deploy
# 或：docker compose build
```

新增 JSON / Logo 文件需要重建。只用 `MAIL_*` 切换当前 id 不需要。
