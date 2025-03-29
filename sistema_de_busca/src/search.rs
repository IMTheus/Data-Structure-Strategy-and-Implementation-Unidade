

// imports


use crate::indexer::Indexer;
use crate::product::Product;

// recebe o indexer e a plavra chave


pub fn search_products(indexer: &Indexer, products: &[Product], term: &str) {
    match indexer.search(term) {
        Some(product_ids) => {
            println!("Produtos encontrados para '{}':", term);
            for id in product_ids {
                if let Some(product) = products.iter().find(|p| p.id == *id) {
                    println!(
                        " - {} (Categoria: {}, Marca: {}, Preço: R$ {:.2})",
                        product.name, product.category, product.brand, product.price
                    );
                }
            }
        }
        None => {
            println!("Nenhum produto encontrado para '{}'", term);
        }
    }
}



