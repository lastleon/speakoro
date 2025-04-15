use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, Token, Type};

/// Struct to parse input
struct AssocStaticData {
    enum_type: Ident,
    data_type: Type,
    mappings: Vec<(Ident, Expr)>,
}

impl Parse for AssocStaticData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse `type Enum = Keys;`
        input.parse::<Token![type]>()?;
        let _enum_ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let enum_type: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        // Parse `type Data = &'static str;`
        input.parse::<Token![type]>()?;
        let _data_ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let data_type: Type = input.parse()?;
        input.parse::<Token![;]>()?;

        // Parse `Keys::A => "hello", Keys::B => "yoyo"`
        let mut mappings = Vec::new();
        while !input.is_empty() {
            let _ = input.parse::<Ident>()?;
            input.parse::<Token![::]>()?;
            let key: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;
            let value: Expr = input.parse()?;

            mappings.push((key, value));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AssocStaticData {
            enum_type,
            data_type,
            mappings,
        })
    }
}

/// The procedural macro itself
#[proc_macro]
pub fn associate_static_data(input: TokenStream) -> TokenStream {
    let AssocStaticData {
        enum_type,
        data_type,
        mappings,
    } = parse_macro_input!(input as AssocStaticData);

    let num_match_arms = mappings.len();
    let array_name = format_ident!("__{}__STATIC_DATA", enum_type.to_string());

    let (keys, values): (Vec<Ident>, Vec<Expr>) = mappings.iter().map(|x| x.clone()).unzip();
    let indices = 0..keys.len();

    let expanded = quote! {
        static #array_name: &[#data_type; #num_match_arms] = &[ #(#values),* ];
        impl #enum_type {
            /// Get static data associated with the variant of the enum this is called on.
            pub fn static_data(&self) -> #data_type {
                let idx = match self {
                    #(#enum_type::#keys => #indices),*
                };

                #array_name[idx]
            }
        }
    };

    TokenStream::from(expanded)
}
