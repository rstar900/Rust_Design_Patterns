// Inspiration taken from https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

// The idea of Builder Pattern is to have a build helper (Builder) class 
// which helps construct the actual class object whose constructor can be private

// It is like saving parameters in Builder class and then using that to build one or more 
// class objects. You can also modify the Builder object parameters incrementally to get different
// configurations for the same class and build multiple class objects.

// It might seem like Decorator pattern, but unlike decorator, where properties can be added
// after creation of the object. Here they need to be configured before construction of the actual object. 
// For more information, see https://dashitesh.medium.com/architecture-learnings-7-design-patterns-builder-vs-decorator-and-the-pizza-problem-50aac30f643d

// Use : Saves from creation of multiple constructor variants of the class

// Example : use PizzaBuilder to build Pizza objects

// Builder class
#[derive(Default)]
struct PizzaBuilder {
    size: u8,
    flavour: String,
    topping: String,
    price: u8
}
    
// Actual class
#[derive(Debug)]
struct Pizza {
    size: u8,
    flavour: String,
    topping: String,
    price: u8
}
    
// Actual class does not implement any public constructor
// But provides the default Builder object to contruct it
// see Default trait https://rust-unofficial.github.io/patterns/idioms/default.html
impl Pizza {
    fn builder() -> PizzaBuilder {
        PizzaBuilder::default()
    }
}

// There will be 2 types of Methods
impl PizzaBuilder {

    // One for setting parameters

    fn size(mut self, size: u8) -> PizzaBuilder {
        self.size = size;
        self
    }
    
    fn flavour(mut self, flavour: String) -> PizzaBuilder {
        self.flavour = flavour;
        self
    }
    
    fn topping(mut self, topping: String) -> PizzaBuilder {
        self.topping = topping;
        self
    }
    
    fn price(mut self, price: u8) -> PizzaBuilder {
        self.price = price;
        self
    }
    
    // The other one for constructing the actual class object (build())
    // Immutable borrow of self to avoid moving the PizzaBuilder object into it
    fn build(&self) -> Pizza {
        Pizza {
            size: self.size,
            flavour: self.flavour.clone(),
            topping: self.topping.clone(),
            price: self.price
        }
    }
}

fn main()
{

    // Setting the parameters / properties in the PizzaBuilder object
    // Setting it to be mutable as we want to modify it later
    let mut pizza_builder = Pizza::builder()
        .size(11)
        .flavour(String::from("Italiano"))
        .topping(String::from("Oregano"))
        .price(17);
        
    println!("---- Pizza Variant 1 ----\n");    
        
    // build pizza 1
    let pizza_1 = pizza_builder.build();
    
    println!("{:?}\n", pizza_1);
    
    println!("---- Pizza Variant 2 ----\n");
    
    // build pizza 2 by modifying properties in PizzaBuilder object a little
    pizza_builder = pizza_builder.size(9).topping(String::from("Chilli Flakes")).price(13);
    let pizza_2 = pizza_builder.build();
    
    println!("{:?}\n", pizza_2);
    
}
