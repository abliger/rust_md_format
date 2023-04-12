
pub mod parse_block {

    use rust_md_parse::block;
    #[test]
    fn parse_head() {
        let head = [
            "#tests",
            "# tests",
            "# Metus arcu odio",
            "### Dui eu eget purus.",
        ];
        block::parse_head(&head[0]);
    }
}
