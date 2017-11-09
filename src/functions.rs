#![allow(dead_code)]

use syn::{MetaItem, Lit, Field, Attribute};
use quote::{Ident, Tokens};

const ATTRIBUTE_NAME: &'static str = "__function";

pub fn implement(field: &Field, impl_generics: &String) -> Tokens {

    let worker : fn(&Attribute, &String) -> Tokens = workerfun;

    let generated = field.attrs.iter().map(|x| { (
            worker(&x, &impl_generics)
        ) }).collect::<Vec<_>>();

    quote! {
        #(#generated)*
    }
}

pub fn workerfun(attr : &Attribute, impl_generics: &String) -> Tokens {
    if attr.name() != ATTRIBUTE_NAME {
        return quote! { }
    }
    match attr.value {
        // `#[get = "pub"]`
        MetaItem::NameValue(_, Lit::Str(ref s, _)) => {

            let classname = Ident::from(impl_generics.clone());
            let content = s;

            let content_top = get_fn_top(&content);
            let content_func = get_fn_contents(&content);

            let fn_pub = is_fn_public(&content_top);
            let fn_mutable = is_fn_mutable(&content_top);
            let fn_type = get_fn_type(&content_top, fn_pub, fn_mutable);
            let copy_fn_type = fn_type.clone();
            let fn_name = get_fn_name(&content_top, fn_pub, fn_mutable);
            let mut fn_arguments = get_fn_arguments(&content_top);

            let mut ref_self = String::from("&");

            if fn_mutable == true {
                ref_self = String::from("&mut");
            }

            if fn_arguments == "" {
                fn_arguments = format!("{} self", ref_self);
            } else {
                fn_arguments = format!("{} self, {}", ref_self, fn_arguments);
            }

            println!("content {}", content);

            println!("top {}", content_top);
            println!("content_func {}", content_func);

            println!("fn_type {}", fn_type);
            println!("fn_name {}", fn_name);
            println!("fn_arguments {}", fn_arguments);

            let ifn_type = Ident::from(fn_type);
            let ifn_name = Ident::from(fn_name);
            let ifn_arguments = Ident::from(fn_arguments);
            let icontent_func = Ident::from(content_func);
            let mut ifn_pub = Ident::from("");

            if fn_pub == true {
                ifn_pub = Ident::from("pub");
            }

            if copy_fn_type.contains("void") {

                return quote! {
                    #ifn_pub fn #ifn_name(#ifn_arguments) {
                        let mut this = self;
                        #icontent_func
                    }
                }
            } else {
                return quote! {
                    #ifn_pub fn #ifn_name(#ifn_arguments) -> #ifn_type {
                        let mut this = self;
                        #icontent_func
                    }
                }
            }
        },
        _ => panic!("Unexpected attribute parameters."),
    }
}

pub fn replace(s : &String, s1 : &String, s2 : &String) -> String {
    let mm = s.replace(s1, s2);
    return String::from(mm);
}

pub fn get_fn_top(s : &String) -> String {
    let split1 : Vec<&str> = s.splitn(2, "{").collect();
    let split2 : Vec<&str> = split1[0].splitn(2, "->").collect();
    let split : String = String::from(split2[0]);

    return split;
}

pub fn trim_braket(s : &String) -> String {
    let mut s_slice: &str = &*s;

    s_slice = s_slice.trim();
    s_slice = s_slice.trim_left_matches('{');
    s_slice = s_slice.trim_right_matches('}');
    return String::from(s_slice);
}

pub fn trim_parenthese(s : &String) -> String {
    let mut s_slice: &str = &*s;

    s_slice = s_slice.trim();
    s_slice = s_slice.trim_left_matches('(');
    s_slice = s_slice.trim_right_matches(')');
    return String::from(s_slice);
}

pub fn get_fn_contents(s : &String) -> String {
    let split1 : Vec<&str> = s.splitn(2, "{").collect();
    let split : String = String::from(split1[1]);

    return trim_braket(&split);
}

pub fn is_fn_public(s : &String) -> bool {
    let split1 : Vec<&str> = s.splitn(3, " ").collect();

    println!("SPLIT1 : {}", split1[0]);
    if split1[0] == "public" || split1[0] == "pub" {
        return true;
    }
    return false;
}

pub fn is_fn_mutable(s : &String) -> bool {
    let split1 : Vec<&str> = s.splitn(3, " ").collect();

    if split1[0] == "mutable"
        || split1[1] == "mutable" {
        return true;
    }
    return false;
}

pub fn get_fn_type(s : &String, ispub : bool, ismut : bool) -> String {
    let split1 : Vec<&str> = s.splitn(4, " ").collect();
    let mut index = 0;

    if ispub == true || ismut == true || split1[0] == "private" || split1[0] == "local" {
        index = 1;
    }
    if split1[0] == "immutable" || split1[1] == "immutable" {
        index += 1;
    }
    if ispub == true && ismut == true {
        index = 2;
    }
    let split : String = String::from(split1[index]);

    return split;
}

pub fn get_fn_name(s : &String, ispub : bool, ismut : bool) -> String {
    let split0 : Vec<&str> = s.splitn(3, " ").collect();
    let split1 : Vec<&str> = s.split("(").collect();
    let split2 : Vec<&str> = split1[0].splitn(5, " ").collect();
    let mut index = 1;

    if ispub == true || ismut == true || split0[0] == "private" || split0[0] == "local" {
        index = 2;
    }
    if split0[0] == "immutable" || split0[1] == "immutable" {
        index += 1;
    }
    if ispub == true && ismut == true {
        index = 3;
    }
    let split : String = String::from(split2[index]);

    return split;
}

pub fn get_fn_arguments(s : &String) -> String {
    let split1 : Vec<&str> = s.split("(").collect();
    let trimmed : String = trim_parenthese(&String::from(split1[1]));

    return trimmed;
}
