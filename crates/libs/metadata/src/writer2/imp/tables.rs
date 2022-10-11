use super::*;

#[derive(Default)]
pub struct AssemblyRef {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: AssemblyFlags,
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
    pub Type: u16,
    pub Parent: HasConstant,
    pub Value: u32,
}

#[derive(Default)]
pub struct CustomAttribute {
    pub Parent: HasCustomAttribute,
    pub Type: CustomAttributeType,
    pub Value: u32,
}

#[derive(Default)]
pub struct Field {
    pub Flags: FieldAttributes,
    pub Name: u32,
    pub Signature: u32,
}

#[derive(Default)]
pub struct GenericParam {
    pub Number: u16,
    pub Flags: GenericParamAttributes,
    pub Owner: TypeOrMethodDef,
    pub Name: u32,
}

#[derive(Default)]
pub struct ImplMap {
    pub MappingFlags: PInvokeAttributes,
    pub MemberForwarded: MemberForwarded,
    pub ImportName: u32,
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
    pub Name: u32,
    pub Signature: u32,
}

#[derive(Default)]
pub struct MethodDef {
    pub RVA: u32,
    pub ImplFlags: MethodImplAttributes,
    pub Flags: MethodAttributes,
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
    pub Flags: ParamAttributes,
    pub Sequence: u16,
    pub Name: u32,
}

#[derive(Default)]
pub struct Property {
    pub Flags: PropertyAttributes,
    pub Name: u32,
    pub Type: u32,
}

#[derive(Default)]
pub struct TypeDef {
    pub Flags: TypeAttributes,
    pub TypeName: u32,
    pub TypeNamespace: u32,
    pub Extends: TypeDefOrRef,
    pub FieldList: u32,
    pub MethodList: u32,
}

#[derive(Default)]
pub struct TypeRef {
    pub ResolutionScope: ResolutionScope,
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
    pub TypeRef: BTreeMap<TypeName, TypeRef>,
    pub TypeDef: BTreeMap<TypeName, TypeDef>,
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
    pub fn into_stream(self) -> Vec<u8> {
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
        buffer.write(&0u32); // Reserved
        buffer.write(&2u8); // MajorVersion
        buffer.write(&0u8); // MinorVersion
        buffer.write(&0b111u8); // HeapSizes
        buffer.write(&0u8); // Reserved
        buffer.write(&valid_tables);
        buffer.write(&0u64); // Sorted

        // Followed by the length of each of the valid tables...

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

        // Followed by each table's rows...

        for x in self.Module {
            buffer.write(&x.Generation);
            buffer.write(&x.Name);
            buffer.write(&x.Mvid);
            buffer.write(&x.EncId);
            buffer.write(&x.EncBaseId);
        }

        for x in self.TypeRef {
            write_coded_index(&mut buffer, x.ResolutionScope.encode(), resolution_scope);
            buffer.write(&x.TypeName);
            buffer.write(&x.TypeNamespace);
        }

        for x in self.TypeDef {
            buffer.write(&x.Flags);
            buffer.write(&x.TypeName);
            buffer.write(&x.TypeNamespace);
            write_coded_index(&mut buffer, x.Extends.encode(), type_def_or_ref);
            write_index(&mut buffer, x.FieldList, self.Field.len());
            write_index(&mut buffer, x.MethodList, self.MethodDef.len());
        }

        for x in self.Field {
            buffer.write(&x.Flags);
            buffer.write(&x.Name);
            buffer.write(&x.Signature);
        }

        // for x in self.MethodDef {}

        // for x in self.Param {}

        for x in self.Constant {
            buffer.write(&x.Type);
            write_coded_index(&mut buffer, x.Parent.encode(), has_constant);
            buffer.write(&x.Value);
        }

        for x in self.AssemblyRef {
            buffer.write(&x.MajorVersion);
            buffer.write(&x.MinorVersion);
            buffer.write(&x.BuildNumber);
            buffer.write(&x.RevisionNumber);
            buffer.write(&x.Flags);
            buffer.write(&x.PublicKeyOrToken);
            buffer.write(&x.Name);
            buffer.write(&x.Culture);
            buffer.write(&x.HashValue);
        }

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

fn write_index(buffer: &mut Vec<u8>, index: u32, len: usize) {
    if len < (1 << 16) {
        buffer.write(&(index as u16 + 1))
    } else {
        buffer.write(&(index as u32 + 1))
    }
}

fn write_coded_index(buffer: &mut Vec<u8>, value: u32, size: usize) {
    if size == 2 {
        buffer.write(&(value as u16))
    } else {
        buffer.write(&(value as u32))
    }
}
