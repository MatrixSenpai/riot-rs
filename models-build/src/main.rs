#![allow(unused, dead_code)]

use std::collections::HashMap;
use std::default::Default;
use oas3::{
    *, spec::{ObjectOrReference, SchemaType}
};
use syn::{
    *, __private::Span, punctuated::Punctuated,
    token::*,
};
use convert_case::*;

const REPLACE_TERMS: [(&'static str, &'static str); 13] = [
    ("12AssistStreakCount", "twelve_assist_streak_count"),
    ("type", "kind"),
    ("10", "ten"),
    ("0", "zero"),
    ("1", "one"),
    ("2", "two"),
    ("3", "three"),
    ("4", "four"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "eight"),
    ("9", "nine"),
];

fn main() -> std::result::Result<(), String> {
    let schema_directory = std::fs::read_dir("./models-build/swagger").unwrap();

    // let mut final_items = Vec::new();
    for item in schema_directory {
        let item = item.unwrap();
        let path = item.path();
        let dir_name = item.file_name().into_string().unwrap()
            .split_once("-swa").unwrap().0
            .to_string().to_case(Case::Snake);

        match std::fs::read_dir(format!("./models/{}", &dir_name)) {
            Err(_) => std::fs::create_dir_all(format!("./models/{}", &dir_name)).unwrap(),
            _ => ()
        };

        let spec = parse_spec(path)?;

        // for (n, f) in spec.iter() {
        //     let filename = n.to_case(Case::Snake);
        //     let formatted = prettyplease::unparse(f);
        //     std::fs::write(format!("./models/{}/{filename}.rs", &dir_name), formatted).unwrap();
        // }
        //
        // let mod_file_items = spec.into_iter()
        //     .map(|(n, _)| n)
        //     .collect();
        // let mod_file = parse_mod_file(mod_file_items);
        // let formatted = prettyplease::unparse(&mod_file);
        // std::fs::write(format!("./models/{}/mod.rs", &dir_name), formatted).unwrap();
        //
        // final_items.push(dir_name);
    }

    // let final_items_mod = parse_mod_file(final_items);
    // let formatted = prettyplease::unparse(&final_items_mod);
    // std::fs::write("./models/mod.rs", formatted).unwrap();

    Ok(())
}

fn parse_mod_file(filenames: Vec<String>) -> File {
    let items = filenames.iter()
        .map(|f| Ident::new(f.to_case(Case::Snake).as_str(), Span::call_site()))
        .map(|i| {
            Item::Mod(ItemMod {
                attrs: Vec::new(),
                vis: Visibility::Public(Pub::default()),
                unsafety: None,
                mod_token: Mod::default(),
                ident: i,
                content: None,
                semi: None,
            })
        })
        .collect::<Vec<_>>();

    File {
        shebang: None,
        attrs: Vec::new(),
        items,
    }
}

fn parse_spec(path: impl AsRef<std::path::Path>) -> std::result::Result<HashMap<String, File>, String> {
    let full_spec = openapi::from_path(path)
        .map_err(|e| format!("Error reading file: {e:?}"))?;

    let spec_models = full_spec.definitions;

    let mut spec_grouped: HashMap<String, Vec<openapi::Schema>> = HashMap::new();
    for (k, v) in spec_models.into_iter() {
        if k.eq(&"Error".to_string()) { continue }
        if !k.contains(".") {
            spec_grouped.insert(k, vec![v]);
            continue
        }

        let key = k.split_once(".").unwrap().0.to_string();

        match spec_grouped.contains_key(&key) {
            true => spec_grouped.get_mut(&key).unwrap().push(v),
            false => _ = spec_grouped.insert(key, vec![v]),
        }
    }

    Ok(
        spec_grouped.into_iter()
            .flat_map(|(f, g)| parse_file(&full_spec, f, g))
            .collect()
    )
}

fn parse_file(
    spec: &openapi::Spec,
    file_name: String,
    group: Vec<openapi::Schema>,
) -> Option<(String, File)> {
    let items = group.iter()
        .flat_map(|t| parse_objects(spec, &file_name, t))
        .collect::<Vec<_>>();

    // TODO
    let attrs = Vec::new();

    Some((file_name, File {
        attrs, items,
        shebang: None
    }))
}

fn parse_objects(
    spec: &openapi::Spec,
    file_name: &String,
    object: &openapi::Schema,
) -> Option<Item> {
    let required_fields = object.required.unwrap_or(Vec::new());

    // let ident = Ident::new(&object.title?, Span::call_site());
    // TODO
    // let mut attrs = Vec::new();

    None
    // let mapped_fields: Punctuated<Field, Comma> = object.properties.iter()
    //     .flat_map(|(n, o)| {
    //         parse_fields(spec, n, o, &required_fields)
    //     })
    //     .collect::<_>();
    // let fields = Fields::Named(FieldsNamed {
    //     named: mapped_fields, brace_token: Brace::default(),
    // });
    //
    // Some(Item::Struct(ItemStruct {
    //     attrs, ident, fields,
    //     vis: Visibility::Public(Pub::default()),
    //     semi_token: None,
    //     struct_token: Struct::default(),
    //     generics: Generics::default(),
    // }))
}

fn parse_fields(
    spec: &OpenApiV3Spec,
    item_name: &String,
    object: &ObjectOrReference<Schema>,
    required_fields: &Vec<String>,
) -> Option<Field> {
    let schema = object.resolve(spec).ok()?;

    let (ident, attr_val) = set_field_ident(schema.title.clone(), item_name);

    let attrs = Vec::new();

    let items_ref = schema.items.map(|s| s.resolve(spec).unwrap());
    let adds_ref = schema.additional_properties.map(|s| s.resolve(spec).unwrap());

    let segments = recursive_set_field_type(
        spec,
        required_fields.contains(&attr_val),
        schema.schema_type.eq(&Some(SchemaType::Array)),
        schema.title,
        schema.schema_type,
        items_ref,
        adds_ref,
    );

    let path = TypePath {
        qself: None,
        path: Path { leading_colon: None, segments }
    };

    Some(Field {
        attrs,
        ident: Some(ident),
        vis: Visibility::Public(Pub::default()),
        mutability: FieldMutability::None,
        colon_token: None,
        ty: syn::Type::Path(path)
    })
}

fn recursive_set_field_type(
    spec: &OpenApiV3Spec,
    is_required: bool,
    is_array: bool,
    field_title: Option<String>,
    field_type: Option<SchemaType>,
    items: Option<Schema>,
    add_items: Option<Schema>,
) -> Punctuated<PathSegment, PathSep> {

    if !is_required {
        let ident = Ident::new("Option", Span::call_site());

        let child_segments = recursive_set_field_type(
            spec,
            true,
            is_array,
            field_title,
            field_type,
            items,
            add_items
        );
        let mut args: Punctuated<GenericArgument, Comma> = Punctuated::new();
        args.push(GenericArgument::Type(syn::Type::Path(
            TypePath { qself: None, path: Path { leading_colon: None, segments: child_segments } }
        )));

        let generic_arguments = AngleBracketedGenericArguments {
            args,
            colon2_token: None, lt_token: Lt::default(), gt_token: Gt::default(),
        };

        let mut main_segment = Punctuated::new();
        main_segment.push(
            PathSegment { ident, arguments: PathArguments::AngleBracketed(generic_arguments) }
        );

        return main_segment
    }

    if is_array {
        let ident = Ident::new("Vec", Span::call_site());

        let child_segments = recursive_set_field_type(
            spec,
            is_required,
            false,
            field_title,
            field_type,
            items,
            add_items
        );
        let mut args: Punctuated<GenericArgument, Comma> = Punctuated::new();
        args.push(GenericArgument::Type(syn::Type::Path(
            TypePath { qself: None, path: Path { leading_colon: None, segments: child_segments } }
        )));

        let generic_arguments = AngleBracketedGenericArguments {
            args,
            colon2_token: None, lt_token: Lt::default(), gt_token: Gt::default(),
        };

        let mut main_segment = Punctuated::new();
        main_segment.push(
            PathSegment { ident, arguments: PathArguments::AngleBracketed(generic_arguments) }
        );

        return main_segment
    }

    let ident_type = recursive_get_type(spec, field_title, field_type, items, add_items);

    let mut segment = Punctuated::new();

    let ident = Ident::new(ident_type.as_str(), Span::call_site());
    segment.push(PathSegment {
        ident, arguments: PathArguments::None
    });

    segment
}

fn recursive_get_type(
    spec: &OpenApiV3Spec,
    field_title: Option<String>,
    field_type: Option<SchemaType>,
    items: Option<Schema>,
    add_items: Option<Schema>,
) -> String {
    match field_type {
        Some(SchemaType::Boolean) => return "bool".to_string(),
        Some(SchemaType::Integer) => return "i64".to_string(),
        Some(SchemaType::Number) => return "f64".to_string(),
        Some(SchemaType::String) => return "String".to_string(),
        Some(SchemaType::Object) if field_title.is_some() => return field_title.unwrap(),

        _ => (),
    };

    if let Some(items) = items {
        let inner_items = items.items.map(|i| i.resolve(spec).unwrap());
        let inner_add = items.additional_properties.map(|i| i.resolve(spec).unwrap());
        return recursive_get_type(spec, items.title, items.schema_type, inner_items, inner_add);
    }

    if let Some(items) = add_items {
        let inner_items = items.items.map(|i| i.resolve(spec).unwrap());
        let inner_add = items.additional_properties.map(|i| i.resolve(spec).unwrap());
        return recursive_get_type(spec, items.title, items.schema_type, inner_items, inner_add);
    }

    unreachable!()
}

fn set_field_ident(
    item_title: Option<String>,
    item_name: &String,
) -> (Ident, String) {
    let mut title = item_name.clone();
    for (n, r) in REPLACE_TERMS {
        if title.eq(n) { title = r.to_string() }
    }

    let cased = title.to_case(Case::Snake);

    let ident = Ident::new(cased.as_str(), Span::call_site());

    (ident, item_name.clone())
}