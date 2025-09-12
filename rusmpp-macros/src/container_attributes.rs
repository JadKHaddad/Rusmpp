use syn::Ident;

pub enum DecodeAttributes {
    Skip,
    Implement(DecodeImplementation),
}

pub enum DecodeImplementation {
    Owned,
    Borrowed,
    All,
}

impl DecodeAttributes {
    pub fn extract(meta: syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        let ident: Ident = meta.value()?.parse()?;

        match ident.to_string().as_str() {
            "skip" => Ok(Self::Skip),
            "owned" => Ok(Self::Implement(DecodeImplementation::Owned)),
            "borrowed" => Ok(Self::Implement(DecodeImplementation::Borrowed)),
            "all" => Ok(Self::Implement(DecodeImplementation::All)),
            other => Err(meta.error(format!(
                "unknown decode attribute: {}, expected skip, owned, borrowed, or all",
                other
            ))),
        }
    }
}

impl Default for DecodeAttributes {
    fn default() -> Self {
        Self::Implement(DecodeImplementation::All)
    }
}

#[derive(Default)]
pub enum TestAttributes {
    Skip,
    #[default]
    Implement,
}

impl TestAttributes {
    pub fn extract(meta: syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        let ident: Ident = meta.value()?.parse()?;

        match ident.to_string().as_str() {
            "skip" => Ok(Self::Skip),
            other => Err(meta.error(format!(
                "unknown decode attribute: {}, expected skip",
                other
            ))),
        }
    }
}
