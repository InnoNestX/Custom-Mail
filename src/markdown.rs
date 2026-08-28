//! CommonMark + GitHub Flavored Markdown → email-safe HTML.
//!
//! Raw HTML in the source is dropped. Only `http`, `https`, `mailto`, and
//! in-page `#` fragment URLs are emitted as links or images.

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
};

const SNIPPET_MAX_CHARS: usize = 1400;

const INLINE_CODE: &str = "display:inline-block;font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:13px;background:#f5f4f1;border:1px solid #e7e5e4;border-radius:6px;padding:2px 7px;color:#1c1917;user-select:all;-webkit-user-select:all;";
const COPY_BTN: &str = "font-size:10px;font-weight:700;color:#57534e;background:#fff;border:1px solid #e7e5e4;border-radius:5px;padding:3px 10px;text-decoration:none;line-height:1.4;font-family:Arial,Helvetica,sans-serif;";
const LINK: &str = "color:#0f766e;text-decoration:underline;";
const P: &str = "margin:8px 0;font-size:15px;line-height:1.65;";
const PRE: &str = "margin:0;font-family:Consolas,Courier,monospace;font-size:12px;line-height:1.55;color:#1c1917;white-space:pre-wrap;word-wrap:break-word;word-break:break-word;-webkit-user-select:all;user-select:all;";

#[derive(Clone, Debug, Default)]
pub struct MarkdownOptions {
    /// Preview UI: in-page copy buttons instead of snippet links.
    pub interactive: bool,
    /// Origin used to build `/snippet?e=` links in sent mail (no trailing slash).
    pub snippet_origin: Option<String>,
}

impl MarkdownOptions {
    pub fn preview(origin: impl Into<String>) -> Self {
        Self {
            interactive: true,
            snippet_origin: Some(origin.into()),
        }
    }

    pub fn email(origin: impl Into<String>) -> Self {
        Self {
            interactive: false,
            snippet_origin: Some(origin.into()),
        }
    }
}

pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

pub fn render_markdown(src: &str, opts: &MarkdownOptions) -> String {
    let normalized = src.replace("\r\n", "\n");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);

    let parser = Parser::new_ext(&normalized, options);
    let mut w = HtmlWriter::new(opts);
    w.feed(parser);
    w.finish()
}

struct TableState {
    alignments: Vec<Alignment>,
    col: usize,
    in_head: bool,
}

enum ListKind {
    Unordered,
    Ordered,
}

struct HtmlWriter<'a> {
    out: String,
    opts: &'a MarkdownOptions,
    lists: Vec<ListKind>,
    tables: Vec<TableState>,
    skip_link_ends: u32,
    code: Option<(String, String)>,
    image: Option<ImageBuf>,
}

struct ImageBuf {
    safe: bool,
    url: String,
    title: String,
    alt: String,
}

impl<'a> HtmlWriter<'a> {
    fn new(opts: &'a MarkdownOptions) -> Self {
        Self {
            out: String::new(),
            opts,
            lists: Vec::new(),
            tables: Vec::new(),
            skip_link_ends: 0,
            code: None,
            image: None,
        }
    }

    fn feed<'p>(&mut self, parser: Parser<'p>) {
        for event in parser {
            self.event(event);
        }
    }

    fn finish(self) -> String {
        if self.out.is_empty() {
            return String::new();
        }
        self.out
    }

    fn event(&mut self, event: Event<'_>) {
        if let Some((_, body)) = self.code.as_mut() {
            match event {
                Event::Text(t) | Event::Code(t) => body.push_str(&t),
                Event::SoftBreak | Event::HardBreak => body.push('\n'),
                Event::End(TagEnd::CodeBlock) => {
                    if let Some((lang, code)) = self.code.take() {
                        self.out.push_str(&code_block_html(
                            &code,
                            if lang.is_empty() {
                                None
                            } else {
                                Some(lang.as_str())
                            },
                            self.opts,
                        ));
                    }
                }
                _ => {}
            }
            return;
        }

        if let Some(img) = self.image.as_mut() {
            match event {
                Event::Text(t) | Event::Code(t) => img.alt.push_str(&t),
                Event::SoftBreak | Event::HardBreak => img.alt.push(' '),
                Event::End(TagEnd::Image) => {
                    if let Some(img) = self.image.take() {
                        self.emit_image(img);
                    }
                }
                _ => {}
            }
            return;
        }

        match event {
            Event::Start(tag) => self.start(tag),
            Event::End(tag) => self.end(tag),
            Event::Text(text) => self.out.push_str(&escape_html(&text)),
            Event::Code(code) => {
                self.out.push_str(&format!(
                    "<code style=\"{INLINE_CODE}\">{}</code>",
                    escape_html(&code)
                ));
            }
            Event::Html(_)
            | Event::InlineHtml(_)
            | Event::DisplayMath(_)
            | Event::InlineMath(_) => {}
            Event::FootnoteReference(label) => {
                let l = escape_html(&label);
                self.out.push_str(&format!(
                    "<sup style=\"font-size:11px;\"><a href=\"#fn-{l}\" style=\"{LINK}\">[{l}]</a></sup>"
                ));
            }
            Event::SoftBreak | Event::HardBreak => self.out.push_str("<br>\n"),
            Event::Rule => self
                .out
                .push_str("<hr style=\"border:0;border-top:1px solid #e7e5e4;margin:16px 0;\">"),
            Event::TaskListMarker(checked) => {
                let mark = if checked { "☑" } else { "☐" };
                self.out.push_str(mark);
                self.out.push('\u{00a0}');
            }
        }
    }

    fn start(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph => {
                self.out.push_str(&format!("<p style=\"{P}\">"));
            }
            Tag::Heading { level, .. } => {
                let (n, size) = heading_style(level);
                self.out.push_str(&format!(
                    "<h{n} style=\"margin:14px 0 6px;font-size:{size}px;line-height:1.35;font-weight:700;color:#1a1c19;\">"
                ));
            }
            Tag::BlockQuote(kind) => {
                let (label, border) = quote_style(kind);
                self.out.push_str(&format!(
                    "<blockquote style=\"margin:12px 0;padding:8px 12px;border-left:3px solid {border};background:#f7f8f6;color:#3f463d;\">"
                ));
                if let Some(label) = label {
                    self.out.push_str(&format!(
                        "<div style=\"font-size:11px;font-weight:800;letter-spacing:.04em;text-transform:uppercase;color:#6f776c;margin-bottom:4px;\">{label}</div>"
                    ));
                }
            }
            Tag::CodeBlock(kind) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(info) => {
                        info.split_whitespace().next().unwrap_or("").to_string()
                    }
                    CodeBlockKind::Indented => String::new(),
                };
                self.code = Some((lang, String::new()));
            }
            Tag::HtmlBlock | Tag::MetadataBlock(_) => {}
            Tag::List(start) => match start {
                Some(n) => {
                    self.lists.push(ListKind::Ordered);
                    if n == 1 {
                        self.out
                            .push_str("<ol style=\"margin:8px 0;padding-left:22px;\">");
                    } else {
                        self.out.push_str(&format!(
                            "<ol start=\"{n}\" style=\"margin:8px 0;padding-left:22px;\">"
                        ));
                    }
                }
                None => {
                    self.lists.push(ListKind::Unordered);
                    self.out
                        .push_str("<ul style=\"margin:8px 0;padding-left:22px;\">");
                }
            },
            Tag::Item => {
                self.out
                    .push_str("<li style=\"margin:2px 0;line-height:1.55;\">");
            }
            Tag::FootnoteDefinition(label) => {
                let l = escape_html(&label);
                self.out.push_str(&format!(
                    "<div id=\"fn-{l}\" style=\"margin:10px 0;padding:8px 10px;background:#f7f8f6;border-radius:8px;font-size:13px;color:#3f463d;\"><strong>[{l}]</strong> "
                ));
            }
            Tag::Table(alignments) => {
                self.tables.push(TableState {
                    alignments,
                    col: 0,
                    in_head: false,
                });
                self.out.push_str(
                    "<table role=\"presentation\" width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" style=\"margin:12px 0;border-collapse:collapse;border:1px solid #e7e5e4;\">",
                );
            }
            Tag::TableHead => {
                if let Some(t) = self.tables.last_mut() {
                    t.in_head = true;
                    t.col = 0;
                }
                self.out.push_str("<thead>");
            }
            Tag::TableRow => {
                if let Some(t) = self.tables.last_mut() {
                    t.col = 0;
                }
                self.out.push_str("<tr>");
            }
            Tag::TableCell => {
                let (tag_name, align) = if let Some(t) = self.tables.last() {
                    let align = t.alignments.get(t.col).copied().unwrap_or(Alignment::None);
                    let name = if t.in_head { "th" } else { "td" };
                    (name, align)
                } else {
                    ("td", Alignment::None)
                };
                if let Some(t) = self.tables.last_mut() {
                    t.col += 1;
                }
                let align_css = alignment_css(align);
                let weight = if tag_name == "th" {
                    "font-weight:700;background:#eef3ea;"
                } else {
                    "font-weight:400;background:#fff;"
                };
                self.out.push_str(&format!(
                    "<{tag_name} style=\"border:1px solid #e7e5e4;padding:8px 10px;font-size:13px;{weight}{align_css}\">"
                ));
            }
            Tag::Emphasis => self.out.push_str("<em>"),
            Tag::Strong => self.out.push_str("<strong>"),
            Tag::Strikethrough => self.out.push_str("<del>"),
            Tag::Link {
                dest_url, title, ..
            } => {
                if let Some(href) = safe_url(&dest_url) {
                    let title_attr = if title.is_empty() {
                        String::new()
                    } else {
                        format!(" title=\"{}\"", escape_html(&title))
                    };
                    self.out.push_str(&format!(
                        "<a href=\"{}\"{title_attr} style=\"{LINK}\">",
                        escape_html(href)
                    ));
                } else {
                    self.skip_link_ends += 1;
                }
            }
            Tag::Image {
                dest_url, title, ..
            } => {
                let safe = safe_url(&dest_url).map(str::to_string);
                self.image = Some(ImageBuf {
                    safe: safe.is_some(),
                    url: safe.unwrap_or_default(),
                    title: title.to_string(),
                    alt: String::new(),
                });
            }
            Tag::DefinitionList => {
                self.out.push_str("<dl style=\"margin:8px 0;\">");
            }
            Tag::DefinitionListTitle => {
                self.out
                    .push_str("<dt style=\"font-weight:700;margin-top:8px;\">");
            }
            Tag::DefinitionListDefinition => {
                self.out
                    .push_str("<dd style=\"margin:2px 0 8px 16px;color:#3f463d;\">");
            }
            Tag::Superscript => self.out.push_str("<sup>"),
            Tag::Subscript => self.out.push_str("<sub>"),
        }
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.out.push_str("</p>"),
            TagEnd::Heading(level) => {
                let n = heading_style(level).0;
                self.out.push_str(&format!("</h{n}>"));
            }
            TagEnd::BlockQuote(_) => self.out.push_str("</blockquote>"),
            TagEnd::CodeBlock => {}
            TagEnd::HtmlBlock | TagEnd::MetadataBlock(_) => {}
            TagEnd::List(_) => match self.lists.pop() {
                Some(ListKind::Ordered) => self.out.push_str("</ol>"),
                Some(ListKind::Unordered) | None => self.out.push_str("</ul>"),
            },
            TagEnd::Item => self.out.push_str("</li>"),
            TagEnd::FootnoteDefinition => self.out.push_str("</div>"),
            TagEnd::Table => {
                self.tables.pop();
                self.out.push_str("</table>");
            }
            TagEnd::TableHead => {
                if let Some(t) = self.tables.last_mut() {
                    t.in_head = false;
                }
                self.out.push_str("</thead>");
            }
            TagEnd::TableRow => self.out.push_str("</tr>"),
            TagEnd::TableCell => {
                let closing = if self.tables.last().map(|t| t.in_head).unwrap_or(false) {
                    "</th>"
                } else {
                    "</td>"
                };
                self.out.push_str(closing);
            }
            TagEnd::Emphasis => self.out.push_str("</em>"),
            TagEnd::Strong => self.out.push_str("</strong>"),
            TagEnd::Strikethrough => self.out.push_str("</del>"),
            TagEnd::Link => {
                if self.skip_link_ends > 0 {
                    self.skip_link_ends -= 1;
                } else {
                    self.out.push_str("</a>");
                }
            }
            TagEnd::Image => {}
            TagEnd::DefinitionList => self.out.push_str("</dl>"),
            TagEnd::DefinitionListTitle => self.out.push_str("</dt>"),
            TagEnd::DefinitionListDefinition => self.out.push_str("</dd>"),
            TagEnd::Superscript => self.out.push_str("</sup>"),
            TagEnd::Subscript => self.out.push_str("</sub>"),
        }
    }

    fn emit_image(&mut self, img: ImageBuf) {
        if img.safe {
            let title_attr = if img.title.is_empty() {
                String::new()
            } else {
                format!(" title=\"{}\"", escape_html(&img.title))
            };
            self.out.push_str(&format!(
                "<img src=\"{}\" alt=\"{}\"{title_attr} style=\"max-width:100%;height:auto;border:0;border-radius:8px;display:block;margin:10px 0;\">",
                escape_html(&img.url),
                escape_html(&img.alt),
            ));
        } else if !img.alt.is_empty() {
            self.out.push_str(&escape_html(&img.alt));
        }
    }
}

fn heading_style(level: HeadingLevel) -> (u8, u8) {
    match level {
        HeadingLevel::H1 => (1, 20),
        HeadingLevel::H2 => (2, 17),
        HeadingLevel::H3 => (3, 15),
        HeadingLevel::H4 => (4, 14),
        HeadingLevel::H5 => (5, 13),
        HeadingLevel::H6 => (6, 13),
    }
}

fn quote_style(kind: Option<BlockQuoteKind>) -> (Option<&'static str>, &'static str) {
    match kind {
        Some(BlockQuoteKind::Note) => (Some("Note"), "#2f9e7b"),
        Some(BlockQuoteKind::Tip) => (Some("Tip"), "#1f6f5b"),
        Some(BlockQuoteKind::Important) => (Some("Important"), "#5b5bd6"),
        Some(BlockQuoteKind::Warning) => (Some("Warning"), "#b45309"),
        Some(BlockQuoteKind::Caution) => (Some("Caution"), "#b42318"),
        None => (None, "#c5cdc0"),
    }
}

fn alignment_css(align: Alignment) -> &'static str {
    match align {
        Alignment::Left => "text-align:left;",
        Alignment::Center => "text-align:center;",
        Alignment::Right => "text-align:right;",
        Alignment::None => "text-align:left;",
    }
}

fn safe_url(raw: &str) -> Option<&str> {
    let url = raw.trim();
    if url.is_empty() {
        return None;
    }
    let lower: String = url
        .chars()
        .take(16)
        .flat_map(|c| c.to_lowercase())
        .collect();
    if lower.starts_with("javascript:")
        || lower.starts_with("vbscript:")
        || lower.starts_with("data:")
        || lower.starts_with("file:")
    {
        return None;
    }
    if url.starts_with('#')
        || lower.starts_with("https://")
        || lower.starts_with("http://")
        || lower.starts_with("mailto:")
    {
        if url
            .chars()
            .any(|c| c.is_ascii_control() || c == '"' || c == '<' || c == '>')
        {
            return None;
        }
        Some(url)
    } else {
        None
    }
}

fn copy_action_html(code: &str, opts: &MarkdownOptions) -> String {
    if opts.interactive {
        return format!(
            "<a href=\"#\" class=\"xxm-copy-btn\" data-copy=\"{}\" style=\"{COPY_BTN}\">Copy</a>",
            escape_html(code)
        );
    }
    if let Some(origin) = opts.snippet_origin.as_deref() {
        if code.len() <= SNIPPET_MAX_CHARS {
            let href = format!("{origin}/snippet?e={}", encode_snippet_param(code));
            return format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" style=\"{COPY_BTN}\">Copy</a>",
                escape_html(&href)
            );
        }
    }
    "<span style=\"font-size:10px;font-weight:600;color:#9aa89f;font-family:Arial,Helvetica,sans-serif;\">Select to copy</span>"
        .into()
}

fn code_block_html(code: &str, lang: Option<&str>, opts: &MarkdownOptions) -> String {
    let code = code.trim_end_matches('\n');
    let escaped = escape_html(code);
    let lang_label = lang
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(escape_html);
    let lang_cell = lang_label
        .as_deref()
        .map(|l| {
            format!(
                "<td align=\"right\" valign=\"middle\" style=\"font-size:10px;font-weight:700;color:#78716c;text-transform:lowercase;font-family:Arial,Helvetica,sans-serif;white-space:nowrap;\">{l}</td>"
            )
        })
        .unwrap_or_default();
    let copy = copy_action_html(code, opts);
    let header_inner = format!(
        "<table role=\"presentation\" align=\"right\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" style=\"border-collapse:collapse;margin:0;\"><tr><td align=\"right\" valign=\"middle\" style=\"padding:0 8px 0 0;font-family:Arial,Helvetica,sans-serif;\">{copy}</td>{lang_cell}</tr></table>"
    );
    format!(
        "<table role=\"presentation\" class=\"xxm-code-block\" width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" style=\"margin:12px 0;border:1px solid #e7e5e4;border-radius:8px;background:#f5f4f1;border-collapse:separate;\">\
         <tr><td style=\"padding:8px 12px;border-bottom:1px solid #e7e5e4;background:#eef3ea;\">\
         <table role=\"presentation\" width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\"><tr><td align=\"right\">{header_inner}</td></tr></table>\
         </td></tr>\
         <tr><td style=\"padding:12px 14px;{PRE}\">{escaped}</td></tr>\
         </table>"
    )
}

/// URL-safe Base64 without padding, matching the Worker `/snippet` decoder.
pub fn encode_snippet_param(text: &str) -> String {
    let mut encoded = base64_encode(text.as_bytes());
    encoded = encoded.replace('+', "-").replace('/', "_");
    while encoded.ends_with('=') {
        encoded.pop();
    }
    encoded
}

pub fn decode_snippet_param(encoded: &str) -> Result<String, String> {
    let s = encoded.replace('-', "+").replace('_', "/");
    let pad = match s.len() % 4 {
        0 => "",
        2 => "==",
        3 => "=",
        _ => return Err("Invalid snippet".into()),
    };
    let s = format!("{s}{pad}");
    let bytes = base64_decode(&s)?;
    String::from_utf8(bytes).map_err(|_| "Invalid snippet".into())
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let remaining = data.len() - i;
        let b0 = data[i];
        let b1 = if remaining > 1 { data[i + 1] } else { 0 };
        let b2 = if remaining > 2 { data[i + 2] } else { 0 };
        out.push(T[(b0 >> 2) as usize] as char);
        out.push(T[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if remaining == 1 {
            out.push('=');
            out.push('=');
        } else if remaining == 2 {
            out.push(T[((b1 & 0x0f) << 2) as usize] as char);
            out.push('=');
        } else {
            out.push(T[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
            out.push(T[(b2 & 0x3f) as usize] as char);
        }
        i += 3;
    }
    out
}

fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0u32;
    for c in data.chars() {
        if c == '=' {
            break;
        }
        let v = T
            .iter()
            .position(|&x| x == c as u8)
            .ok_or_else(|| "Invalid snippet".to_string())? as u32;
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn html(md: &str) -> String {
        render_markdown(md, &MarkdownOptions::default())
    }

    #[test]
    fn headings_bold_italic_and_links() {
        let out = html("# Hello\n\n**bold** and *italic* and [docs](https://example.com)");
        assert!(out.contains("<h1"), "{out}");
        assert!(out.contains("<strong>bold</strong>"), "{out}");
        assert!(out.contains("<em>italic</em>"), "{out}");
        assert!(
            out.contains("href=\"https://example.com\"") && out.contains(">docs</a>"),
            "{out}"
        );
    }

    #[test]
    fn lists_nested_and_ordered() {
        let md = "1. one\n2. two\n   - nested\n3. three\n";
        let out = html(md);
        assert!(out.contains("<ol"), "{out}");
        assert!(out.contains("<ul"), "{out}");
        assert!(out.contains("nested"), "{out}");
    }

    #[test]
    fn gfm_table_strikethrough_task() {
        let md = "\
| A | B |\n\
| --- | ---: |\n\
| 1 | 2 |\n\
\n\
~~old~~\n\
\n\
- [x] done\n\
- [ ] todo\n";
        let out = html(md);
        assert!(out.contains("<table"), "{out}");
        assert!(out.contains("<th"), "{out}");
        assert!(out.contains("text-align:right;"), "{out}");
        assert!(out.contains("<del>old</del>"), "{out}");
        assert!(out.contains('☑'), "{out}");
        assert!(out.contains('☐'), "{out}");
    }

    #[test]
    fn fenced_code_and_inline_code() {
        let md = "Use `cargo test`.\n\n```rust\nfn main() {}\n```\n";
        let out = html(md);
        assert!(out.contains("<code"), "{out}");
        assert!(out.contains("fn main() {}"), "{out}");
        assert!(out.contains("rust"), "{out}");
        assert!(out.contains("xxm-code-block"), "{out}");
    }

    #[test]
    fn blockquote_hr_image_and_paragraphs() {
        let md =
            "> a quote\n\n---\n\n![logo](https://example.com/logo.png)\n\npara one\n\npara two\n";
        let out = html(md);
        assert!(out.contains("<blockquote"), "{out}");
        assert!(out.contains("<hr"), "{out}");
        assert!(
            out.contains("src=\"https://example.com/logo.png\""),
            "{out}"
        );
        assert!(out.contains("alt=\"logo\""), "{out}");
        assert!(out.matches("<p ").count() >= 2, "{out}");
    }

    #[test]
    fn drops_raw_html_and_javascript_urls() {
        let md = "<script>alert(1)</script>\n\n[x](javascript:alert(1))\n\n![x](data:text/html;base64,aaaa)\n";
        let out = html(md);
        assert!(!out.contains("<script"), "{out}");
        assert!(!out.contains("javascript:"), "{out}");
        assert!(!out.contains("data:"), "{out}");
        assert!(!out.contains("href=\"javascript"), "{out}");
    }

    #[test]
    fn allows_mailto_and_https() {
        let out = html("[mail](mailto:ops@example.com) <https://example.com/a>");
        assert!(out.contains("href=\"mailto:ops@example.com\""), "{out}");
        assert!(out.contains("href=\"https://example.com/a\""), "{out}");
    }

    #[test]
    fn setext_heading_and_indented_code() {
        let md = "Title\n=====\n\n    indented();\n";
        let out = html(md);
        assert!(out.contains("<h1"), "{out}");
        assert!(out.contains("indented();"), "{out}");
    }

    #[test]
    fn nested_inline_and_hard_break() {
        let out = html("**bold with `code`**  \nnext line");
        assert!(out.contains("<strong>"), "{out}");
        assert!(out.contains("<code"), "{out}");
        assert!(out.contains("<br>"), "{out}");
    }

    #[test]
    fn snippet_roundtrip() {
        let src = "hello, 世界\nline2";
        let enc = encode_snippet_param(src);
        assert!(!enc.contains('+') && !enc.contains('/') && !enc.contains('='));
        assert_eq!(decode_snippet_param(&enc).unwrap(), src);
    }

    #[test]
    fn escape_quotes_and_amp() {
        assert_eq!(escape_html(r#"a<b>&"'"#), "a&lt;b&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn gfm_alert_blockquote() {
        let md = "> [!WARNING]\n> Watch out\n";
        let out = html(md);
        assert!(
            out.contains("Warning") || out.contains("blockquote"),
            "{out}"
        );
        assert!(out.contains("Watch out"), "{out}");
    }

    #[test]
    fn preview_copy_button_uses_data_attr() {
        let out = render_markdown(
            "```js\nconsole.log(1)\n```\n",
            &MarkdownOptions::preview("https://mail.example.com"),
        );
        assert!(out.contains("xxm-copy-btn"), "{out}");
        assert!(out.contains("data-copy="), "{out}");
    }

    #[test]
    fn sent_mail_code_uses_snippet_link() {
        let out = render_markdown(
            "```js\nconsole.log(1)\n```\n",
            &MarkdownOptions::email("https://mail.example.com"),
        );
        assert!(out.contains("/snippet?e="), "{out}");
        assert!(out.contains("https://mail.example.com"), "{out}");
    }
}
