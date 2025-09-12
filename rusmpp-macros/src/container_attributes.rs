use syn::Ident;

#[derive(Debug)]
pub enum DecodeAttributes {
    Skip,
    Implement(DecodeImplementation),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum TestAttributes {
    Skip,
    Implement(TestImplementation),
}

#[derive(Debug)]
pub enum TestImplementation {
    Owned,
    Borrowed,
    All,
}

impl TestAttributes {
    pub fn extract(meta: syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        let ident: Ident = meta.value()?.parse()?;

        match ident.to_string().as_str() {
            "skip" => Ok(Self::Skip),
            "owned" => Ok(Self::Implement(TestImplementation::Owned)),
            "borrowed" => Ok(Self::Implement(TestImplementation::Borrowed)),
            "all" => Ok(Self::Implement(TestImplementation::All)),
            other => Err(meta.error(format!(
                "unknown decode attribute: {}, expected skip, owned, borrowed, or all",
                other
            ))),
        }
    }
}

impl Default for TestAttributes {
    fn default() -> Self {
        Self::Implement(TestImplementation::All)
    }
}
