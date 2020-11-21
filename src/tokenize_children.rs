
use djed_self_tokenize_trait::{ToCustomTokens, Tokens};

use parse_children_types::{RSXChild, RSXChildren, RSXText};

impl ToCustomTokens for RSXChildren {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let children = &self.0;
        tokens.append(quote! { vec!#children });
    }
}

impl ToCustomTokens for RSXChild {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXChild::Element(ref element) => element.to_custom_tokens(tokens),
            &RSXChild::Text(ref text) => tokens.append(quote! { DOMNode::from(#text) }),
            &RSXChild::CodeBlock(ref code) => tokens.append(quote! { DOMNode::from(#code) })
        }
    }
}

impl ToCustomTokens for RSXText {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let string_ref: &str = self.0.as_ref();
        tokens.append(quote! { #string_ref });
    }
}
