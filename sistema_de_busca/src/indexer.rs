// imports bibli e modulo
// 

use std::collections::HashMap;
use crate::product::Product;

// Define a estrutura Indexer, que contém um HashMap

pub struct Indexer {
    product_index: HashMap<String, Vec<u32>>, 
}

impl Indexer {
    pub fn new() -> Self {
        Indexer {
            product_index: HashMap::new(), //cria uma nova instanca
        }
    }

// O método index_product recebe um 
// Product e extrai os seguintes nome, categoria e marca


    pub fn index_product(&mut self, product: &Product) {
        let terms = vec![product.name.clone(), product.category.clone(), product.brand.clone()];

        for term in terms {
            self.product_index
                .entry(term.to_lowercase()) //maiscula
                .or_insert_with(Vec::new)
                .push(product.id); // o iid é add ao termo
        }
    }

//O método search busca um termo no HashMap

    pub fn search(&self, term: &str) -> Option<&Vec<u32>> {
        self.product_index.get(&term.to_lowercase())
    }
}
