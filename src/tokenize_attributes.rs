
use djed_rsx_shared::types::dom_types::KnownAttributeName;
use djed_self_tokenize_trait::{ToCustomTokens, Tokens};

use parse_attributes_types::{
    RSXAttribute,
    RSXAttributeBoolean,
    RSXAttributeName,
    RSXAttributeNumber,
    RSXAttributeString,
    RSXAttributeValue,
    RSXAttributes
};

use parse_js_types::{JSDoubleStringCharacters, JSSingleStringCharacters};

impl ToCustomTokens for RSXAttributes {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let attributes = &self.0;
        tokens.append(quote! { vec!#attributes });
    }
}

impl ToCustomTokens for RSXAttribute {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXAttribute::Named(ref n, ref v) => {
                tokens.append(quote! { DOMAttribute::from((#n, #v)) });
            }
            &RSXAttribute::Spread(ref spread) => {
                tokens.append(quote! { DOMAttribute::from(#spread) });
            }
        }
    }
}

impl ToCustomTokens for RSXAttributeName {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        //use self::KnownAttributeName::*;
        match self {
            // HTML global attributes
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("accesskey") => {
                RSXAttributeName::KnownName(KnownAttributeName::Accesskey).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("class") => {
                RSXAttributeName::KnownName(KnownAttributeName::Class).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("contenteditable") => {
                RSXAttributeName::KnownName(KnownAttributeName::CntEditable).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("contextmenu") => {
                RSXAttributeName::KnownName(KnownAttributeName::Contextmenu).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("dir") => {
                RSXAttributeName::KnownName(KnownAttributeName::Dir).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("draggable") => {
                RSXAttributeName::KnownName(KnownAttributeName::Draggable).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("dropzone") => {
                RSXAttributeName::KnownName(KnownAttributeName::Dropzone).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("hidden") => {
                RSXAttributeName::KnownName(KnownAttributeName::Hidden).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("id") => {
                RSXAttributeName::KnownName(KnownAttributeName::Id).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("lang") => {
                RSXAttributeName::KnownName(KnownAttributeName::Lang).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("spellcheck") => {
                RSXAttributeName::KnownName(KnownAttributeName::Spellcheck).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("src") => {
                RSXAttributeName::KnownName(KnownAttributeName::Src).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("style") => {
                RSXAttributeName::KnownName(KnownAttributeName::Style).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("tabindex") => {
                RSXAttributeName::KnownName(KnownAttributeName::Tabindex).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("title") => {
                RSXAttributeName::KnownName(KnownAttributeName::Title).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("translate") => {
                RSXAttributeName::KnownName(KnownAttributeName::Translate).to_custom_tokens(tokens);
            }

            // Parsed
            &RSXAttributeName::KnownName(ref n) => {
                tokens.append(quote! { DOMAttributeName::from(#n) });
            }
            &RSXAttributeName::Name(ref n) => {
                tokens.append(quote! { DOMAttributeName::from(#n) });
            }
            &RSXAttributeName::NamedspacedName(ref ns, ref n) => {
                tokens.append(quote! { DOMAttributeName::from((#ns, #n)) });
            }
        }
    }
}

impl ToCustomTokens for RSXAttributeValue {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXAttributeValue::Default => {
                tokens.append(quote! { DOMAttributeValue::from(true) });
            }
            &RSXAttributeValue::Boolean(ref boolean) => {
                tokens.append(quote! { DOMAttributeValue::from(#boolean) });
            }
            &RSXAttributeValue::Number(ref number) => {
                tokens.append(quote! { DOMAttributeValue::from(#number) });
            }
            &RSXAttributeValue::Str(ref string) => {
                tokens.append(quote! { DOMAttributeValue::from(#string) });
            }
            &RSXAttributeValue::Element(ref element) => {
                tokens.append(quote! { DOMAttributeValue::from(#element) });
            }
            &RSXAttributeValue::CodeBlock(ref expression) => {
                tokens.append(quote! { DOMAttributeValue::from(#expression) });
            }
        }
    }
}

impl ToCustomTokens for RSXAttributeBoolean {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        self.0.to_custom_tokens(tokens);
    }
}
impl ToCustomTokens for RSXAttributeNumber {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        self.0.to_custom_tokens(tokens);
    }
}

impl ToCustomTokens for RSXAttributeString {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXAttributeString::SingleQuoted(JSSingleStringCharacters(ref chars))
            | &RSXAttributeString::DoubleQuoted(JSDoubleStringCharacters(ref chars)) => {
                let string_ref: &str = chars.as_ref();
                string_ref.to_custom_tokens(tokens);
            }
        }
    }
}
