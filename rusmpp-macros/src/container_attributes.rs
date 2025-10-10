use syn::Ident;

/// `#[rusmpp(decode = skip|owned|borrowed|all)]`
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

/// `#[rusmpp(test = skip)]`
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

/// `#[rusmpp(from_into = skip)]`
#[derive(Default)]
pub enum FromIntoAttributes {
    Skip,
    #[default]
    Implement,
}

impl FromIntoAttributes {
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

    pub const fn is_implement(&self) -> bool {
        matches!(self, Self::Implement)
    }
}
