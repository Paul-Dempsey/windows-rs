use super::*;

#[derive(Default)]
pub struct Tables {
    pub module: Vec<Module>,
    pub type_ref: Vec<TypeRef>,
    pub type_def: Vec<TypeDef>,
    pub field: Vec<Field>,
    pub method_def: Vec<MethodDef>,
    pub param: Vec<Param>,
    pub interface_impl: Vec<InterfaceImpl>,
    pub member_ref: Vec<MemberRef>,
    pub constant: Vec<Constant>,
    pub custom_attribute: Vec<CustomAttribute>,
    pub class_layout: Vec<ClassLayout>,
    pub property: Vec<Property>,
    pub module_ref: Vec<ModuleRef>,
    pub type_spec: Vec<TypeSpec>,
    pub impl_map: Vec<ImplMap>,
    pub assembly_ref: Vec<AssemblyRef>,
    pub nested_class: Vec<NestedClass>,
    pub generic_param: Vec<GenericParam>,
}

pub struct AssemblyRef {
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub name: String,
}

pub struct ClassLayout {}

pub struct Constant {
    pub value: Value,
    pub(crate) parent_index: HasConstant,
}

pub struct CustomAttribute {}

pub struct Field {
    pub name: String,
    pub ty: Type,
    pub constant: Option<Value>,
}

pub struct GenericParam {}

pub struct ImplMap {}

pub struct InterfaceImpl {}

pub struct MemberRef {}

pub struct MethodDef {
    pub name: String,
    pub signature: Vec<u8>,
    pub param_list: Vec<Param>,
    pub(crate) param_index: usize,
}

pub struct ModuleRef {}

pub struct Module {
    pub name: String,
}

pub struct NestedClass {}

pub struct Param {
    pub flags: ParamAttributes,
    pub sequence: u16,
    pub name: String,
}

pub struct Property {}

pub struct TypeDef {
    pub flags: TypeAttributes,
    pub type_name: TypeName,
    pub extends: Option<TypeRef>,
    pub field_list: Vec<Field>,
    pub method_list: Vec<MethodDef>,

    pub(crate) field_index: usize,
    pub(crate) method_index: usize,
    pub(crate) extends_index: TypeDefOrRef,
}

pub struct TypeRef {
    pub type_name: TypeName,
    pub assembly_ref: AssemblyRef,

    pub(crate) assembly_index: ResolutionScope,
}

pub struct TypeSpec {}
