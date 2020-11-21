
use std::fmt;
use std::iter::FromIterator;

use itertools::Itertools;
use djed_rsx_shared::types::dom_types::KnownElementName;
use djed_self_tokenize_macro::DefaultQuote;
use djed_self_tokenize_trait::ToCustomTokens;

use parse_attributes_types::RSXAttributes;
use parse_children_types::RSXChildren;

#[derive(Debug, PartialEq, DefaultQuote)]
pub enum RSXElement {
    SelfClosing(RSXSelfClosingElement),
    Normal(RSXNormalElement)
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub struct RSXSelfClosingElement(pub RSXElementName, pub RSXAttributes);

#[derive(Debug, PartialEq, DefaultQuote)]
pub struct RSXNormalElement(pub RSXElementName, pub RSXAttributes, pub RSXChildren);

#[derive(Debug, PartialEq)]
pub struct RSXOpeningElement(pub RSXElementName, pub RSXAttributes);

#[derive(Debug, PartialEq)]
pub struct RSXClosingElement<'a>(pub &'a RSXElementName);

#[derive(Debug, PartialEq, DefaultQuote)]
pub enum RSXElementName {
    KnownName(KnownElementName),
    Name(RSXIdentifier),
    NamedspacedName(RSXIdentifier, RSXIdentifier),
    MemberExpression(Box<[RSXIdentifier]>)
}

impl fmt::Display for RSXElementName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RSXElementName::*;
        match self {
            &Name(ref n) => write!(f, "{}", n.0),
            &NamedspacedName(ref ns, ref n) => write!(f, "{}:{}", ns.0, n.0),
            &MemberExpression(ref e) => write!(f, "{}", e.iter().map(|v| &v.0).join(".")),
            &KnownName(_) => {
                panic!("Unsupported operation: Known names only created during tokenization")
            }
        }
    }
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub struct RSXIdentifier(pub String);

impl FromIterator<RSXIdentifier> for RSXIdentifier {
    fn from_iter<I: IntoIterator<Item = RSXIdentifier>>(iter: I) -> Self {
        RSXIdentifier(iter.into_iter().map(|v| v.0).join("-"))
    }
}
