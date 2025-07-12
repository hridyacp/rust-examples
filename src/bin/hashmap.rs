use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    category: String,
}

fn main() {
    let products = vec![
        Product { id: 1, name: "Laptop".to_string(), category: "Electronics".to_string() },
        Product { id: 2, name: "T-Shirt".to_string(), category: "Apparel".to_string() },
        Product { id: 3, name: "Keyboard".to_string(), category: "Electronics".to_string() },
        Product { id: 4, name: "Coffee Mug".to_string(), category: "Kitchenware".to_string() },
        Product { id: 5, name: "Jeans".to_string(), category: "Apparel".to_string() },
    ];

    // The HashMap will map a category to a vector of products in that category.
    let mut products_by_category: HashMap<String, Vec<Product>> = HashMap::new();

    // Iterate through the list of products. `for product in products` consumes the vector.
    for product in products {
        products_by_category
            .entry(product.category.clone()) //clone the category
            .or_default()          //create new one   
            .push(product);                   // Push the current product into that vec
    }

    for (category, product_list) in &products_by_category {
        println!("\nCategory: {}", category);
        for product in product_list {
            println!("  - {:?}", product);
        }
    }
}