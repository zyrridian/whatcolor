/// Thin wrapper around `arboard` for clipboard operations.

pub struct Clipboard {
    inner: arboard::Clipboard,
}

impl Clipboard {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            inner: arboard::Clipboard::new()?,
        })
    }

    pub fn set_text(&mut self, text: &str) -> anyhow::Result<()> {
        self.inner.set_text(text)?;
        Ok(())
    }
}
