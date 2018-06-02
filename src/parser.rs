extern crate unicode_segmentation;
use self::unicode_segmentation::UnicodeSegmentation;

pub struct ASTNode<'a> {
    pub value: &'a str,
    pub children: Vec<ASTNode<'a>>
}

impl<'a> ASTNode<'a> {
    fn new(value: &str) -> ASTNode {
        ASTNode {
            value,
            children: Vec::new()
        }
    }
}

pub fn parse_string(input: &str) -> Option<ASTNode> {
    parse_program(&input)
}

fn parse_program(input: &str) -> Option<ASTNode> {
    parse_operator(&input)
}

fn parse_expression(input: &str) -> Option<ASTNode> {
    None
}

fn parse_operator(input: &str) -> Option<ASTNode> {
    let mut iter = input.graphemes(true);
    match iter.next() {
        Some(value) => {
            if value == "+" {
                Some(ASTNode::new(value))
            } else {
                None
            }
        },
        None => None
    }
}

fn parse_number(input: &str) -> Option<ASTNode> {
    None
}
