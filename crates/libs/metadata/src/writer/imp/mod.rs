// mod strings;
// mod blobs;
mod tables;
// mod codes;
// mod file;

// use crate::imp::*;
use super::*;
// use strings::*;
// use blobs::*;
// use std::collections::*;
use crate::reader;
// use std::slice::*;
// use std::mem::*;
// use tables::*;
// use codes::*;

struct TypeDef<'a> {
    item: &'a Item,
}

struct TypeRef<'a> {
    is_value_type: bool,
    assembly: &'a str,
}

fn item_is_value_type(item: &Item) -> bool {
    match item {
        Item::Struct(_) => true,
        Item::Enum(_) => true,
    }
}

fn item_type_name(item: &Item) -> (&str, &str) {
    match item {
        Item::Struct(i) => (&i.namespace, &i.name),
        Item::Enum(i) => (&i.namespace, &i.name),
    }
}

fn type_type_refs<'a>(ty: &'a Type, defs:&'a BTreeMap::<(&'a str, &'a str), TypeDef<'a>>) -> Vec<(&'a str, &'a str)> {
    let mut refs = vec![];

    match ty {
        Type::TypeDef((namespace, name)) => {
            if !defs.contains_key(&(&namespace, &name)) {
                refs.push((namespace.as_str(), name.as_str()));
            }
        }
        _ => {
            // No type to reference
        }
    }

    refs
}

fn item_type_refs<'a>(item: &'a Item, defs:&'a BTreeMap::<(&'a str, &'a str), TypeDef<'a>>) -> Vec<(&'a str, &'a str)> { // TODO: impl Iterator<Item = (&'a str, &'a str)> + 'a {
    let mut refs = vec![];

    match item {
        Item::Struct(item) => {
            for field in &item.fields {
                refs.append(&mut type_type_refs(&field.ty, &defs));
            }
        }
        Item::Enum(_) => {
            // No type refs in an enum
        }
    }

    refs
}

// // TODO: return a Vec<(&str, &str)> for all types to reference
// fn type_to_ref<'a>(ty:&'a Type, defs:&BTreeMap::<(&'a str, &'a str), TypeDef>, refs: &'a mut BTreeMap::<(&'a str, &'a str), TypeRef<'a>>, references: &'a reader::Reader) {
//     match ty {
//         Type::TypeDef((namespace, name)) => {
//             if !defs.contains_key(&(&namespace, &name)) {
//                 refs.entry((&namespace, &name)).or_insert_with(|| {
//                     let def = references.get(reader::TypeName::new(&namespace, &name)).next().expect("Type not found");
//                     TypeRef { 
//                         is_value_type: references.type_def_is_value_type(def),
//                         assembly: references.type_def_module(def),
//                     }
//                 });
//             }
//         }
//         _ => {
//             // No type to reference
//         }
//     }
// }

pub fn write<P: AsRef<std::path::Path>>(path: P, references: &[P], items: &[Item]) {
    let references: Vec<reader::File> = references.iter().map(|path| reader::File::new(path).expect("Invalid winmd file")).collect();
    let references = reader::Reader::new(&references);

    let mut defs = BTreeMap::<(&str, &str), TypeDef>::new();
    let mut refs = BTreeMap::<(&str, &str), TypeRef>::new();

    for item in items {
        defs.insert(item_type_name(item), TypeDef { item });
    }

    for item in items {
        for (namespace, name) in item_type_refs(item, &defs) { 
            refs.entry((namespace, name)).or_insert_with(|| {
                let def = references.get(reader::TypeName::new(namespace, name)).next().expect("Type not found");
                TypeRef { 
                    is_value_type: references.type_def_is_value_type(def),
                    assembly: references.type_def_module(def),
                }
            });
        }
    }

    // for item in items {
    //     match item {
    //         Item::Struct(item) => {
    //             for field in &item.fields {
    //                 type_to_ref(&field.ty, &defs, &mut refs, &references);
    //             }
    //         }
    //         Item::Enum(_) => {
    //             // No type refs in an enum
    //         }
    //     }
    // }

    // tables.Module.push(Module { Name: tables.strings.insert(path.as_ref().file_name().expect("Missing file name").to_str().expect("Invalid file name")), Mvid: 1, ..Default::default() });
    // tables.TypeDef.push(TypeDef { TypeName: tables.strings.insert("<Module>"), ..Default::default() });
    // let mscorlib = tables.AssemblyRef.push2(AssemblyRef { MajorVersion: 4, Name: tables.strings.insert("mscorlib"), ..Default::default() });
    // let value_type = tables.TypeRef.push2(TypeRef { TypeName: tables.strings.insert("ValueType"), TypeNamespace: tables.strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });
    // let enum_type = tables.TypeRef.push2(TypeRef { TypeName: tables.strings.insert("Enum"), TypeNamespace: tables.strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });

    // for (index, item) in items.iter().enumerate() {
    //     let index = index + 1;
    //     match item {
    //         Item::Struct(s) => {
    //             tables.type_def_index.insert((&s.name.0, &s.name.1), (index as u32, true));
    //         }
    //         Item::Enum(e) => {
    //             tables.type_def_index.insert((&e.name.0, &e.name.1), (index as u32, true));
    //         }
    //     }
    // }

    // for item in items {
    //     match item {
    //         Item::Struct(s) => {
    //             for f in &s.fields {
    //                 tables.reference(&f.ty, &references);
    //             }
    //         }
    //         Item::Enum(e) => {}
    //     }
    // }

    // // TODO: first fill in the TypeRef table by walking the items and resolving type refs

    // // then walk the items and fill in the definitions

    // for item in items {
    //     match item {
    //         Item::Struct(s) => {
    //             let mut flags = TypeAttributes(0);
    //             flags.set_public();
    //             if s.winrt {
    //                 flags.set_winrt();
    //             }
    //             tables.TypeDef.push(TypeDef {
    //                 Flags: flags.0,
    //                 TypeNamespace: tables.strings.insert(&s.name.0),
    //                 TypeName: tables.strings.insert(&s.name.1),
    //                 Extends: TypeDefOrRef::TypeRef(value_type).encode(),
    //                 FieldList: tables.Field.len() as _,
    //                 MethodList: 0,
    //             });
    //             for f in &s.fields {
    //                 let mut flags = FieldAttributes(0);
    //                 flags.set_public();
    //                 let signature = tables.field_sig(&f.ty);
    //                 tables.Field.push(tables::Field { Flags: flags.0, Name: tables.strings.insert(&f.name), Signature: tables.blobs.insert(&signature) })
    //             }
    //         }
    //         Item::Enum(e) => {

    //         }
    //     }
    // }

    // file::write(path,  tables.into_streams());
}

pub struct Streams {
    pub tables: Vec<u8>,
    pub strings: Vec<u8>,
    pub blobs: Vec<u8>,
    pub guids: Vec<u8>,
}

impl Streams {
    pub fn len(&self) -> usize {
        self.tables.len() + self.guids.len() + self.strings.len() + self.blobs.len()
    }
}

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

