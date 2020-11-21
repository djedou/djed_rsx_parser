
# djed_rsx_parser

JSX-like parser combinator for Rust

## Purpose
This is an experimental parser for JSX-like code in Rust. The long term goal might be to build "something like React" in Rust, but this can mean a number of things, from a direct port with 100% API compatibility to a completely different product. A JSX-like parser is a good and simple place to start experimenting from.

## How to use
add this to your `Cargo.toml` file:

```toml
[dependencies]
djed_rsx_parser = { git = "https://github.com/djedou/djed_rsx_parser.git" }
```

All data structures (including the AST) are [self tokenizing](https://github.com/victorporof/rust_self_tokenize.git) values, meaning that they can serialize themselves to generate a `quote::Tokens` which can be then directly translated into a `proc_macro::TokenStream`, used for creating Rust compiler plugins as [procedural macros](https://doc.rust-lang.org/book/first-edition/procedural-macros.html). See the [syn](https://github.com/dtolnay/syn) and [quote](https://github.com/dtolnay/quote) crates for more information.

Use the [rust-self-tokenize](https://github.com/victorporof/rust_self_tokenize.git) or [quote](https://github.com/dtolnay/quote) crates for the `quote::Tokens` type.

```rust
extern crate self_tokenize_trait;
use self_tokenize_trait::{ToCustomTokens, Tokens};

let source = "<div>{ external_rust_code() }</div>";
let (ast, remaining) = parse(source).unwrap();
let mut tokens = Tokens::new();
ast.to_custom_tokens(&mut tokens);
```

## RSX vs. JSX
The [JSX spec](http://facebook.github.io/jsx) is, although a draft, presumably stable. Syntax extension equivalents can be found for Rust, which is the main scope of this experiment.

Example, inspired from the JSX spec website linked above:

```jsx
const FunDropdown = (props) =>
  <Dropdown show={props.visible}>
    A dropdown list
    <Menu
      icon={props.menu.icon}
      onHide={(e) => console.log(e)}
      onShow={(e) => console.log(e)}
    >
      <MenuItem>Do Something</MenuItem>
      {
        shouldDoSomethingFun()
          ? <MenuItem>Do Something Fun!</MenuItem>
          : <MenuItem>Do Something Else</MenuItem>
      }
    </Menu>
  </Dropdown>;
```

An equivalent interpretation of JSX in Rust, using compiler plugins, looks this:

```rust
fn fun_dropdown(props: Props) -> RSXElement {
  rsx! {
    <Dropdown show={props.visible}>
      A dropdown list
      <Menu
        icon={props.menu.icon}
        onHide={|e: Event| println!("{:?}", e)}
        onShow={|e: Event| println!("{:?}", e)}
      >
        <MenuItem>Do Something</MenuItem>
        {
          if should_do_something_fun() {
            <MenuItem>Do Something Fun!</MenuItem>
          } else {
            <MenuItem>Do Something Else</MenuItem>
          }
        }
      </Menu>
    </Dropdown>
  }
}
```

## Supported grammar
All of the [JSX official grammar](http://facebook.github.io/jsx) is supported. In the case of handling arbitrary Rust code inside RSX, the treatment is similar: JSX can contain JavaScript "code blocks" delimited by curly braces (specifically "assignment expressions"), in clearly defined locations such as attribute values, children contents etc. Rust expressions provide sufficient equivalence.

### PrimaryExpression
- [X] JSXElement

### Elements

#### JSXElement
- [X] JSXSelfClosingElement
- [X] JSXOpeningElement JSXChildren? JSXClosingElement

#### JSXSelfClosingElement
- [X] `<` JSXElementName JSXAttributes? `/` `>`

#### JSXOpeningElement
- [X] `<` JSXElementName JSXAttributes? `>`

#### JSXClosingElement
- [X] `<` `/` JSXElementName `>`

#### JSXElementName
- [X] JSXIdentifier
- [X] JSXNamedspacedName
- [X] JSXMemberExpression

#### JSXIdentifier
- [X] IdentifierStart
- [X] JSXIdentifier IdentifierPart
- [X] JSXIdentifier `-`

#### JSXNamespacedName
- [X] JSXIdentifier `:` JSXIdentifier

#### JSXMemberExpression
- [X] JSXIdentifier `.` JSXIdentifier
- [X] JSXMemberExpression `.` JSXIdentifier

### Attributes

#### JSXAttributes
- [X] JSXSpreadAttribute JSXAttributes?
- [X] JSXAttribute JSXAttributes?

#### JSXSpreadAttribute
- [X] `{` ... AssignmentExpression `}`

#### JSXAttribute
- [X] JSXAttributeName `=` JSXAttributeValue

#### JSXAttributeName
- [X] JSXIdentifier
- [X] JSXNamespacedName

#### JSXAttributeValue
- [X] `"` JSXDoubleStringCharacters? `"`
- [X] `'` JSXSingleStringCharacters? `'`
- [X] `{` AssignmentExpression `}`
- [X] JSXElement

#### JSXDoubleStringCharacters
- [X] JSXDoubleStringCharacter JSXDoubleStringCharacters?

#### JSXDoubleStringCharacter
- [X] SourceCharacter *but not* `"`

#### JSXSingleStringCharacters
- [X] JSXSingleStringCharacter JSXSingleStringCharacters?

#### JSXSingleStringCharacter
- [X] SourceCharacter *but not* `'`

### Children

#### JSXChildren
- [X] JSXChild JSXChildren?

#### JSXChild
- [X] JSXText
- [X] JSXElement
- [X] `{` AssignmentExpression? `}`

#### JSXText
- [X] JSXTextCharacter JSXText?

#### JSXTextCharacter
- [X] SourceCharacter *but not one of* `{`, `<`, `>` *or* `}`
