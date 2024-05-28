use std::io;
struct House {
    address: String,
    price: u32,
    is_for_sale: bool,
    is_for_rent: bool,
    rent_price: Option<u32>,
}

impl House {
    fn new(
        address: &str,
        price: u32,
        is_for_sale: bool,
        is_for_rent: bool,
        rent_price: Option<u32>,
    ) -> Self {
        House {
            address: address.to_string(),
            price,
            is_for_sale,
            is_for_rent,
            rent_price,
        }
    }


    fn display_details(&self) {
        if self.is_for_rent {
            match self.rent_price {
                Some(price) => println!("House at {} - Rent Price: {} KES", self.address, price),
                None => println!("House at {} - For Rent", self.address),
            }
        } else {
            println!("House at {} - Price: {} KES", self.address, self.price);
        }
    }
}

struct Buyer {
    name: String,
    budget: u32,
    location: String,
}

struct Renter {
    name: String,
    #[allow(dead_code)]
    rental_budget: u32,
    location: String,
}
impl Renter {
    fn new(name: &str, rental_budget: u32, location: &str) -> Self {
        Renter {
            name: name.to_string(),
            rental_budget,
            location: location.to_string(),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

fn main() {
    // Create some sample houses
    let house1 = House::new("2167 Adalberto Turnpike Machakos,Kenya", 240000, true, false,None);
    let house2 = House::new("337 Len Flat Homabay,Kenya", 68500, true, true,None);
    let house3 = House::new("7453 McClue Creek Kilifi,Kenya", 190000, true, true,None);
    let house4 = House::new("59907 Schroeder Street Garissa,Kenya", 58500, true, true,None);
    let house5 = House::new("59497 Corey Pine Wajir,Kenya", 43000, true, true,None);
    let house6 = House::new("26258 Konopelski Locks Kwale,Kenya", 29000, true, true,None);
    let house7 = House::new("1087 Lida Underpass Wajir South,Kenya", 67500, true, true,None);
    let house8 = House::new("187 Schimmel Cape Marsabit,Kenya", 22500, true, true,None);
    let house9 = House::new("877 Donnie Viaduct Turkana,Kenya", 34000, true, true,None);
    let house10 = House::new(" 56770 Keebler Crest Samburu,Kenya", 24200, true, true,None);
    let house11 = House::new("8648 Reichert Fords Oloitoktok,Kenya", 38900, true, true,None);
    let house12 = House::new("6225 Keven Shoals Kericho,Kenya", 28500, true, true,None);

    // Display details of available houses
    println!("Available Houses:");
    house1.display_details();
    house2.display_details();
    house3.display_details();
    house4.display_details();
    house5.display_details();
    house6.display_details();
    house7.display_details();
    house8.display_details();
    house9.display_details();
    house10.display_details();
    house11.display_details();
    house12.display_details();

    // Get buyer's personal information
    let buyer_name = get_user_input("Enter your name:");
    let buyer_budget: u32 = get_user_input("Enter your budget:").parse().expect("Invalid input");
    let buyer_location = get_user_input("Enter your location:");

    // Create a buyer
    let buyer = Buyer {
        name: buyer_name,
        budget: buyer_budget,
        location: buyer_location,
    };


    // Check if the buyer can afford a house and if any are for sale
    if house1.is_for_sale && house1.price <= buyer.budget {
        println!(
            "{} can buy the house at {} in {}!",
            buyer.name, house1.address, buyer.location
        );
    } else {
        println!(
            "{} cannot afford or the house is not for sale.",
            buyer.name
        );
    }

    // Get renter's personal information
    let renter_name = get_user_input("Enter your name:");
    let rental_budget: u32 = get_user_input("Enter your rental budget:")
        .parse()
        .expect("Invalid input");
    let renter_location = get_user_input("Enter your location:");

    // Create a renter
  
 let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

// Check if the buyer can afford a house and if any are for sale
if house2.is_for_sale && house2.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house2.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");



// Create a renter

 let _renter = Renter::new(&renter_name, rental_budget, &renter_location);
// Now renter is accessible
let _details = (_renter.name, house2.address, _renter.location);


 // Check if the buyer can afford a house and if any are for sale
 if house3.is_for_sale && house3.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house3.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house4.is_for_sale && house4.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house4.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house5.is_for_sale && house5.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house5.address, buyer.location
    );

     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house6.is_for_sale && house6.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house6.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house7.is_for_sale && house7.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house7.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house8.is_for_sale && house8.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house8.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house9.is_for_sale && house9.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house9.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house10.is_for_sale && house10.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house10.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house11.is_for_sale && house11.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house11.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }

} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

//Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);

 // Check if the buyer can afford a house and if any are for sale
 if house12.is_for_sale && house12.price <= buyer.budget {
    println!(
        "{} can buy the house at {} in {}!",
        buyer.name, house12.address, buyer.location
    );
     // Offer payment option
     println!("Would you like to pay the rent for this house?");
     let choice = get_user_input("Enter 'yes' to proceed or any other key to exit:");
     if choice == "yes" {
         println!("Rent payment successful! Enjoy your new home.");
     } else {
         println!("Thank you for considering. Have a great day!");
     }
     
} else {
    println!(
        "{} cannot afford or the house is not for sale.",
        buyer.name
    );
}

// Get renter's personal information
let renter_name = get_user_input("Enter your name:");
let rental_budget: u32 = get_user_input("Enter your rental budget:")
    .parse()
    .expect("Invalid input");
let renter_location = get_user_input("Enter your location:");

// Create a renter

let _renter = Renter::new(&renter_name, rental_budget, &renter_location);
}





