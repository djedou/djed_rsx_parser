
use djed_self_tokenize_trait::{ToCustomTokens, Tokens};

use parse_external_types::RSXParsedExpression;

impl ToCustomTokens for RSXParsedExpression {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let mut code = format!("{}{}{}", "{", self.tokens, "}");

        self.elements.iter().for_each(|&(ref placeholder, ref element)| {
            let mut inner_tokens = Tokens::new();
            element.to_custom_tokens(&mut inner_tokens);
            code = code.replace(placeholder.as_ref(), &inner_tokens.to_string());
        });

        tokens.append(code);
    }
}
