
//imports 

mod product;
mod indexer;
mod search;

use product::Product;
use indexer::Indexer;
use search::search_products;


//iniciar o indexer 
fn main() {
    let mut indexer = Indexer::new();

    let products = vec![
        Product { id: 1, name: "Smartphone X".to_string(), category: "eletrônicos".to_string(), brand: "TechBrand".to_string(), price: 1999.99 },
        Product { id: 2, name: "Notebook Pro".to_string(), category: "eletrônicos".to_string(), brand: "PowerComp".to_string(), price: 4999.99 },
        Product { id: 3, name: "Camiseta Esportiva".to_string(), category: "vestuário".to_string(), brand: "FitWear".to_string(), price: 79.90 },
    ];

    //indexa os produtos
    for product in &products {
        indexer.index_product(product);
    }

    //fazer as buscas
    search_products(&indexer, &products, "eletrônicos");
    search_products(&indexer, &products, "Smartphone X");
    search_products(&indexer, &products, "Camiseta");
}


