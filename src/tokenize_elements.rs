
use djed_rsx_shared::types::dom_types::KnownElementName;
use djed_self_tokenize_trait::{ToCustomTokens, Tokens};

use parse_elements_types::{
    RSXElement,
    RSXElementName,
    RSXIdentifier,
    RSXNormalElement,
    RSXSelfClosingElement
};

impl ToCustomTokens for RSXElement {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXElement::SelfClosing(ref element) => {
                element.to_custom_tokens(tokens);
            }
            &RSXElement::Normal(ref element) => {
                element.to_custom_tokens(tokens);
            }
        }
    }
}

impl ToCustomTokens for RSXSelfClosingElement {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let name = &self.0;
        let attributes = &self.1;
        let has_attributes = attributes.0.len() != 0;

        if has_attributes {
            tokens.append(quote! {
                DOMNode::from((#name, DOMAttributes::from(#attributes)))
            })
        } else {
            tokens.append(quote! {
                DOMNode::from(#name)
            })
        }
    }
}

impl ToCustomTokens for RSXNormalElement {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let name = &self.0;
        let attributes = &self.1;
        let children = &self.2;
        let has_attributes = attributes.0.len() != 0;
        let has_children = children.0.len() != 0;

        if has_attributes && has_children {
            tokens.append(quote! {
                DOMNode::from((#name, #attributes, #children))
            })
        } else if has_attributes {
            tokens.append(quote! {
                DOMNode::from((#name, DOMAttributes::from(#attributes)))
            })
        } else if has_children {
            tokens.append(quote! {
                DOMNode::from((#name, DOMChildren::from(#children)))
            })
        } else {
            tokens.append(quote! {
                DOMNode::from(#name)
            })
        }
    }
}

#[allow(unknown_lints, cyclomatic_complexity)]
impl ToCustomTokens for RSXElementName {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        //use self::KnownElementName::*;
        match self {
            // HTML content sectioning
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("address") => {
                RSXElementName::KnownName(KnownElementName::Address).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("article") => {
                RSXElementName::KnownName(KnownElementName::Article).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("aside") => {
                RSXElementName::KnownName(KnownElementName::Aside).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("footer") => {
                RSXElementName::KnownName(KnownElementName::Footer).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("header") => {
                RSXElementName::KnownName(KnownElementName::Header).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("nav") => {
                RSXElementName::KnownName(KnownElementName::Nav).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("section") => {
                RSXElementName::KnownName(KnownElementName::Section).to_custom_tokens(tokens);
            }

            // HTML text sectioning
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("hgroup") => {
                RSXElementName::KnownName(KnownElementName::Hgroup).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h1") => {
                RSXElementName::KnownName(KnownElementName::H1).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h2") => {
                RSXElementName::KnownName(KnownElementName::H2).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h3") => {
                RSXElementName::KnownName(KnownElementName::H3).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h4") => {
                RSXElementName::KnownName(KnownElementName::H4).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h5") => {
                RSXElementName::KnownName(KnownElementName::H5).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h6") => {
                RSXElementName::KnownName(KnownElementName::H6).to_custom_tokens(tokens);
            }

            // HTML text content
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("main") => {
                RSXElementName::KnownName(KnownElementName::Main).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("div") => {
                RSXElementName::KnownName(KnownElementName::Div).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("span") => {
                RSXElementName::KnownName(KnownElementName::Span).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("p") => {
                RSXElementName::KnownName(KnownElementName::P).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("ol") => {
                RSXElementName::KnownName(KnownElementName::Ol).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("ul") => {
                RSXElementName::KnownName(KnownElementName::Ul).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("li") => {
                RSXElementName::KnownName(KnownElementName::Li).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("dl") => {
                RSXElementName::KnownName(KnownElementName::Dl).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("dt") => {
                RSXElementName::KnownName(KnownElementName::Dt).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("dd") => {
                RSXElementName::KnownName(KnownElementName::Dd).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("figure") => {
                RSXElementName::KnownName(KnownElementName::Figure).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("figcaption") => {
                RSXElementName::KnownName(KnownElementName::Figcaption).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("hr") => {
                RSXElementName::KnownName(KnownElementName::Hr).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("pre") => {
                RSXElementName::KnownName(KnownElementName::Pre).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("blockquote") => {
                RSXElementName::KnownName(KnownElementName::Blockquote).to_custom_tokens(tokens);
            }

            // HTML inline text semantics
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("a") => {
                RSXElementName::KnownName(KnownElementName::A).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("b") => {
                RSXElementName::KnownName(KnownElementName::Bold).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("i") => {
                RSXElementName::KnownName(KnownElementName::Italic).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("u") => {
                RSXElementName::KnownName(KnownElementName::Underline).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("s") => {
                RSXElementName::KnownName(KnownElementName::Strikethrough).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("em") => {
                RSXElementName::KnownName(KnownElementName::Emphasis).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("mark") => {
                RSXElementName::KnownName(KnownElementName::Mark).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("q") => {
                RSXElementName::KnownName(KnownElementName::Quotation).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("cite") => {
                RSXElementName::KnownName(KnownElementName::Citation).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("code") => {
                RSXElementName::KnownName(KnownElementName::Code).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("data") => {
                RSXElementName::KnownName(KnownElementName::Data).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("time") => {
                RSXElementName::KnownName(KnownElementName::Time).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("sub") => {
                RSXElementName::KnownName(KnownElementName::Sub).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("sup") => {
                RSXElementName::KnownName(KnownElementName::Sup).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("br") => {
                RSXElementName::KnownName(KnownElementName::Br).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("wbr") => {
                RSXElementName::KnownName(KnownElementName::Wbr).to_custom_tokens(tokens);
            }

            // HTML media and links
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("img") => {
                RSXElementName::KnownName(KnownElementName::Image).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("area") => {
                RSXElementName::KnownName(KnownElementName::Area).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("map") => {
                RSXElementName::KnownName(KnownElementName::Map).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("audio") => {
                RSXElementName::KnownName(KnownElementName::Audio).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("video") => {
                RSXElementName::KnownName(KnownElementName::Video).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("track") => {
                RSXElementName::KnownName(KnownElementName::Track).to_custom_tokens(tokens);
            }

            // HTML forms
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("button") => {
                RSXElementName::KnownName(KnownElementName::Button).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("datalist") => {
                RSXElementName::KnownName(KnownElementName::Datalist).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("fieldset") => {
                RSXElementName::KnownName(KnownElementName::Fieldset).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("form") => {
                RSXElementName::KnownName(KnownElementName::Form).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("input") => {
                RSXElementName::KnownName(KnownElementName::Input).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("label") => {
                RSXElementName::KnownName(KnownElementName::Label).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("legend") => {
                RSXElementName::KnownName(KnownElementName::Legend).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("meter") => {
                RSXElementName::KnownName(KnownElementName::Meter).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("optgroup") => {
                RSXElementName::KnownName(KnownElementName::Optgroup).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("option") => {
                RSXElementName::KnownName(KnownElementName::Option).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("output") => {
                RSXElementName::KnownName(KnownElementName::Output).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("progress") => {
                RSXElementName::KnownName(KnownElementName::Progress).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("select") => {
                RSXElementName::KnownName(KnownElementName::Select).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("textarea") => {
                RSXElementName::KnownName(KnownElementName::Textarea).to_custom_tokens(tokens);
            }

            // React Fiber components
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("fragment") => {
                RSXElementName::KnownName(KnownElementName::Fragment).to_custom_tokens(tokens);
            }

            // React Native basic components
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("view") => {
                RSXElementName::KnownName(KnownElementName::View).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("text") => {
                RSXElementName::KnownName(KnownElementName::Text).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("image") => {
                RSXElementName::KnownName(KnownElementName::Image).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("textinput") => {
                RSXElementName::KnownName(KnownElementName::TextInput).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("scrollview") => {
                RSXElementName::KnownName(KnownElementName::ScrollView).to_custom_tokens(tokens);
            }

            // React Native user interface
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("picker") => {
                RSXElementName::KnownName(KnownElementName::Picker).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("slider") => {
                RSXElementName::KnownName(KnownElementName::Slider).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("switch") => {
                RSXElementName::KnownName(KnownElementName::Switch).to_custom_tokens(tokens);
            }

            // React Native list views
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("flatlist") => {
                RSXElementName::KnownName(KnownElementName::FlatList).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("sectionlist") => {
                RSXElementName::KnownName(KnownElementName::SectionList).to_custom_tokens(tokens);
            }

            // Parsed
            &RSXElementName::KnownName(ref n) => {
                tokens.append(quote! { DOMTagName::from(#n) });
            }
            &RSXElementName::Name(ref n) => {
                tokens.append(quote! { DOMTagName::from(#n) });
            }
            &RSXElementName::NamedspacedName(ref ns, ref n) => {
                tokens.append(quote! { DOMTagName::from((#ns, #n)) });
            }
            &RSXElementName::MemberExpression(ref member_expression) => {
                let mut inner_tokens = Tokens::new();
                member_expression.to_custom_tokens(&mut inner_tokens);
                tokens.append(quote! { DOMTagName::from(#inner_tokens) });
            }
        }
    }
}

impl ToCustomTokens for RSXIdentifier {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let string_ref: &str = self.0.as_ref();
        string_ref.to_custom_tokens(tokens);
    }
}
