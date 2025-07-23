use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

fn todo_app_impl(input: &mut DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {    
    match &mut input.data {
        Data::Struct(data) => {
            match &mut data.fields {
                Fields::Named(fields) => {
                    for field in fields.named.iter_mut() {
                        let field_name = field.ident.as_ref().unwrap();
                        let pascal_name = to_pascal_case(&field_name.to_string());
                        let renamed  = format!("TodoApp{}",pascal_name);
                        
                        let rename_attr: syn::Attribute = syn::parse_quote!(
                            #[serde(rename = #renamed)]
                        );
                        field.attrs.push(rename_attr);
                    }
                }
                _ => panic!("Only Named fields are allowed")
            }
        },
        _ => panic!("Only Structs are allowed")
       }
    
       Ok(quote! {
        #input
       })
}

#[proc_macro_attribute]
pub fn todo_app(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    match todo_app_impl(&mut input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn to_pascal_case(s: &String) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars.as_str().to_lowercase().chars()).collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::{parse_quote, DeriveInput};
    use proc_macro2::TokenStream as TokenStream2;

    // Helper function to parse and transform a struct using our macro implementation
    fn apply_todo_app_macro(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
        let parsed_input: DeriveInput = syn::parse2(input)?;
        todo_app_impl(parsed_input)
    }

    #[test]
    fn test_simple_struct_transformation() {
        let input = quote! {
            struct User {
                name: String,
                email: String,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct User {
                #[serde(rename = "TodoAppName")]
                name: String,
                #[serde(rename = "TodoAppEmail")]
                email: String,
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_struct_with_snake_case_fields() {
        let input = quote! {
            struct UserProfile {
                first_name: String,
                last_name: String,
                email_address: String,
                phone_number: Option<String>,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct UserProfile {
                #[serde(rename = "TodoAppFirstName")]
                first_name: String,
                #[serde(rename = "TodoAppLastName")]
                last_name: String,
                #[serde(rename = "TodoAppEmailAddress")]
                email_address: String,
                #[serde(rename = "TodoAppPhoneNumber")]
                phone_number: Option<String>,
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_struct_with_visibility_modifiers() {
        let input = quote! {
            pub struct User {
                pub name: String,
                pub(crate) email: String,
                private_field: i32,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            pub struct User {
                #[serde(rename = "TodoAppName")]
                pub name: String,
                #[serde(rename = "TodoAppEmail")]
                pub(crate) email: String,
                #[serde(rename = "TodoAppPrivateField")]
                private_field: i32,
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_struct_with_existing_attributes() {
        let input = quote! {
            #[derive(Debug, Clone)]
            #[serde(rename_all = "camelCase")]
            struct User {
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<String>,
                #[doc = "User's email address"]
                email: String,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            #[derive(Debug, Clone)]
            #[serde(rename_all = "camelCase")]
            struct User {
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "TodoAppName")]
                name: Option<String>,
                #[doc = "User's email address"]
                #[serde(rename = "TodoAppEmail")]
                email: String,
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_struct_with_complex_types() {
        let input = quote! {
            struct ComplexStruct {
                vec_field: Vec<String>,
                hash_map_field: std::collections::HashMap<String, i32>,
                tuple_field: (String, i32, bool),
                reference_field: &'static str,
                generic_field: Option<Box<dyn std::fmt::Display>>,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct ComplexStruct {
                #[serde(rename = "TodoAppVecField")]
                vec_field: Vec<String>,
                #[serde(rename = "TodoAppHashMapField")]
                hash_map_field: std::collections::HashMap<String, i32>,
                #[serde(rename = "TodoAppTupleField")]
                tuple_field: (String, i32, bool),
                #[serde(rename = "TodoAppReferenceField")]
                reference_field: &'static str,
                #[serde(rename = "TodoAppGenericField")]
                generic_field: Option<Box<dyn std::fmt::Display>>,
            }
        };

        assert_tokens_eq(result, expected);
    }


    #[test]
    fn test_empty_struct() {
        let input = quote! {
            struct EmptyStruct {}
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct EmptyStruct {
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_single_field_struct() {
        let input = quote! {
            struct SingleField {
                value: i32,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct SingleField {
                #[serde(rename = "TodoAppValue")]
                value: i32,
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_struct_with_generics() {
        let input = quote! {
            struct GenericStruct<T, U> {
                field1: T,
                field2: U,
                field3: Vec<T>,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct GenericStruct<T, U> {
                #[serde(rename = "TodoAppField1")]
                field1: T,
                #[serde(rename = "TodoAppField2")]
                field2: U,
                #[serde(rename = "TodoAppField3")]
                field3: Vec<T>,
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    // #[test]
    fn test_struct_with_where_clause() {
        let input = quote! {
            struct ConstrainedStruct<T>
            where
                T: Clone + Send
            {
                data: T,
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct ConstrainedStruct<T>
            where
                T: Clone + Send
            {
                #[serde(rename = "TodoAppData")]
                data: T,
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    // #[test]
    fn test_struct_with_lifetimes() {
        let input = quote! {
            struct LifetimeStruct<'a> {
                name: &'a str,
                data: &'a [u8],
            }
        };

        let result = apply_todo_app_macro(input).unwrap();
        let expected = quote! {
            struct LifetimeStruct<'a> {
                #[serde(rename = "TodoAppName")]
                name: &'a str,
                #[serde(rename = "TodoAppData")]
                data: &'a [u8],
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

     // Helper function for comparing token streams with normalized formatting
     fn assert_tokens_eq(left: TokenStream2, right: TokenStream2) {
        // Normalize whitespace around angle brackets and other common formatting differences
        fn normalize_tokens(tokens: TokenStream2) -> String {
            tokens.to_string()
                .replace(" >", ">")
                .replace("> >", ">>")
                .replace("< ", "<")
                .replace(" <", "<")
                .replace(" ,", ",")
                .replace("  ", " ")
                .trim()
                .to_string()
        }
        
        let left_normalized = normalize_tokens(left);
        let right_normalized = normalize_tokens(right);
        
        if left_normalized != right_normalized {
            println!("Token comparison failed:");
            println!("Left:  {}", left_normalized);
            println!("Right: {}", right_normalized);
            assert_eq!(left_normalized, right_normalized);
        }
    }

    // Integration test to ensure the macro actually compiles and works
    // Note: This test requires the trybuild crate and test files
    #[test]
    #[ignore] // Ignore by default since it requires additional setup
    fn test_macro_compilation() {
        // This test ensures that the generated code compiles
        let t = trybuild::TestCases::new();
        t.pass("tests/integration/pass/*.rs");
        t.compile_fail("tests/integration/fail/*.rs");
    }
}