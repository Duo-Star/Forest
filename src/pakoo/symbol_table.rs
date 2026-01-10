// symbol_table.rs
use std::collections::HashMap;

#[derive(Default)]
pub struct SymbolTable {
    name_to_id: HashMap<String, usize>,
    id_to_name: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    // 获取 ID，如果不存在则创建（用于解析新变量定义或前向引用）
    pub fn get_or_create_id(&mut self, name: &str) -> usize {
        if let Some(&id) = self.name_to_id.get(name) {
            id
        } else {
            let id = self.id_to_name.len();
            self.id_to_name.push(name.to_string());
            self.name_to_id.insert(name.to_string(), id);
            id
        }
    }

    // 查询 ID (用于检查是否存在)
    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.name_to_id.get(name).cloned()
    }

    pub fn get_name(&self, id: usize) -> Option<&str> {
        if id < self.id_to_name.len() {
            Some(&self.id_to_name[id])
        } else {
            None
        }
    }
}
