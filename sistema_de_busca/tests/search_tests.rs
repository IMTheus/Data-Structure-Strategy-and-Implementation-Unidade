// tests/search_tests.rs

use sistema_de_busca::{product::Product, indexer::Indexer};

#[test]
fn test_search_product_by_name() {
    let product = Product::new(1, "Smartphone X".to_string(), "Eletrônicos".to_string(), "BrandA".to_string(), 1999.99);
    let mut indexer = Indexer::new();
    indexer.index_product(&product);

    let result = indexer.search("smartphone x");
    assert!(result.is_some());
    assert_eq!(result.unwrap().as_ref(), vec![1]);  // Converte para referência
 // Modifique aqui para comparar corretamente
}

#[test]
fn test_search_product_not_found() {
    let product = Product::new(1, "Smartphone X".to_string(), "Eletrônicos".to_string(), "BrandA".to_string(), 1999.99);
    let mut indexer = Indexer::new();
    indexer.index_product(&product);

    let result = indexer.search("laptop y");
    assert!(result.is_none());
}

