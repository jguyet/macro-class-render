#![allow(dead_code)]

use syn::{MetaItem, Lit, Field};
use quote::{Ident, Tokens};

const ATTRIBUTE_NAME: &'static str = "__constructor";
const FN_NAME_PREFIX: &'static str = "__constructor";
const FN_NAME_SUFFIX: &'static str = "";

pub fn implement(field: &Field, impl_generics: &String) -> Tokens {

    let attr = field.attrs.iter()
        .filter(|v| v.name() == ATTRIBUTE_NAME)
        .last();

    let doc = field.attrs.iter()
        .filter(|v| v.name() == "doc")
        .collect::<Vec<_>>();

    match attr {
        Some(attr) => {
            match attr.value {
                // `#[get = "pub"]`
                MetaItem::NameValue(_, Lit::Str(ref s, _)) => {

                    //let visibility = Ident::from(s.clone());
                    let classname = Ident::from(impl_generics.clone());
                    //println!("ClassName : {}", impl_generics);
                    let top_string : String = get_fn_top(&s);
                    let top_const_var : String = get_contructor_types(&s, &impl_generics.clone());

                    //println!("TOP --> {}", top_string);
                    //println!("TOP RIGHT --> {}", top_const_var);

                    let content_string : String = replace(&get_fn_contents(&s), &String::from("\n"), &String::from(""));
                    let values = Ident::from(top_string.clone());
                    let top_arguments : Vec<&str> = get_contructor_arguments(&top_string);
                    let usablevals = get_vars_name(&top_arguments);

                    //println!("TOP RIGHTLL --> {:?}", usablevals);
                    //println!("TOP VALUES --> {:?}", values);

                    let constructor_vars = get_vars_name_for_constructor_two(&top_const_var);
                    let content = Ident::from(content_string);

                    quote! {
                        #(#doc)*
                        pub fn new(#values) -> #classname {
                            return #classname::__constructor(#usablevals);
                        }

                        pub fn __constructor(#values) -> #classname {
                            let mut this : #classname = #classname {
                                #constructor_vars
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

pub fn add_reference_string(s : &String) -> String {
    let tmp : String = format!("&{}", s);
    return tmp;
}

pub fn string_to_str(s : &String) -> &str {
    let s_slice: &str = &*s;

    return s_slice;
}

pub fn trim_string(s : &String) -> String {
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

pub fn replace(s : &String, s1 : &String, s2 : &String) -> String {
    let mm = s.replace(s1, s2);
    return String::from(mm);
}

pub fn get_fn_top(s : &String) -> String {
    let split1 : Vec<&str> = s.splitn(2, "{").collect();
    let split2 : Vec<&str> = split1[0].splitn(2, "->").collect();
    let split : String = String::from(split2[0]);

    return trim_parenthese(&split);
}

pub fn get_contructor_types(s : &String, classname : &String) -> String {
    let split1 : Vec<&str> = s.splitn(2, "{").collect();
    let split2 : Vec<&str> = split1[0].splitn(2, "->").collect();
    let mut split : String = String::from(split2[1]);

    split = replace(&split, &classname, &String::from(""));

    return trim_parenthese(&split);
}

pub fn get_fn_contents(s : &String) -> String {
    let split1 : Vec<&str> = s.splitn(2, "{").collect();
    let split : String = String::from(split1[1]);

    return trim_string(&split);
}

pub fn get_contructor_arguments(s : &String) -> Vec<&str> {
    let mut vars : Vec<&str> = s.split(",").collect();

    for i in 0..vars.len() {
        let svar : Vec<&str> = vars[i].split(":").collect();

        if svar[1].contains("&") {
            //let mut c : String = String::from(vars[i].to_string() + "&");
            //vars[i] = string_to_str(&c);
            vars[i] = "&test";//&svar[0].trim();
        } else {
            vars[i] = &svar[0].trim();
        }
    }
    return vars;
}

pub fn get_vars_name_for_constructor(vars : &Vec<&str>) -> Ident {
    let mut varsname = String::from("");

    for i in 0..vars.len() {
        if varsname.len() > 0 {
            varsname = format!("{}, ", varsname);
        }

        varsname = format!("{}{}: {}", varsname, vars[i], vars[i]);
    }

    return Ident::from(varsname);
}

pub fn get_vars_name_for_constructor_two(types : &String) -> Ident {
    let mut varsname = String::from("");
    let vars : Vec<&str> = types.split(",").collect();

    for i in 0..vars.len() {
        let svar : Vec<&str> = vars[i].split(":").collect();

        if varsname.len() > 0 {
            varsname = format!("{}, ", varsname);
        }

        if svar[1].contains("i8") || svar[1].contains("i16") || svar[1].contains("i32")
            || svar[1].contains("i64") || svar[1].contains("i128")
            || svar[1].contains("u8") || svar[1].contains("u16") || svar[1].contains("u32")
            || svar[1].contains("u64") || svar[1].contains("u128")
            || svar[1].contains("isize") || svar[1].contains("usize") {
                varsname = format!("{}{}: 0", varsname, svar[0]);
        }
        else if svar[1].contains("f32") {
            varsname = format!("{}{}: 0f32", varsname, svar[0]);
        }
        else if svar[1].contains("f64") {
            varsname = format!("{}{}: 0f64", varsname, svar[0]);
        }
        else if svar[1].contains("String") {
                varsname = format!("{}{}: String::from(\"null\")", varsname, svar[0]);
        }
        else if svar[1].contains("char") {
                varsname = format!("{}{}: '\0'", varsname, svar[0]);
        }
    }

    return Ident::from(varsname);
}

pub fn get_vars_name(vars : &Vec<&str>) -> Ident {
    let mut varsname = String::from("");

    for i in 0..vars.len() {
        if varsname.len() > 0 {
            varsname = format!("{}, ", varsname);
        }
        varsname = format!("{}{}", varsname, vars[i]);
    }

    return Ident::from(varsname);
}
