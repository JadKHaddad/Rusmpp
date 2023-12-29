use quote::quote;

pub enum PrimitiveType {
    U8,
    U16,
    U32,
}

impl quote::ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            PrimitiveType::U8 => quote! { u8 },
            PrimitiveType::U16 => quote! { u16 },
            PrimitiveType::U32 => quote! { u32 },
        }
        .to_tokens(tokens)
    }
}

pub fn derive_rusmpp_io_primitive(
    name: &proc_macro2::Ident,
    primitive_type: PrimitiveType,
) -> proc_macro::TokenStream {
    let expanded = quote! {

        impl rusmpp_io::io::length::IoLength for #name {
            fn length(&self) -> usize {
                #primitive_type::from(*self).length()
            }
        }

        #[async_trait::async_trait]
        impl rusmpp_io::io::write::AsyncIoWrite for #name {
            async fn async_io_write(&self, buf: &mut rusmpp_io::io::write::AsyncIoWritable) -> std::io::Result<()> {
                #primitive_type::from(*self).async_io_write(buf).await
            }
        }

        #[async_trait::async_trait]
        impl rusmpp_io::io::read::AsyncIoRead for #name {
            async fn async_io_read(buf: &mut rusmpp_io::io::read::AsyncIoReadable) -> Result<Self, rusmpp_io::io::read::IoReadError> {
                #primitive_type::async_io_read(buf).await.map(Self::from)
            }
        }
    };

    expanded.into()
}
