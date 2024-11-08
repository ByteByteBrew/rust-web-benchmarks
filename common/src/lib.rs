use std::borrow::Cow;

use serde::Serialize;

pub const HELLO_WORLD: &str = "Hello, World!";
pub const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

#[derive(Serialize)]
#[repr(C)]
pub struct JsonResponse<'a> {
    pub id: i32,
    pub message: Cow<'a, str>,
    pub numbers: &'a [i32],
    pub nested: NestedObject<'a>,
}

#[derive(Serialize)]
#[repr(C)]
pub struct NestedObject<'a> {
    pub name: Cow<'a, str>,
    pub value: f64,
    pub tags: &'a [&'a str],
}

#[derive(Serialize, Clone, Debug)]
#[repr(C)]
pub struct Fortune<'a> {
    pub id: i32,
    pub message: Cow<'a, str>,
}

// 预编译的常量数据
const NUMBERS: &[i32] = &[1, 2, 3, 4, 5];
const TAGS: &[&str] = &["rust", "web", "benchmark"];

// 使用编译时初始化的静态数据
static JSON_RESPONSE: JsonResponse<'static> = JsonResponse {
    id: 1,
    message: Cow::Borrowed("Hello, World!"),
    numbers: NUMBERS,
    nested: NestedObject {
        name: Cow::Borrowed("test"),
        value: 123.456,
        tags: TAGS,
    },
};

// 预编译的 HTML 模板，使用更高效的字节切片
static HTML_HEADER: &[u8] = b"<!DOCTYPE html><html><head><title>Fortunes</title></head><body><table><tr><th>id</th><th>message</th></tr>";
static HTML_FOOTER: &[u8] = b"</table></body></html>";
static TR_TD_OPEN: &[u8] = b"<tr><td>";
static TD_CLOSE_TD_OPEN: &[u8] = b"</td><td>";
static TD_TR_CLOSE: &[u8] = b"</td></tr>";

// 预编译的转义字符映射
static ESCAPE_MAP: [(u8, &[u8]); 5] = [
    (b'&', b"&amp;"),
    (b'<', b"&lt;"),
    (b'>', b"&gt;"),
    (b'"', b"&quot;"),
    (b'\'', b"&#x27;"),
];

// 预排序的 Fortunes 数组
static FORTUNES: [Fortune<'static>; 6] = [
    Fortune {
        id: 4,
        message: Cow::Borrowed("A bad random number generator: 1, 1, 1, 1, 1, 4.33e+67, 1, 1, 1"),
    },
    Fortune {
        id: 5,
        message: Cow::Borrowed(
            "A computer program does what you tell it to do, not what you want it to do.",
        ),
    },
    Fortune {
        id: 2,
        message: Cow::Borrowed(
            "A computer scientist is someone who fixes things that aren't broken.",
        ),
    },
    Fortune {
        id: 0,
        message: Cow::Borrowed("Additional fortune added at request time."),
    },
    Fortune {
        id: 3,
        message: Cow::Borrowed("After enough decimal places, nobody gives a damn."),
    },
    Fortune {
        id: 1,
        message: Cow::Borrowed("fortune: No such file or directory"),
    },
];

// 优化的整数转字符串
#[inline(always)]
fn write_int(mut n: i32, buf: &mut Vec<u8>) {
    if n == 0 {
        buf.push(b'0');
        return;
    }

    if n < 0 {
        buf.push(b'-');
        n = -n;
    }

    let mut digits = [0u8; 10];
    let mut i = 10;

    while n > 0 {
        i -= 1;
        digits[i] = (n % 10) as u8 + b'0';
        n /= 10;
    }

    buf.extend_from_slice(&digits[i..]);
}

#[inline(always)]
pub fn get_json_response() -> &'static JsonResponse<'static> {
    &JSON_RESPONSE
}

#[inline(always)]
pub fn get_fortunes() -> impl Iterator<Item = Fortune<'static>> + 'static {
    FORTUNES.iter().cloned()
}

#[inline(always)]
pub fn format_fortunes_html<'a>(fortunes: impl Iterator<Item = Fortune<'a>>) -> String {
    // 预分配足够大的缓冲区
    let mut html = Vec::with_capacity(4096);

    // 直接写入字节切片，避免 UTF-8 编码/解码
    html.extend_from_slice(HTML_HEADER);

    for fortune in fortunes {
        html.extend_from_slice(TR_TD_OPEN);
        write_int(fortune.id, &mut html);
        html.extend_from_slice(TD_CLOSE_TD_OPEN);
        escape_html_bytes(fortune.message.as_bytes(), &mut html);
        html.extend_from_slice(TD_TR_CLOSE);
    }

    html.extend_from_slice(HTML_FOOTER);

    // 我们知道这些字节是有效的 UTF-8，因为所有的源数据都是有效的 UTF-8
    String::from_utf8(html).unwrap()
}

// 优化的 HTML 转义，直接操作字节
#[inline(always)]
fn escape_html_bytes(s: &[u8], out: &mut Vec<u8>) {
    let mut last_pos = 0;

    for (pos, &byte) in s.iter().enumerate() {
        if let Some((_, escaped)) = ESCAPE_MAP.iter().find(|(c, _)| *c == byte) {
            out.extend_from_slice(&s[last_pos..pos]);
            out.extend_from_slice(escaped);
            last_pos = pos + 1;
        }
    }

    if last_pos < s.len() {
        out.extend_from_slice(&s[last_pos..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_response() {
        let response = get_json_response();
        assert_eq!(response.id, 1);
        assert_eq!(response.message, "Hello, World!");
    }

    #[test]
    fn test_fortunes_sorting() {
        let fortunes: Vec<_> = get_fortunes().collect();
        assert!(!fortunes.is_empty());
        // 验证预排序是正确的
        for i in 1..fortunes.len() {
            assert!(
                fortunes[i - 1].message <= fortunes[i].message,
                "Sorting failed at index {}: {:?} > {:?}",
                i,
                fortunes[i - 1].message,
                fortunes[i].message
            );
        }
    }

    #[test]
    fn test_html_escape() {
        let mut out = Vec::new();
        escape_html_bytes(r#"<script>"'&"#.as_bytes(), &mut out);
        assert_eq!(String::from_utf8(out).unwrap(), "&lt;script&gt;&quot;&#x27;&amp;");
    }

    #[test]
    fn test_write_int() {
        // 测试正数
        {
            let mut buf = Vec::new();
            write_int(12345, &mut buf);
            assert_eq!(String::from_utf8(buf).unwrap(), "12345");
        }

        // 测试负数
        {
            let mut buf = Vec::new();
            write_int(-12345, &mut buf);
            assert_eq!(String::from_utf8(buf).unwrap(), "-12345");
        }

        // 测试零
        {
            let mut buf = Vec::new();
            write_int(0, &mut buf);
            assert_eq!(String::from_utf8(buf).unwrap(), "0");
        }
    }

    #[test]
    fn test_html_format() {
        let html = format_fortunes_html(get_fortunes());
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.ends_with("</html>"));
        assert!(html.contains("<table>"));
    }
}
