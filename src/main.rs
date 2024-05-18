use std::io;

static MAX_PRODUCT: usize = 5;

#[derive(Debug)]
enum ProductType {
    Food,
    Electronics,
    Clothing,
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
    quantity: i32,
    product_type: ProductType,
}

// Function to create a new product and add it to the store
fn create_product(store: &mut Vec<Product>, product: Product) -> &mut Vec<Product> {
    if store.len() < MAX_PRODUCT {
        println!("Product added successfully to the store {:?}", product);
        store.push(product);
        return store;
    } else {
        panic!("Store is full");
    }
}

fn main() {
    let mut store: Vec<Product> = Vec::new();

    println!("Let's create a new Product!");

    // Read the product name from the user
    println!("Enter the product name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("You entered: {}", name);

    // Read the product price from the user
    println!("Enter the product price: ");
    let mut price = String::new();
    io::stdin()
        .read_line(&mut price)
        .expect("Failed to read line");
    let price: f64 = price.trim().parse().expect("Please enter a number");
    println!("You entered: {}", price);

    // Read the product quantity from the user
    println!("Enter the product quantity: ");
    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read line");
    let quantity: i32 = quantity.trim().parse().expect("Please enter a number");
    println!("You entered: {}", quantity);

    // Read the product type from the user
    println!("Enter the product type (Food, Electronics, Clothing): ");
    let mut product_type = String::new();
    io::stdin()
        .read_line(&mut product_type)
        .expect("Failed to read line");

    // Validate the product type entered by the user
    if (!product_type.trim().eq("Food"))
        && (!product_type.trim().eq("Electronics"))
        && (!product_type.trim().eq("Clothing"))
    {
        io::stdin()
            .read_line(&mut product_type)
            .expect("Failed to read line... make sure you enter Food, Electronics, or Clothing");
    }

    // Convert the product type string to the corresponding enum value
    let product_type = match product_type.trim() {
        "Food" => ProductType::Food,
        "Electronics" => ProductType::Electronics,
        "Clothing" => ProductType::Clothing,
        _ => panic!("Invalid product type"),
    };

    // Create a new product instance
    let product = Product {
        name,
        price,
        quantity,
        product_type,
    };

    // Add the product to the store
    create_product(&mut store, product);

    // Print the products in the store
    for product in &store {
        println!(
            "Product: {}, Price: {}, Quantity: {}, Type: {:?}",
            product.name, product.price, product.quantity, product.product_type
        );
    }

    // Check if the store can accommodate more products
    if store.len() < MAX_PRODUCT {
        println!("add another product? (y/n)");
        let mut add_another = String::new();
        io::stdin()
            .read_line(&mut add_another)
            .expect("Failed to read line");
        if add_another.trim().eq("y") {
            // Recursive call to create another product
            main();
        } else {
            println!("Goodbye!");
        }
    } else {
        println!("Store is full");
    }
}
