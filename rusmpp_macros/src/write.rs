pub fn derive_rusmpp_io_write_struct(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &input.ident;
    let fields_with_skip_options =
        crate::utils::collect_skip_options_from_from_data("rusmpp_io_write", &input.data);

    let async_wirte_fields = fields_with_skip_options.iter().map(|options| {
        let field_name = &options.name_ident;
        if options.skip {
            return quote::quote! {};
        }

        quote::quote! {
            self.#field_name.async_io_write(buf).await?;
        }
    });

    let wirte_fields = fields_with_skip_options.iter().map(|options| {
        let field_name = &options.name_ident;
        if options.skip {
            return quote::quote! {};
        }

        quote::quote! {
            self.#field_name.io_write(buf)?;
        }
    });

    let expanded = quote::quote! {
        #[async_trait::async_trait]
        impl rusmpp_io::io::write::AsyncIoWrite for #struct_name {
            async fn async_io_write(&self, buf: &mut rusmpp_io::io::write::AsyncIoWritable) -> std::io::Result<()> {
                #(#async_wirte_fields)*;

                Ok(())
            }
        }

        impl rusmpp_io::io::write::IoWrite for #struct_name {
            fn io_write(&self, buf: &mut rusmpp_io::io::write::IoWritable) -> std::io::Result<()> {
                #(#wirte_fields)*;

                Ok(())
            }
        }
    };

    expanded.into()
}

pub fn derive_rusmpp_io_write_enum(_input: syn::DeriveInput) -> proc_macro::TokenStream {
    panic!("Not implemented yet")
}
