use proc_macro::TokenStream;
use quote::*;
use syn::*;

#[proc_macro_derive(NameString)]
pub fn impl_name(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(input);
    let struct_name = ident;

    let expand = quote! {
        impl NameString for #struct_name {
            fn name_string(&self) -> String {
                self.name.to_string()
            }
        }

        impl NameString for &#struct_name {
            fn name_string(&self) -> String {
                self.name.to_string()
            }
        }
    };

    TokenStream::from(expand)
}

#[proc_macro_derive(LinePosition)]
pub fn impl_line_pos(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(input);
    let struct_name = ident;

    let expand = quote! {
        impl LinePosition for #struct_name {
            fn line_position(&self) -> usize {
                self.line_pos
            }
        }

        impl LinePosition for &#struct_name {
            fn line_position(&self) -> usize {
                self.line_pos
            }
        }
    };

    TokenStream::from(expand)
}

// --- exmaple -----------------------
//
//    async fn value_from_db(
//        &self,
//        ctx: &Context<'_>,
//        #[graphql(desc = "Id of object")] id: i64
//    ) -> Result<String> {
//        let conn = ctx.data::<DbPool>()?.take();
//        Ok(conn.query_something(id)?.name)
//    }
//
//
//#[Object]
//impl Query {
//    #[field(desc = "\"me: Single-line comment\"")]
//    pub async fn me(&self, ctx: &Context<'_>) -> Me {
//        ctx.data_unchecked::<DataSource>().me()
//    }
//    pub async fn active(&self) -> bool {
//        self.active.clone()
//    }
//}
