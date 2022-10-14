use super::*;
use crate::imp::*;

// TODO: call it Staging?
pub struct Tables<'a> {
    pub AssemblyRef: Vec<AssemblyRef>,
    pub ClassLayout: Vec<ClassLayout>,
    pub Constant: Vec<Constant>,
    pub CustomAttribute: Vec<CustomAttribute>,
    pub Field: Vec<Field>,
    pub GenericParam: Vec<GenericParam>,
    pub ImplMap: Vec<ImplMap>,
    pub InterfaceImpl: Vec<InterfaceImpl>,
    pub MemberRef: Vec<MemberRef>,
    pub MethodDef: Vec<MethodDef>,
    pub Module: Vec<Module>,
    pub ModuleRef: Vec<ModuleRef>,
    pub NestedClass: Vec<NestedClass>,
    pub Param: Vec<Param>,
    pub Property: Vec<Property>,
    pub TypeDef: Vec<TypeDef>,
    pub TypeRef: Vec<TypeRef>,
    pub TypeSpec: Vec<TypeSpec>,

    pub strings: Strings,
    pub blobs : Blobs,
    pub type_def_index : BTreeMap::<(&'a str, &'a str), (u32, bool)>,
    pub type_ref_index : BTreeMap::<(&'a str, &'a str), (u32, bool)>,
    pub references: &'a reader::Reader<'a>,
}

#[derive(Default)]
pub struct AssemblyRef {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: u32,
    pub PublicKeyOrToken: u32,
    pub Name: u32,
    pub Culture: u32,
    pub HashValue: u32,
}

#[derive(Default)]
pub struct ClassLayout {
    pub PackingSize: u16,
    pub ClassSize: u32,
    pub Parent: u32,
}

#[derive(Default)]
pub struct Constant {
    pub Type: u8,
    pub Parent: u32,
    pub Value: u32,
}

#[derive(Default)]
pub struct CustomAttribute {
    pub Parent: u32,
    pub Type: u32,
    pub Value: u32,
}

#[derive(Default)]
pub struct Field {
    pub Flags: u16,
    pub Name: u32,
    pub Signature: u32,
}

#[derive(Default)]
pub struct GenericParam {
    pub Number: u16,
    pub Flags: u16,
    pub Owner: u32,
    pub Name: u32,
}

#[derive(Default)]
pub struct ImplMap {
    pub MappingFlags: u16,
    pub MemberForwarded: u32,
    pub ImportName: u32,
    pub ImportScope: u32,
}

#[derive(Default)]
pub struct InterfaceImpl {
    pub Class: u32,
    pub Interface: u32,
}

#[derive(Default)]
pub struct MemberRef {
    pub Class: u32,
    pub Name: u32,
    pub Signature: u32,
}

#[derive(Default)]
pub struct MethodDef {
    pub RVA: u32,
    pub ImplFlags: u16,
    pub Flags: u16,
    pub Name: u32,
    pub Signature: u32,
    pub ParamList: u32,
}

#[derive(Default)]
pub struct Module {
    pub Generation: u16,
    pub Name: u32,
    pub Mvid: u32,
    pub EncId: u32,
    pub EncBaseId: u32,
}

#[derive(Default)]
pub struct ModuleRef {
    pub Name: u32,
}

#[derive(Default)]
pub struct NestedClass {
    pub NestedClass: u32,
    pub EnclosingClass: u32,
}

#[derive(Default)]
pub struct Param {
    pub Flags: u16,
    pub Sequence: u16,
    pub Name: u32,
}

#[derive(Default)]
pub struct Property {
    pub Flags: u16,
    pub Name: u32,
    pub Type: u32,
}

#[derive(Default)]
pub struct TypeDef {
    pub Flags: u32,
    pub TypeName: u32,
    pub TypeNamespace: u32,
    pub Extends: u32,
    pub FieldList: u32,
    pub MethodList: u32,
}

#[derive(Default)]
pub struct TypeRef {
    pub ResolutionScope: u32,
    pub TypeName: u32,
    pub TypeNamespace: u32,
}

#[derive(Default)]
pub struct TypeSpec {
    pub Signature: u32,
}

impl<'a> Tables<'a> {
    pub fn new(references: &'a reader::Reader) -> Self {
        Self {
        AssemblyRef: Default::default(),
        ClassLayout: Default::default(),
        Constant: Default::default(),
        CustomAttribute: Default::default(),
        Field: Default::default(),
        GenericParam: Default::default(),
        ImplMap: Default::default(),
        InterfaceImpl: Default::default(),
        MemberRef: Default::default(),
        MethodDef: Default::default(),
        Module: Default::default(),
        ModuleRef: Default::default(),
        NestedClass: Default::default(),
        Param: Default::default(),
        Property: Default::default(),
        TypeDef: Default::default(),
        TypeRef: Default::default(),
        TypeSpec: Default::default(),
    
        strings: Default::default(),
        blobs : Default::default(),
        type_def_index : Default::default(),
        type_ref_index : Default::default(),
        references,
        }
        
    }

    pub fn field_sig(&mut self, ty: &Type) -> Vec<u8> {
        let mut buffer = vec![0x6];
        match ty {
            Type::Void => buffer.push(0x01),
            Type::Bool => buffer.push(0x02),
            Type::Char => buffer.push(0x03),
            Type::I8 => buffer.push(0x04),
            Type::U8 => buffer.push(0x05),
            Type::I16 => buffer.push(0x06),
            Type::U16 => buffer.push(0x07),
            Type::I32 => buffer.push(0x08),
            Type::U32 => buffer.push(0x09),
            Type::I64 => buffer.push(0x0a),
            Type::U64 => buffer.push(0x0b),
            Type::F32 => buffer.push(0x0c),
            Type::F64 => buffer.push(0x0d),
            Type::ISize => buffer.push(0x18),
            Type::USize => buffer.push(0x19),
            Type::String => buffer.push(0x0e),
            Type::TypeDef((namespace, name)) => todo!(), // Build a TypeDefOrRef...
        }
        buffer
    }

    pub fn into_streams(self) -> Streams {
        let resolution_scope = coded_index_size(&[self.Module.len(), self.ModuleRef.len(), self.AssemblyRef.len(), self.TypeRef.len()]);
        let type_def_or_ref = coded_index_size(&[self.TypeDef.len(), self.TypeRef.len(), self.TypeSpec.len()]);
        let has_constant = coded_index_size(&[self.Field.len(), self.Param.len(), self.Property.len()]);

        let valid_tables: u64 = 1 << 0 | // Module 
        1 << 0x01 | // TypeRef
        1 << 0x02 | // TypeDef
        1 << 0x04 | // Field
        1 << 0x06 | // MethodDef
        1 << 0x08 | // Param
        1 << 0x09 | // InterfaceImpl
        1 << 0x0A | // MemberRef
        1 << 0x0B | // Constant
        1 << 0x0C | // CustomAttribute
        1 << 0x0F | // ClassLayout
        1 << 0x17 | // Property
        1 << 0x1A | // ModuleRef
        1 << 0x1B | // TypeSpec
        1 << 0x1C | // ImplMap
        1 << 0x23 | // AssemblyRef
        1 << 0x29 | // NestedClass
        1 << 0x2A; // GenericParam

        // The table stream header...

        let mut buffer = Vec::new();
        buffer.write_u32(0); // Reserved
        buffer.write_u8(2); // MajorVersion
        buffer.write_u8(0); // MinorVersion
        buffer.write_u8(0b111); // HeapSizes
        buffer.write_u8(0); // Reserved
        buffer.write_u64(valid_tables);
        buffer.write_u64(0); // Sorted

        // Followed by the length of each of the valid tables...

        buffer.write_u32(self.Module.len() as u32);
        buffer.write_u32(self.TypeRef.len() as u32);
        buffer.write_u32(self.TypeDef.len() as u32);
        buffer.write_u32(self.Field.len() as u32);
        buffer.write_u32(self.MethodDef.len() as u32);
        buffer.write_u32(self.Param.len() as u32);
        buffer.write_u32(self.InterfaceImpl.len() as u32);
        buffer.write_u32(self.MemberRef.len() as u32);
        buffer.write_u32(self.Constant.len() as u32);
        buffer.write_u32(self.CustomAttribute.len() as u32);
        buffer.write_u32(self.ClassLayout.len() as u32);
        buffer.write_u32(self.Property.len() as u32);
        buffer.write_u32(self.ModuleRef.len() as u32);
        buffer.write_u32(self.TypeSpec.len() as u32);
        buffer.write_u32(self.ImplMap.len() as u32);
        buffer.write_u32(self.AssemblyRef.len() as u32);
        buffer.write_u32(self.NestedClass.len() as u32);
        buffer.write_u32(self.GenericParam.len() as u32);

        // Followed by each table's rows...

        for x in self.Module {
            buffer.write_u16(x.Generation);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Mvid);
            buffer.write_u32(x.EncId);
            buffer.write_u32(x.EncBaseId);
        }

        for x in self.TypeRef {
            buffer.write_code(x.ResolutionScope, resolution_scope);
            buffer.write_u32(x.TypeName);
            buffer.write_u32(x.TypeNamespace);
        }

        for x in self.TypeDef {
            buffer.write_u32(x.Flags);
            buffer.write_u32(x.TypeName);
            buffer.write_u32(x.TypeNamespace);
            buffer.write_code(x.Extends, type_def_or_ref);
            buffer.write_index(x.FieldList, self.Field.len());
            buffer.write_index(x.MethodList, self.MethodDef.len());
        }

        for x in self.Field {
            buffer.write_u16(x.Flags);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Signature);
        }

        // for x in self.MethodDef {

        // }

        // for x in self.Param {}

        // for x in self.Constant {
        //     buffer.write_u16(x.Type);
        //     buffer.write_code(x.Parent, has_constant);
        //     buffer.write_u32(blobs.insert(&x.Value));
        // }

        for x in self.AssemblyRef {
            buffer.write_u16(x.MajorVersion);
            buffer.write_u16(x.MinorVersion);
            buffer.write_u16(x.BuildNumber);
            buffer.write_u16(x.RevisionNumber);
            buffer.write_u32(x.Flags);
            buffer.write_u32(x.PublicKeyOrToken);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Culture);
            buffer.write_u32(x.HashValue);
        }

        buffer.resize(round(buffer.len(), 4), 0);

        Streams { 
            tables: buffer,
            strings: self.strings.into_stream(),
            blobs: self.blobs.into_stream(),
            guids: vec![0; 16], // zero guid
        }
    }
}
