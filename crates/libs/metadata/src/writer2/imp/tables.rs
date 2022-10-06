use super::*;

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
    pub Signature:u32,
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
pub struct ModuleRef {
    pub Name: u32,
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


#[derive(Default)]
pub struct Tables {
    pub Module: Vec<Module>,
    pub TypeRef: Vec<TypeRef>,
    pub TypeDef: Vec<TypeDef>,
    pub Field: Vec<Field>,
    pub MethodDef: Vec<MethodDef>,
    pub Param: Vec<Param>,
    pub InterfaceImpl: Vec<InterfaceImpl>,
    pub MemberRef: Vec<MemberRef>,
    pub Constant: Vec<Constant>,
    pub CustomAttribute: Vec<CustomAttribute>,
    pub ClassLayout: Vec<ClassLayout>,
    pub Property: Vec<Property>,
    pub ModuleRef: Vec<ModuleRef>,
    pub TypeSpec: Vec<TypeSpec>,
    pub ImplMap: Vec<ImplMap>,
    pub AssemblyRef: Vec<AssemblyRef>,
    pub NestedClass: Vec<NestedClass>,
    pub GenericParam: Vec<GenericParam>,
}

impl Tables {
    pub fn into_stream(mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = Header::new();
        buffer.write(&header);

        buffer.write(&(self.Module.len() as u32)); 
        buffer.write(&(self.TypeRef.len() as u32));
        buffer.write(&(self.TypeDef.len() as u32));
        buffer.write(&(self.Field.len() as u32));
        buffer.write(&(self.MethodDef.len() as u32));
        buffer.write(&(self.Param.len() as u32));
        buffer.write(&(self.InterfaceImpl.len() as u32));
        buffer.write(&(self.MemberRef.len() as u32));
        buffer.write(&(self.Constant.len() as u32));
        buffer.write(&(self.CustomAttribute.len() as u32));
        buffer.write(&(self.ClassLayout.len() as u32));
        buffer.write(&(self.Property.len() as u32));
        buffer.write(&(self.ModuleRef.len() as u32));
        buffer.write(&(self.TypeSpec.len() as u32));
        buffer.write(&(self.ImplMap.len() as u32));
        buffer.write(&(self.AssemblyRef.len() as u32));
        buffer.write(&(self.NestedClass.len() as u32));
        buffer.write(&(self.GenericParam.len() as u32));

        for x in self.Module {
            buffer.write(&x.Generation);
            buffer.write(&x.Name);
            buffer.write(&x.Mvid);
            buffer.write(&x.EncId);
            buffer.write(&x.EncBaseId);
        }

        for x in self.TypeRef {
            buffer.write(&x.ResolutionScope);
            buffer.write(&x.TypeName);
            buffer.write(&x.TypeNamespace);
        }

        for x in self.TypeDef {
            buffer.write(&x.Flags);
            buffer.write(&x.TypeName);
            buffer.write(&x.TypeNamespace);
            buffer.write(&x.Extends);
            buffer.write(&x.FieldList);
            buffer.write(&x.MethodList);
        }

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}


#[repr(C)]
#[derive(Default)]
struct Header {
    reserved1: u32,
    major_version: u8,
    minor_version: u8,
    heap_sizes: u8,
    reserved2: u8,
    valid: u64,
    sorted: u64,
}

impl Header {
    fn new() -> Self {
        Self {
            major_version: 2,
            reserved2: 1,
            heap_sizes: 0b111, // 4 byte indexes
            valid: 1 << 0 |    // Module 
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
                   1 << 0x2A, // GenericParam
            ..Default::default()
        }
    }
}
