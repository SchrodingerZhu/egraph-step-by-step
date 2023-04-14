use std::collections::HashMap;
use arc_symbol::Symbol;

#[derive(Debug)]
pub struct ColumnarSchema<S, V> {
    columns: Vec<Symbol<S>>,
    sym_to_idx: HashMap<Symbol<S>, usize>,
    data: Vec<Vec<V>>,
}

impl<S, V> ColumnarSchema<S, V> {
    pub fn new(columns: Vec<Symbol<S>>) -> Self {
        let sym_to_idx = columns
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();
        Self {
            columns,
            sym_to_idx,
            data: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<V>) {
        debug_assert_eq!(row.len(), self.columns.len());
        for (i, v) in row.into_iter().enumerate() {
            self.data[i].push(v);
        }
    }

    pub fn get_column(&self, column: &Symbol<S>) -> Option<&[V]> {
        self.sym_to_idx.get(column).map(|&i| self.data[i].as_slice())
    }
}