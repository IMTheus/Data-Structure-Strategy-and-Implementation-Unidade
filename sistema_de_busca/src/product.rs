//  estrutura de um projeto 


#[derive(Debug, Clone)] // permite a cola no terminal 
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub brand: String,
    pub price: f32,
}

//  cria uma nova instância de Product.
impl Product {
    pub fn new(id: u32, name: String, category: String, brand: String, price: f32) -> Self {
        Product {
            id,
            name,
            category,
            brand,
            price,
        }
    }
}
