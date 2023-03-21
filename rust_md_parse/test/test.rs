mod parseBlock {
    fn parseHead() {
        let head = [
            "#test",
            "# test",
            "# Metus arcu odio",
            "### Dui eu eget purus.",
        ];
        let tree = block::parseHead(head);
    }
}
