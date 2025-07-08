// fn main() {
//     let a = String::from("hello");
//     let b = a; // Ownership of `a` is moved to `b`
//     println!("{}", a); // ❌ Error: use of moved value `a`
//     println!("{}", b); // ✅ This works
// }

#[derive(Debug, Clone)]
struct Item {
    name: String,
    quantity: u32,
}

fn sell_item(item: Item) {
    println!("Selling {} ({} in stock)", item.name, item.quantity);
}
fn main() {
    let mut inventory: Vec<Item> = Vec::new();

    let sword = Item {
        name: String::from("Sword"),
        quantity: 1,
    };

    inventory.push(sword); // Cloning the sword to avoid ownership issues

    let potion = Item {
        name: String::from("Health Potion"),
        quantity: 5,
    };
    inventory.push(potion.clone());
    println!("Original potion still exists: {:?}", potion);
    sell_item(potion)
}
