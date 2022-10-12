use super::*;

#[derive(Default)]
pub struct AssemblyRef {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: AssemblyFlags,
    pub PublicKeyOrToken: Vec<u8>,
    pub Name: String,
    pub Culture: String,
    pub HashValue: Vec<u8>,
}

#[derive(Default)]
pub struct ClassLayout {
    pub PackingSize: u16,
    pub ClassSize: u32,
    pub Parent: TypeName,
}

#[derive(Default)]
pub struct Constant {
    pub Type: Type,
    pub Parent: HasConstant,
    pub Value: Vec<u8>,
}

#[derive(Default)]
pub struct CustomAttribute {
    pub Parent: HasCustomAttribute,
    pub Type: CustomAttributeType,
    pub Value: Vec<u8>,
}

#[derive(Default)]
pub struct Field {
    pub Flags: FieldAttributes,
    pub Name: String,
    pub Signature: Type,
}

#[derive(Default)]
pub struct GenericParam {
    pub Number: u16,
    pub Flags: GenericParamAttributes,
    pub Owner: TypeOrMethodDef,
    pub Name: String,
}

#[derive(Default)]
pub struct ImplMap {
    pub MappingFlags: PInvokeAttributes,
    pub MemberForwarded: MemberForwarded,
    pub ImportName: String,
    pub ImportScope: u32,
}

#[derive(Default)]
pub struct InterfaceImpl {
    pub Class: u32,
    pub Interface: TypeDefOrRef,
}

#[derive(Default)]
pub struct MemberRef {
    pub Class: MemberRefParent,
    pub Name: String,
    pub Signature: Vec<u8>,
}

#[derive(Default)]
pub struct MethodDef {
    pub RVA: u32,
    pub ImplFlags: MethodImplAttributes,
    pub Flags: MethodAttributes,
    pub Name: String,
    pub Signature: Vec<u8>,
    pub ParamList: u32,
}

#[derive(Default)]
pub struct ModuleRef {
    pub Name: String,
}

#[derive(Default)]
pub struct Module {
    pub Name: String,
}

#[derive(Default)]
pub struct NestedClass {
    pub NestedClass: u32,
    pub EnclosingClass: u32,
}

#[derive(Default)]
pub struct Param {
    pub Flags: ParamAttributes,
    pub Sequence: u16,
    pub Name: String,
}

#[derive(Default)]
pub struct Property {
    pub Flags: PropertyAttributes,
    pub Name: String,
    pub Type: Type,
}

#[derive(Default)]
pub struct TypeDef {
    pub Flags: TypeAttributes,
    pub TypeName: String,
    pub TypeNamespace: String,
    pub Extends: TypeDefOrRef,
    pub FieldList: u32,
    pub MethodList: u32,
}

#[derive(Default)]
pub struct TypeRef {
    pub ResolutionScope: ResolutionScope,
    pub TypeName: String,
    pub TypeNamespace: String,
}

#[derive(Default)]
pub struct TypeSpec {
    pub Signature: Type,
}

// TODO: some of these need to be sorted by a primary key
#[derive(Default)]
pub struct Tables {
    pub Module: Vec<Module>,
    pub TypeRef: Vec< TypeRef>,
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
    pub fn into_stream(self, strings: &mut Strings, blobs: &mut Blobs) -> Vec<u8> {
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

        // The #~ stream header...

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
            buffer.write_u16(0); // Generation (reserved)
            buffer.write_u32(strings.insert(&x.Name));
            buffer.write_u32(0); // Mvid (zero guid for repeatable builds)
            buffer.write_u32(0); // EncId (reserved)
            buffer.write_u32(0); // EncBaseId (reserved)
        }

        for x in self.TypeRef {
            buffer.write_code(x.ResolutionScope.encode(), resolution_scope);
            buffer.write_u32(strings.insert(&x.TypeName));
            buffer.write_u32(strings.insert(&x.TypeNamespace));
        }

        for x in self.TypeDef {
            buffer.write_u32(x.Flags.0);
            buffer.write_u32(strings.insert(&x.TypeName));
            buffer.write_u32(strings.insert(&x.TypeNamespace));
            buffer.write_code(x.Extends.encode(), type_def_or_ref);
            buffer.write_index(x.FieldList, self.Field.len());
            buffer.write_index(x.MethodList, self.MethodDef.len());
        }

        for x in self.Field {
            buffer.write_u32(x.Flags.0);
            buffer.write_u32(strings.insert(&x.Name));
            buffer.write_u32(blobs.insert(&x.Signature));
        }

        // for x in self.MethodDef {}

        // for x in self.Param {}

        for x in self.Constant {
            buffer.write_u16(x.Type);
            buffer.write_code(x.Parent.encode(), has_constant);
            buffer.write_u32(blobs.insert(&x.Value));
        }

        for x in self.AssemblyRef {
            buffer.write_u8(x.MajorVersion);
            buffer.write_u8(x.MinorVersion);
            buffer.write_u8(x.BuildNumber);
            buffer.write_u8(x.RevisionNumber);
            buffer.write_16(x.Flags.0);
            buffer.write_u32(blobs.insert(&x.PublicKeyOrToken));
            buffer.write_u32(strings.insert(&x.Name));
            buffer.write_u32(strings.insert(&x.Culture));
            buffer.write_u32(blobs.insert(&x.HashValue));
        }

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

