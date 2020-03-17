#![allow(unused_variables)]
#![allow(unused_assignments)]
use std::process;
use text_io::read;
use blinmaker;
fn main() {
    println!("Blinmaker is starting up...");
    //Eggs
    println!("How many eggs do you have? ");
    let eggs_amount: i32 = read!();
    println!("You have {} eggs", eggs_amount);
    //Milk
    println!("How many milk do you have? ");
    let milk_amount: f32 = read!();
    println!("You have {} ml of milk", milk_amount);
    //Flour
    println!("How many flour do you have? ");
    let flour_amount: f32 = read!();
    println!("You have {} gr of flour", flour_amount);
    //Calculations
    if flour_amount<blinmaker::FLOUR_MIN || milk_amount<blinmaker::MILK_MIN || eggs_amount<blinmaker::EGGS_MIN{
        eprintln!("No blins today :(");
        process::exit(0x0100);
    }
    let blin_amount: f32 = blinmaker::find_blin_amount(flour_amount, milk_amount, eggs_amount);
    let eggs_required: i32;
    let flour_required: f32;
    let milk_required: f32;
    let materials: (f32, f32, i32) = blinmaker::find_materials_amount(flour_amount,milk_amount,eggs_amount);
    flour_required = materials.0;
    milk_required = materials.1;
    eggs_required = materials.2;
    println!("You will need {} eggs", eggs_required);
    println!("You will need {} grams of flour", flour_required);
    println!("You will need {} mililiters of milk", milk_required);
    println!("You can make {} blins", blin_amount as i32);
}
