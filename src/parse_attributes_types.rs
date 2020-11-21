
use std::iter::FromIterator;

use djed_rsx_shared::types::dom_types::KnownAttributeName;
use djed_self_tokenize_macro::DefaultQuote;
use djed_self_tokenize_trait::ToCustomTokens;

use parse_elements_types::{RSXElement, RSXIdentifier};
use parse_external_types::RSXParsedExpression;
use parse_js_types::{JSBool, JSDoubleStringCharacters, JSNumber, JSSingleStringCharacters};

#[derive(Default, Debug, PartialEq, DefaultQuote)]
pub struct RSXAttributes(pub Box<[RSXAttribute]>);

impl From<Option<RSXAttributes>> for RSXAttributes {
    fn from(children: Option<RSXAttributes>) -> Self {
        children.unwrap_or_default()
    }
}

impl From<Vec<RSXAttribute>> for RSXAttributes {
    fn from(vec: Vec<RSXAttribute>) -> Self {
        RSXAttributes(vec.into_boxed_slice())
    }
}

impl FromIterator<RSXAttribute> for RSXAttributes {
    fn from_iter<I: IntoIterator<Item = RSXAttribute>>(iter: I) -> Self {
        RSXAttributes::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub enum RSXAttribute {
    Named(RSXAttributeName, RSXAttributeValue),
    Spread(RSXParsedExpression)
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub enum RSXAttributeName {
    KnownName(KnownAttributeName),
    Name(RSXIdentifier),
    NamedspacedName(RSXIdentifier, RSXIdentifier)
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub enum RSXAttributeValue {
    Default,
    Boolean(RSXAttributeBoolean),
    Number(RSXAttributeNumber),
    Str(RSXAttributeString),
    Element(RSXElement),
    CodeBlock(RSXParsedExpression)
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub struct RSXAttributeBoolean(pub bool);

impl From<JSBool> for RSXAttributeBoolean {
    fn from(v: JSBool) -> Self {
        RSXAttributeBoolean(v.0)
    }
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub struct RSXAttributeNumber(pub f64);

impl From<JSNumber> for RSXAttributeNumber {
    fn from(n: JSNumber) -> Self {
        RSXAttributeNumber(n.0)
    }
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub enum RSXAttributeString {
    SingleQuoted(JSSingleStringCharacters),
    DoubleQuoted(JSDoubleStringCharacters)
}
