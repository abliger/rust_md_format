use regex::Regex;

pub fn parse_head(head: &str) -> Option<Head> {
    let regex =
        Regex::new(r"^(\s*)?(?P<level>#{1,6})\s(?P<text>.*?)(?:\s#*)?$").expect("Regex fail");
    if regex.is_match(head) {
        let caps = regex.captures(head).unwrap();
        Some(Head {
            level: caps.name("level").unwrap().as_str().len(),
            content: caps.name("text").unwrap().as_str().trim(),
        })
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Head<'a> {
    level: usize,
    content: &'a str,
}

#[cfg(test)]
mod test {
    use crate::block::parse_head;

    #[test]
    fn parse_head_test() {
        assert_eq!(
            crate::block::Head {
                level: 3,
                content: "test"
            },
            parse_head("### test").unwrap()
        );
        assert_eq!(
            crate::block::Head {
                level: 3,
                content: "test"
            },
            parse_head("### test        ").unwrap()
        );
        assert_eq!(
            crate::block::Head {
                level: 3,
                content: "test"
            },
            parse_head("   ### test        ").unwrap()
        );
        assert_eq!(
            crate::block::Head {
                level: 3,
                content: "test"
            },
            parse_head("   ###   test        ").unwrap()
        );
        assert_eq!(None, parse_head(""));
        assert_eq!(None, parse_head("#test"));
        assert_eq!(None, parse_head("test# test"));
    }
}
