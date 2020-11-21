
use std::borrow::Cow;
use std::cell::RefCell;

use rand::Rng;
use rand::XorShiftRng;

thread_local! {
    static RNG: RefCell<XorShiftRng> = RefCell::new(XorShiftRng::new_unseeded());
}

#[derive(Debug)]
pub struct RSXElementPlaceholder(Cow<'static, str>);

impl RSXElementPlaceholder {
    pub fn dummy() -> Self {
        RSXElementPlaceholder(Cow::from(""))
    }

    pub fn generate() -> Self {
        let placeholder = format!("/* rsx:{} */", RNG.with(|v| v.borrow_mut().next_u64()));
        RSXElementPlaceholder(Cow::from(placeholder))
    }
}

impl AsRef<str> for RSXElementPlaceholder {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for RSXElementPlaceholder {
    fn eq(&self, _: &RSXElementPlaceholder) -> bool {
        true
    }
}
