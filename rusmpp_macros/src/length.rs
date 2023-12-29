pub fn derive_rusmpp_io_length_struct(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &input.ident;
    let fields_with_skip_options =
        crate::utils::collect_skip_options_from_from_data("rusmpp_io_length", &input.data);

    let length_fields = fields_with_skip_options.iter().map(|options| {
        let field_name = &options.name_ident;

        if options.skip {
            return quote::quote! {};
        }

        quote::quote! {
            self.#field_name.length() +
        }
    });

    let expanded = quote::quote! {
        impl rusmpp_io::io::length::IoLength for #struct_name {
            fn length(&self) -> usize {
                #(#length_fields)* 0
            }
        }
    };

    expanded.into()
}

pub fn derive_rusmpp_io_length_enum(_input: syn::DeriveInput) -> proc_macro::TokenStream {
    panic!("Not implemented yet")
}
