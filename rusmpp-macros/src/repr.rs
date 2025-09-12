use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::container_attributes::{
    DecodeAttributes, DecodeImplementation, TestAttributes, TestImplementation,
};

pub struct Repr {
    ident: Ident,
}

impl Repr {
    pub fn new(repr_type: ReprType) -> Self {
        let ident = repr_type.to_ident();

        Self { ident }
    }
}

pub enum ReprType {
    U8,
    U16,
    U32,
}

impl ReprType {
    fn to_ident(&self) -> Ident {
        match self {
            Self::U8 => Ident::new("u8", proc_macro2::Span::call_site()),
            Self::U16 => Ident::new("u16", proc_macro2::Span::call_site()),
            Self::U32 => Ident::new("u32", proc_macro2::Span::call_site()),
        }
    }
}

impl Repr {
    fn quote_length_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl crate::encode::Length for #name {
                fn length(&self) -> usize {
                    #repr_ident::from(*self).length()
                }
            }
        }
    }

    fn quote_encode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl crate::encode::Encode for #name {
                fn encode(&self, dst: &mut [u8]) -> usize {
                    #repr_ident::from(*self).encode(dst)
                }
            }
        }
    }

    fn quote_owned_decode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl crate::decode::owned::Decode for #name {
                fn decode(src: &[u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
                    #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                }
            }
        }
    }

    fn quote_borrowed_decode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl<'a> crate::decode::borrowed::Decode<'a> for #name {
                fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
                    #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                }
            }
        }
    }

    fn quote_decode_impl(&self, name: &Ident, decode_attrs: &DecodeAttributes) -> TokenStream {
        match decode_attrs {
            DecodeAttributes::Skip => quote! {},
            DecodeAttributes::Implement(impl_type) => match impl_type {
                DecodeImplementation::Owned => self.quote_owned_decode_impl(name),
                DecodeImplementation::Borrowed => self.quote_borrowed_decode_impl(name),
                DecodeImplementation::All => {
                    let owned = self.quote_owned_decode_impl(name);
                    let borrowed = self.quote_borrowed_decode_impl(name);

                    quote! {
                        #owned
                        #borrowed
                    }
                }
            },
        }
    }

    fn quote_owned_test_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            #[cfg(test)]
            impl crate::test::owned::TestInstance for #name {
                fn instances() -> alloc::vec::Vec<Self> {
                    alloc::vec![Self::default(),]
                }
            }
        }
    }

    fn quote_borrowed_test_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            // TODO: Implement borrowed test instances
        }
    }

    fn quote_test_impl(&self, name: &Ident, test_attrs: &TestAttributes) -> TokenStream {
        match test_attrs {
            TestAttributes::Skip => quote! {},
            TestAttributes::Implement(impl_type) => match impl_type {
                TestImplementation::Owned => self.quote_owned_test_impl(name),
                TestImplementation::Borrowed => self.quote_borrowed_test_impl(name),
                TestImplementation::All => {
                    let owned = self.quote_owned_test_impl(name);
                    let borrowed = self.quote_borrowed_test_impl(name);

                    quote! {
                        #owned
                        #borrowed
                    }
                }
            },
        }
    }

    pub fn expand(
        &self,
        name: &Ident,
        decode_attrs: &DecodeAttributes,
        test_attrs: &TestAttributes,
    ) -> TokenStream {
        let length_impl = self.quote_length_impl(name);
        let encode_impl = self.quote_encode_impl(name);
        let decode_impl = self.quote_decode_impl(name, decode_attrs);
        let test_impl = self.quote_test_impl(name, test_attrs);

        quote! {
            #length_impl
            #encode_impl
            #decode_impl
            #test_impl
        }
    }
}
