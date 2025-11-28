use anyhow::{anyhow, Result};
use syn::{visit::Visit, File, Item, ItemFn, ItemStruct, ItemEnum, ItemImpl};

pub struct RustParser;

impl RustParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_file(&self, content: &str) -> Result<File> {
        syn::parse_file(content).map_err(|e| anyhow!("Failed to parse Rust file: {}", e))
    }

    pub fn list_functions(&self, content: &str) -> Result<Vec<String>> {
        let ast = self.parse_file(content)?;
        let mut visitor = FunctionVisitor::new();
        visitor.visit_file(&ast);
        Ok(visitor.functions)
    }

    pub fn list_structs(&self, content: &str) -> Result<Vec<String>> {
        let ast = self.parse_file(content)?;
        let mut visitor = StructVisitor::new();
        visitor.visit_file(&ast);
        Ok(visitor.structs)
    }

    pub fn list_enums(&self, content: &str) -> Result<Vec<String>> {
        let ast = self.parse_file(content)?;
        let mut visitor = EnumVisitor::new();
        visitor.visit_file(&ast);
        Ok(visitor.enums)
    }

    pub fn extract_function(&self, content: &str, function_name: &str) -> Result<String> {
        let ast = self.parse_file(content)?;
        
        for item in ast.items {
            if let Item::Fn(func) = item {
                if func.sig.ident == function_name {
                    return Ok(quote::quote!(#func).to_string());
                }
            } else if let Item::Impl(impl_block) = item {
                for impl_item in impl_block.items {
                    if let syn::ImplItem::Fn(method) = impl_item {
                        if method.sig.ident == function_name {
                            return Ok(quote::quote!(#method).to_string());
                        }
                    }
                }
            }
        }
        
        Err(anyhow!("Function '{}' not found", function_name))
    }

    pub fn find_item_location(&self, content: &str, item_name: &str) -> Result<(usize, usize)> {
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains(&format!("fn {}", item_name)) 
                || line.contains(&format!("struct {}", item_name))
                || line.contains(&format!("enum {}", item_name)) {
                return Ok((i + 1, i + 1));
            }
        }
        
        Err(anyhow!("Item '{}' not found", item_name))
    }
}

struct FunctionVisitor {
    functions: Vec<String>,
}

impl FunctionVisitor {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let name = node.sig.ident.to_string();
        let params: Vec<String> = node.sig.inputs.iter().map(|arg| {
            quote::quote!(#arg).to_string()
        }).collect();
        
        let return_type = match &node.sig.output {
            syn::ReturnType::Default => "()".to_string(),
            syn::ReturnType::Type(_, ty) => quote::quote!(#ty).to_string(),
        };
        
        self.functions.push(format!("fn {}({}) -> {}", name, params.join(", "), return_type));
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let type_name = quote::quote!(#(&node.self_ty)).to_string();
        
        for item in &node.items {
            if let syn::ImplItem::Fn(method) = item {
                let name = method.sig.ident.to_string();
                let params: Vec<String> = method.sig.inputs.iter().map(|arg| {
                    quote::quote!(#arg).to_string()
                }).collect();
                
                let return_type = match &method.sig.output {
                    syn::ReturnType::Default => "()".to_string(),
                    syn::ReturnType::Type(_, ty) => quote::quote!(#ty).to_string(),
                };
                
                self.functions.push(format!("impl {}::{}({}) -> {}", type_name, name, params.join(", "), return_type));
            }
        }
    }
}

struct StructVisitor {
    structs: Vec<String>,
}

impl StructVisitor {
    fn new() -> Self {
        Self {
            structs: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let name = node.ident.to_string();
        self.structs.push(name);
    }
}

struct EnumVisitor {
    enums: Vec<String>,
}

impl EnumVisitor {
    fn new() -> Self {
        Self {
            enums: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        let name = node.ident.to_string();
        let variants: Vec<String> = node.variants.iter().map(|v| v.ident.to_string()).collect();
        self.enums.push(format!("{} {{ {} }}", name, variants.join(", ")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_functions() {
        let code = r#"
            fn main() {
                println!("Hello");
            }
            
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;

        let parser = RustParser::new();
        let functions = parser.list_functions(code).unwrap();
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn test_extract_function() {
        let code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;

        let parser = RustParser::new();
        let func = parser.extract_function(code, "add").unwrap();
        assert!(func.contains("add"));
    }
}
