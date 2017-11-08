use syn::{MetaItem, Lit, Field};
use quote::{Ident, Tokens};

const ATTRIBUTE_NAME: &'static str = "__constructor";
const FN_NAME_PREFIX: &'static str = "__constructor";
const FN_NAME_SUFFIX: &'static str = "";

pub fn implement(field: &Field, impl_generics: &String) -> Tokens {
    let field_name = field.clone().ident.expect("Expected the field to have a name");
    let fn_name = Ident::from(format!("{}{}{}", FN_NAME_PREFIX, field_name, FN_NAME_SUFFIX));
    let ty = field.ty.clone();

    let attr = field.attrs.iter()
        .filter(|v| v.name() == ATTRIBUTE_NAME)
        .last();

    let doc = field.attrs.iter()
        .filter(|v| v.name() == "doc")
        .collect::<Vec<_>>();

    match attr {
        Some(attr) => {
            match attr.value {
                // `#[get]`
                MetaItem::Word(_) => {
                    quote! {
                        #(#doc)*
                        fn #fn_name(&mut self) -> &mut #ty {
                            &mut self.#field_name
                        }
                    }
                },
                // `#[get = "pub"]`
                MetaItem::NameValue(_, Lit::Str(ref s, _)) => {

                    //let visibility = Ident::from(s.clone());
                    let classname = Ident::from(impl_generics.clone());
                    println!("ClassName : {}", impl_generics);
                    let split1 : Vec<&str> = s.trim().split("{").collect();
                    let split2 : Vec<&str> = split1[0].split("(").collect();
                    let split3 : Vec<&str> = split2[1].split(")").collect();


                    let values = Ident::from(String::from(split3[0]));
                    println!("VALUES --> {}", values);

                    let mut vars : Vec<&str> = split3[0].split(",").collect();

                    for i in 0..vars.len() {
                        let svar : Vec<&str> = vars[i].split(":").collect();

                        vars[i] = &svar[0].trim();
                    }
                    let mut cr = String::from("");
                    for tmp in vars {
                        if cr.len() > 0 {
                            cr = format!("{}, ", cr);
                        }
                        cr = format!("{}{}: {}", cr, tmp, tmp);
                    }
                    let vals = Ident::from(cr);

                    let splitcontent : Vec<&str> = split1[1].trim().split("}").collect();
                    let content = Ident::from(splitcontent[0]);
                    println!("CONTENT --> {}", splitcontent[0]);

                    quote! {
                        #(#doc)*
                        pub fn __constructor(#values) -> #classname {
                            let mut this : #classname = #classname {
                                #vals
                            };
                            #content
                            return this;
                        }
                    }
                },
                _ => panic!("Unexpected attribute parameters."),
            }
        },
        // Don't need to do anything.
        None => quote! { }
    }
}
