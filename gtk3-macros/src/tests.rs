use crate::composite_template_derive::impl_composite_template;

fn diagnostic(input: syn::DeriveInput, expected: &str) {
    let output = impl_composite_template(&input).to_string();
    assert!(output.contains("compile_error"), "{output}");
    assert!(output.contains(expected), "{output}");
}

#[test]
fn missing_template_reports_the_required_attribute() {
    diagnostic(
        syn::parse_quote!(
            struct Widget;
        ),
        "Missing 'template' attribute",
    );
}

#[test]
fn malformed_template_reports_the_invalid_literal() {
    diagnostic(
        syn::parse_quote!(
            #[template(string = 123)]
            struct Widget;
        ),
        "expected string literal",
    );
}

#[test]
fn enum_template_reports_the_supported_item_kind() {
    diagnostic(
        syn::parse_quote!(
            #[template(string = "<interface/>")]
            enum Widget {}
        ),
        "only supports structs",
    );
}

#[test]
fn invalid_child_reports_the_supported_attribute_argument() {
    diagnostic(
        syn::parse_quote! {
            #[template(string = "<interface/>")]
            struct Widget {
                #[template_child(unknown = "label")]
                label: gtk::TemplateChild<gtk::Label>,
            }
        },
        "expected `id`",
    );
}
