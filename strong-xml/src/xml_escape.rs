
use lazy_static::lazy_static;
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::collections::BTreeSet;

struct MultipleMemchr<'a> {
    needle: BTreeSet<&'a u8>,
}

impl<'a> MultipleMemchr<'a> {
    pub fn new (needle: &'a [u8]) -> Self {
        MultipleMemchr {
            needle : needle.into_iter().collect()
        }
    }

    pub fn find (&self, haystack: &[u8]) -> Option<usize> {
        let mut it = haystack.iter().enumerate();
        it.find_map(|n|{
                let (index,value) = n;
                if self.needle.contains(value) {
                    Some(index)
                }else {
                        None
                }
        })
    }
}


pub fn xml_escape(raw: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref ESCAPE_BYTES: MultipleMemchr<'static> = MultipleMemchr::new(&[b'<', b'>', b'&', b'\'', b'"']);
    }

    let bytes = raw.as_bytes();

    if let Some(off) = ESCAPE_BYTES.find(bytes) {
        let mut result = String::with_capacity(raw.len());
        
        result.push_str(&raw[0..off]);

        let mut pos = off + 1;

        match bytes[pos - 1] {
            b'<' => result.push_str("&lt;"),
            b'>' => result.push_str("&gt;"),
            b'&' => result.push_str("&amp;"),
            b'\'' => result.push_str("&apos;"),
            b'"' => result.push_str("&quot;"),
            _ => unreachable!(),
        }

        while let Some(off) = ESCAPE_BYTES.find(&bytes[pos..]) {
            
            result.push_str(&raw[pos..pos + off]);

            pos += off + 1;

            match bytes[pos - 1] {
                b'<' => result.push_str("&lt;"),
                b'>' => result.push_str("&gt;"),
                b'&' => result.push_str("&amp;"),
                b'\'' => result.push_str("&apos;"),
                b'"' => result.push_str("&quot;"),
                _ => unreachable!(),
            }
        }

        result.push_str(&raw[pos..]);

        Cow::Owned(result)
    } else {
        Cow::Borrowed(raw)
    }
}

#[test]
fn test_escape() {
    assert_eq!(xml_escape("< < <"), "&lt; &lt; &lt;");
    assert_eq!(xml_escape(">"), "&gt;");
    assert_eq!(xml_escape("<  > <"), "&lt;  &gt; &lt;");
    assert_eq!(
        xml_escape("<script>alert('Hello XSS')</script>"),
        "&lt;script&gt;alert(&apos;Hello XSS&apos;)&lt;/script&gt;"
    );

}

#[test]
fn test_memchr(){
    let escape = MultipleMemchr::new(&[b'<', b'>', b'&', b'\'', b'"']);

    assert_eq!(escape.find(br#"<  > <"#),Some(0));
    assert_eq!(escape.find(br#"  > <"#),Some(2));
} 