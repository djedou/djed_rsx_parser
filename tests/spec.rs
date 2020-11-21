
extern crate djed_rsx_parser;
extern crate djed_self_tokenize_trait;

use djed_rsx_parser::parse;
use djed_self_tokenize_trait::{ToCustomTokens, Tokens};

use djed_rsx_parser::parse_attributes_types::{
    RSXAttributes,
};
use djed_rsx_parser::parse_children_types::{
    RSXText,
    RSXChild,
    RSXChildren
};
use djed_rsx_parser::parse_elements_types::{
    RSXElement,
    RSXElementName,
    RSXIdentifier,
    RSXNormalElement
};
#[test]
pub fn test_simple() {
    let source = "<div>Hello world!</div>";
    let (ast, remaining): (RSXElement, _) = parse(source).unwrap();

    assert_eq!(
        (ast, remaining),
        (
            RSXElement::Normal(RSXNormalElement(
                RSXElementName::Name(RSXIdentifier("div".to_string())),
                RSXAttributes::from(vec![]),
                RSXChildren::from(vec![RSXChild::Text(RSXText("Hello world!".to_string()))])
            )),
            ""
        )
    );
}

#[test]
pub fn test_tokenize_1() {
    let source = "<foo>Hello world!</foo>";
    let (ast, _): (RSXElement, _) = parse(source).unwrap();

    let mut tokens = Tokens::new();
    ast.to_custom_tokens(&mut tokens);

    assert_eq!(
        tokens.to_string(),
        "DOMNode :: from ( ( DOMTagName :: from ( \"foo\" ) , DOMChildren :: from ( vec ! [ \
         DOMNode :: from ( \"Hello world!\" ) , ] ) ) )"
    );
}

#[test]
pub fn test_tokenize_2() {
    let source = "<div hidden style={stylesheet.get(\".foo\")}>Hello world!</div>";
    let (ast, _): (RSXElement, _) = parse(source).unwrap();

    let mut tokens = Tokens::new();
    ast.to_custom_tokens(&mut tokens);

    assert_eq!(
        tokens.to_string(),
        "DOMNode :: from ( ( DOMTagName :: from ( KnownElementName :: Div ) , vec ! [ \
         DOMAttribute :: from ( ( DOMAttributeName :: from ( KnownAttributeName :: Hidden ) , \
         DOMAttributeValue :: from ( true ) ) ) , DOMAttribute :: from ( ( DOMAttributeName :: \
         from ( KnownAttributeName :: Style ) , DOMAttributeValue :: from ( \
         {stylesheet.get(\".foo\")} ) ) ) , ] , vec ! [ DOMNode :: from ( \"Hello world!\" ) , ] \
         ) )",
    );
}

#[test]
pub fn test_tokenize_3() {
    let source = "<x-foo-bar>Hello world!</x-foo-bar>";
    let (ast, _): (RSXElement, _) = parse(source).unwrap();

    let mut tokens = Tokens::new();
    ast.to_custom_tokens(&mut tokens);

    assert_eq!(
        tokens.to_string(),
        "DOMNode :: from ( ( DOMTagName :: from ( \"x-foo-bar\" ) , DOMChildren :: from ( vec ! [ \
         DOMNode :: from ( \"Hello world!\" ) , ] ) ) )"
    );
}
