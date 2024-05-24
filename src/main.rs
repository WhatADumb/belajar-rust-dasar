mod module;
mod information;

use std::cell::RefCell;
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::{LinkedList, VecDeque};
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Deref;
use std::rc::Rc;

#[allow(unused_imports)]
use module::first::say_hello;

#[allow(unused_imports)]
use module::second::say_hello as say_hello_second;

#[test]
fn mod_model_test(){
    let hanami = module::model::User{
        name: String::from("Hanami Mizuka"),
        level: 103,
        role: String::from("Assassin")
    };

    hanami.greeting("Budiantok")
}

#[test]
fn mod_hello_test(){
    say_hello();
    say_hello_second();
}

#[test]
fn mod_info_test(){
    println!("{}", information::dev_status());
}

fn main() {
    println!("-----------------");
}

#[test]
fn main_test(){
    println!("Hello from Testing...");
}

// VARIABLE
#[test]
fn variable_test(){
    let name = "Mr. Firmansyah";
    // name = "Fadhil"; Error cause name is imutable
    println!("Hello {}", name);

    let mut name = "Fadhil";
    println!("Hello {}", name);

    name = "Mr. Fadhil";
    println!("Sorry... I mean {}", name);
}

#[test]
fn string_slice_test(){
    let sentence = "AD VICTORIANNNN";
    println!("{}", sentence);

    // sentence = 123;
    // println!("My number {}", sentence); Error cause var sentece initiated as &str
}

#[test]
fn shadowing_test(){
    // Not recomended, causing hard to read a code and maintaining

    let index = 01;
    println!("INDEX: {}", index);

    let index = "Alpha";
    println!("INDEX: {}", index);
}

//VARIABLE SCALAR
#[test]
fn explicit_implicit_test(){
    // Implicit
    let number = 200;
    println!("Number: {}", number);

    // Explicit
    let number: i32 = 199;
    println!("Number: {}", number);
}

#[test]
fn number_test(){
    /*
        Integer
        8 bit ==> (Unsigned u8 // Integer i8)
        16 bit ==> (Unsigned u16 // Integer i16)
        32 bit ==> "Default implicit" (Unsigned u32 // Integer i32)
        64 bit ==> (Unsigned u64 // Integer i64)
        128 bit ==> (Unsigned u128 // Integer i128)

        Float
        32 bit ==> (Unsigned f32 // Integer f32)
        64 bit ==> "Default implicit" (Unsigned f64 // Integer f64)

        Architecture Size
        32/64 bit on your device ==> (Unsigned usize // Integer isize)
     */
    let number_a = 100;
    let number_b = 10.55;

    println!("Number A: {}\nNumber: {}", number_a, number_b);
}

#[test]
fn number_conversion_test(){
    let number: i8 = 9;
    let number =  number as i16;
    let number = number as i32;
    let number = number as i64;
    println!("My Number: {}", number);

    let number: i128 = 12435761023491;
    let number = number as i8;
    println!("My Number: {}", number);
}

#[test]
fn numeric_operation_test(){
    let a = 10;
    let b = 10;
    println!("Value a: {}, Value b: {}", a, b);

    let c = a + b;
    println!("Penjumlahan: {}", c);
    let d = a - b;
    println!("Pengurangan: {}", d);
    let e = a * b;
    println!("Perkalian: {}", e);
    let f = a / b;
    println!("Pembagian: {}", f);
    let g = a % b;
    println!("Modulus (sisa bagi): {}", g);
}

#[test]
fn augmented_assignment_test(){
    /*
        variable x
        x = x + n ==> x += n
        x = x - n ==> x -= n
        x = x * n ==> x *= n
        x = x / n ==> x /= n
        x = x % n ==> x %= n
     */
    let mut a = 100;

    a *= 5;
    println!("{a}");
}

#[test]
fn boolean_test(){
    let is_alive: bool = true;
    println!("Your status alive? {}", is_alive);
}

#[test]
fn comparison_test(){
    /*
        x > y
        x < y
        x >= y
        x <= y
        x == y
        x != y
     */
    let x = 10;
    let y = 21;
    let result = x > y;
    println!("X = {} & Y = {}, result will be {}", x, y, result);
}

#[test]
fn boolean_operator_test(){
    let presensi = 75;
    let nilai_akhir = 80;

    let tingkat_presensi = presensi >= 75;
    let tingkat_nilai_akhir = nilai_akhir >= 80;


    /*
        1 && 1 = true
        1 && 0 = false
        0 && 1 = false
        0 && 0 = false

        1 || 1 = true
        1 || 0 = true
        0 || 1 = true
        0 || 0 = false

        !1 = false
        !0 = true
     */
    let lulus = tingkat_nilai_akhir && tingkat_presensi;
    println!("Apakah anda lulus?: {}", lulus);
}


#[test]
fn char_test(){
    let a = 'A';
    let s: char = 'S';
    let u = 'U';

    print!("{}{}{}\n", a,s,u);
}

//VARIABLE COMPOUND
#[test]
fn tuple_test(){
    let datas: (i32, char, bool) = (123, 'C', false);
    println!("{:?}", datas);

    let number = datas.0;
    let character =datas.1;
    let status = datas.2;
    println!("My Number = {}, My Character = {}, My Status = {}", number, character, status);

    let another_datas: (i8, bool, String) = (8, true, String::from("World")) ;
    let (score, _, text) = another_datas;
    println!("{} & {}", score, text);
    
    let mut data: (i32, char, bool) = (123, 'C', false);
    data.0 = 100;
    data.1 = 'A';
    data.2 = true;
    println!("{:?}", data);
}

#[test]
fn unit_test(){
    let single = main();
    println!("{:?}", single);

    let unit: () = ();
    println!("{:?}", unit);
}

#[test]
fn array_test(){
    let array_1 = [1,2,3,4,5];
    println!("{:?}", array_1);
    println!("{}", array_1[0]);
    println!("{}", array_1[array_1.len() - 1]);

    let mut array_2: [i8; 3] = [3,2,1];
    println!("{:?}", array_2);
    println!("{}", array_2[0]);
    println!("{}", array_1[array_1.len() - 2]);

    array_2[0] = -3;
    array_2[array_2.len() - 1] = -1;
    println!("{:?}", array_2);

    let arrays: [[i16; 3]; 2] = 
    [
        [10, 20, 30],
        [40, 50, 60]
    ];
    println!("{:?}", arrays);
    println!("{:?}", arrays[0]);
    println!("{}, {}, {}", arrays[0][0], arrays[0][1], arrays[0][2]);
    println!("{:?}", arrays[1]);
    println!("{}, {}, {}", arrays[1][0], arrays[1][1], arrays[1][2]);
}


#[allow(dead_code)]
const MAX_VALUE: i32 = 100;
#[test]
fn constant_test(){
    const MIN_VALUE: i8 = 1;
    println!("MAXIMUM VALUE = {} AND MINIMUM VALUE = {}", MAX_VALUE, MIN_VALUE);
}

#[test]
fn variable_scope_test(){
    let outer = "Hello";
    {
        println!("{}", outer);

        let inner = "World";
        println!("{}", inner);
    }

    // println!("{}", inner); Causing variable scope of variable inner
}

#[allow(dead_code)]
fn function_a(){
    let a = 12;
    let b = String::from("Hello");
    println!("{} {}", a, b);
}

#[allow(dead_code)]
fn function_b(){
    let a = 21;
    let b = String::from("World");
    println!("{} {}", a, b);
}

#[test]
fn heap_stack_test(){
    function_a();
    function_b();
}

#[test]
fn string_test(){
    let text = "Hello World!!!";
    println!("{}", text);

    let name = "    BITCOIN     ";
    println!("{}", name);
    println!("{}", name.trim());

    let mut sample_world = "ALPHA";
    println!("ALIAS: {}", sample_world);
    sample_world = "BETA";
    println!("ALIAS: {}", sample_world);

    let my_word = String::from("HELLO");
    println!("{}", my_word);

    let mut hello_world = String::from("Hello");
    println!("{}", hello_world);
    hello_world.push_str(" World!!!");
    println!("{}", hello_world);

    let morning_world = hello_world.replace("Hello", "Morning");
    println!("{}", morning_world);
}

#[test]
fn string_and_slice_test(){
    let hello = "Hello";
    println!("{}", hello);

    let world = String::from("World");
    println!("{}", world);
}

#[test]
fn ownership_rules(){
    // Variable a cant be access in this line
    let a = 10;
    {
        //In this scope variable b can be access
        let b = 21;
        println!("{}", b);
    } //Variable b out of scope and will be drop
    //Variable a get accessed
    println!("{}", a);
}   //Variable a out of scope and will be drop

#[test]
fn copy_data_stack(){
    let a = 110;
    let b = a;

    println!("a = {} & b = {}", a, b);
}

#[test]
fn clone_change_ownership_heap(){
    // Ownerhship data hello world move to variable another_value
    let my_value = String::from("Hello World");
    let another_value = my_value;
    println!("{}", another_value);
    // println!("{}", my_value); Ownership already borrowed

    let task = String::from("Morning......");
    let another_task = task.clone();
    println!("{}", another_task);
    println!("{}", task);
}

#[test]
fn if_expression_test(){
    let score = 1;
    let result: &str;

    if score >= 8{
        result = "Good Job";
    }else if score >= 5 {
        result = "Well Done";
    }else if score >= 3 {
        result = "Keep it up";
    }else {
        result = "You're fucking donkey";
    }

    println!("{}", result);
}

#[test]
fn let_expression_test(){
    let score = 7;
    let result: &str = if score >= 8{
        "Good Job"
    }else if score >= 5 {
        "Well Done"
    }else if score >= 3 {
        "Keep it up"
    }else {
        "You're fucking donkey"
    };

    println!("{}", result);
}

#[test]
fn loop_condition_test(){
    let mut value = 0;

    loop {
        value += 1;

        if value > 10{
            break;
        }

        if value % 2 == 1 {
            continue;
        }

        println!("Counter = {}", value);
    }
}

#[test]
fn let_loop_condition_test(){
    let mut score = 1;
    let counter: i32 = loop {
        score += 1;

        if score > 10{
            break score * 5
        }
    };
    println!("{}", counter);
}

#[test]
fn label_loop_condition_test(){
    let mut i = 0;
    'parent: loop {
        i += 1;

        let mut j = 0;
        loop {
            if i > 10 {
                break 'parent;
            }

            j += 1;
            if j > 10{
                break;
            }

            println!("{} + {} = {}", i, j, i+j);
        }
        println!();
    }
}

#[test]
fn while_condition_test(){
    let mut counter = 0;

    while counter <= 10 {
        
        if counter % 2 == 1 {
            println!("Counter = {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_while_test(){
    let characters: [char; 5] = ['A', 'B', 'C', 'D', 'E'];

    let mut index = 0;
    while index < characters.len() {
        println!("Character = {}", characters[index]);

        index += 1;
    }
}

#[test]
fn array_for_test(){
    let characters: [char; 5] = ['A', 'B', 'C', 'D', 'E'];

    for data in characters{
        println!("Data = {}", data);
    }
}

#[test]
fn range_exclusive_test(){
    let my_range = 1..10;
    
    println!("Start Range {}", my_range.start);
    println!("End Range {}", my_range.end);
    
    for i in my_range{
        print!("{} ", i);
    }
}

#[test]
fn range_inclusive_test(){
    let my_range = 1..=10;
    
    println!("Start Range {}", my_range.start());
    println!("End Range {}", my_range.end());
    
    for i in my_range{
        print!("{} ", i);
    }
}   

#[allow(dead_code)]
fn greet_function(){
    println!("Hello There...");
}

#[allow(dead_code)]
fn greet_function_name(name: String){
    println!("Have a good day {}", name);
}

#[allow(dead_code)]
fn greet_function_value(name: String) -> String{
    format!("Have a good day {}", name)
}

#[allow(dead_code)]
fn greet_function_recursive(n: i32){
    if n == 0 {
        return;
    }else {
        println!("Hello World!!!");
    }

    greet_function_recursive(n - 1);
}

#[allow(dead_code)]
fn factorial(n: u32) -> u32{
    if n <= 1{
        return 1;
    }
    n * factorial(n - 1)
}

#[test]
fn function_test(){
    greet_function();
    greet_function_name("Fadhil Firmansyah".to_string());
    println!("{}", greet_function_value(String::from("Fadhil Firmansyah")));
    greet_function_recursive(5);
    println!("Factorial of 5 is {}", factorial(5));
}

#[allow(dead_code)]
fn calling_in_heap(name: String){
    println!("Heap: {}", name);
}

#[allow(dead_code)]
fn calling_in_stack(number: i32){
    println!("Stack: {}", number);
}

#[allow(dead_code)]
fn full_name_one(first: String, last: String) -> String{
    format!("{} {}", first, last)
}

#[allow(dead_code)]
fn full_name_two(first: String, last: String) -> (String, String, String){
    let name = format!("{} {}", first, last);
    (first, last, name)
}

#[test]
fn ownerhship_function(){
    let number = 100;
    calling_in_stack(number);
    println!("(RAW) {}", number);

    let name = String::from("Beta");
    calling_in_heap(name);
    // println!("(RAW): {}", name); borrow of moved value: `name`
}

#[test]
fn take_over_ownership(){
    let first_name = String::from("Fadhil");
    let last_name = String::from("Firmansyah");

    let name = full_name_one(first_name, last_name);
    calling_in_heap(name);
    // println!("{}", first_name); borrow of moved value: `first_name`
    // println!("{}", last_name); borrow of moved value: `last_name`
}

#[test]
fn return_back_ownership(){
    let first_name = String::from("Fadhil");
    let last_name = String::from("Firmansyah");

    let (first_name, last_name, name) = full_name_two(first_name, last_name);

    calling_in_heap(name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[allow(dead_code)]
fn full_name_three(first: &String, last: &String) -> String{
    format!("{} {}", first, last)
}

#[test]
fn reference_ownership_test(){
    let first_name = String::from("Fadhil");
    let last_name = String::from("Firmansyah");

    let name = full_name_three(&first_name, &last_name);

    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[allow(dead_code)]
fn alter_value(val: &mut String){
    val.push_str(" World");
}

#[test]
fn borrowing_test(){
    let mut value = String::from("Hello");

    alter_value(&mut value);
    alter_value(&mut value);
    alter_value(&mut value);
    alter_value(&mut value);

    println!("{}", value);
}

// #[allow(dead_code)]
// fn return_it(first: &String, last: &String) -> &String{
    // format!("{} {}", first, last)
// } Error causing of Dangling References

#[test]
fn slicing_array_test(){
    let array = [1,2,3,4,5,6,7,8,9,10];

    let slicing1 = &array[0..5];
    println!("{:?}", slicing1);

    let slicing2 = &array[5..10];
    println!("{:?}", slicing2);

    let slicing3 = &array[..5];
    println!("{:?}", slicing3);

    let slicing4 = &array[5..];
    println!("{:?}", slicing4);

    let slicing5: &[i32] = &array[..];
    println!("{:?}", slicing5);
}

#[test]
fn slicing_string_test(){
    let username = String::from("Fadhil Firmansyah");

    let fadhil: &str = &username[..6];
    println!("{}", fadhil);

    let firmansyah = "Firmansyah";
    println!("{}", firmansyah)
}

struct Person{
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u16
}

#[allow(dead_code)]
impl Person {
    fn greeting(&self, name: &str){
        println!("Hello {}, My name is {}", self.last_name, name);
    }
}

#[allow(dead_code)]
struct GeoPoint(i32, i32);

#[allow(dead_code)]
impl GeoPoint {
    fn new(long: i32, lat: i32) -> GeoPoint{
        GeoPoint(long, lat)
    }

    fn home() -> GeoPoint{
        GeoPoint(0, 0)
    }

    fn print_location(&self){
        println!("Longtitude = {} & Latitude = {}", self.0, self.1);
    }
}

#[allow(dead_code)]
struct NothingButEmpty;

#[allow(dead_code)]
fn print_person(person: &Person){
    println!("First Name = {}", person.first_name);
    println!("Middle Name = {}", person.middle_name);
    println!("Last Name = {}", person.last_name);
    println!("Age = {}", person.age);
}

#[test]
fn struct_test(){
    let fadhil = Person{
        age: 20,
        first_name: String::from("Fadhil"),
        middle_name: String::from(""),
        last_name: String::from("Firmansyah")
    };

    println!("{}", fadhil.first_name);
    println!("{}", fadhil.middle_name);
    println!("{}", fadhil.last_name);
    println!("{}", fadhil.age);
}

#[test]
fn struct_function_test(){
    let fadhil = Person{
        age: 20,
        first_name: String::from("Fadhil"),
        middle_name: String::from(""),
        last_name: String::from("Firmansyah")
    };

    print_person(&fadhil);
}

#[test]
fn struct_init_shorthand_test(){
    let first_name = String::from("Fadhil");

    let fadhil = Person{
        age: 20,
        first_name,
        middle_name: String::from(""),
        last_name: String::from("Firmansyah")
    };

    print_person(&fadhil);
}

#[test]
fn struct_update_test(){
    let fadhil = Person{
        age: 20,
        first_name: String::from("Fadhil"),
        middle_name: String::from(""),
        last_name: String::from("Firmansyah")
    };

    let rangga = Person{..fadhil};

    print_person(&rangga);
}

#[test]
fn struct_partial_update_test(){
    let fadhil = Person{
        age: 20,
        first_name: String::from("Fadhil"),
        middle_name: String::from(""),
        last_name: String::from("Firmansyah")
    };

    //To Evade memory previous owner on heap
    let rangga = Person{
        first_name: String::from("Rangga"),
        middle_name: String::from("Fadhil"),
        last_name: String::from("Priyatama"),
        ..fadhil
    };

    print_person(&rangga);
}

#[test]
fn struct_tuple_test(){
    let geo_point = GeoPoint(212312312, -1423213123);
    
    println!("Longtitude = {}", geo_point.0);
    println!("Longtitude = {}", geo_point.1);
}

#[test]
fn struct_unit_test(){
    let _nothing = NothingButEmpty;
}

#[test]
fn method_test(){
    let person = Person{
        first_name: String::from("Rangga"),
        middle_name: String::from("Fadhil"),
        last_name: String::from("Priyatama"),
        age: 21
    };

    person.greeting("Fadhil");
}

#[test]
fn associated_function_test(){
    let school = GeoPoint::new(12312312, 431414321);
    println!("Longtitude = {}", school.0);
    println!("Latitude = {}", school.1);

    let home = GeoPoint::home();
    home.print_location();
}

#[allow(dead_code)]
enum Rarity{
    Common, Epic, Legend
}

#[allow(dead_code)]
enum Payment{
    Transfer(u32),
    VirtualAccount(String, u32),
    Cash(String)
}

#[allow(dead_code)]
impl Payment {
    fn pay(&self, ammount: u32){
        println!("You're paying with ammount {}", ammount);
    }
}

#[test]
fn enum_test(){
    let _tier_list = Rarity::Legend;
}

#[test]
fn enum_data_test(){
    let _va = Payment::VirtualAccount(String::from("Bank ABC"), 12300000);
}

#[test]
fn enum_method_test(){
    let cash = Payment::Cash(String::from("333333333333"));
    cash.pay(1233333333);
}

#[test]
fn match_enum_test(){
    let my_skin = Rarity::Legend;

    match my_skin {
        Rarity::Common => println!("Your skin is good"),
        Rarity::Epic => println!("Your skin is cool dude"),
        Rarity::Legend => println!("Damn, your skin so slick mannnn")
    }
}

#[test]
fn match_destruct_enum_test(){
    let ojk = Payment::VirtualAccount(String::from("Bank Jateng"), 10000000);

    match ojk{
        Payment::Transfer(ammount) =>{
            println!("You're paying with ammount Rp {}", ammount)
        },
        Payment::Cash(money) => {
            println!("{} is ammount that you're pay for this bussiness", money)
        },
        Payment::VirtualAccount(from, ammount) => {
            println!("You're paying with ammount {} at {} virtul account", ammount, from)
        }
    }
}

#[test]
fn match_value_test(){
    let username = "Gundala";

    match username {
        "Godam" => println!("Kinda look alike Superman"),
        "Gundala" => println!("Kinda look alike with Volt"),
        heroes => println!("{} is something else", heroes)
    }
}

#[test]
fn match_multiple_value_test(){
    let username = "Gundala";

    match username {
        "Godam" | "Gundala" => println!("Iconic heros from Indonesia"),
        heroes => println!("{} is something else", heroes)
    }
}
#[test]
fn match_range_test(){
    let score = 100;

    match score {
        71..=100 => println!("You're amazing"),
        41..=70 => println!("Keep it up kiddo"),
        0..=40 => println!("Everything start from zero, right???"),
        invalid => println!("TF is {}", invalid)
    }
}

#[test]
fn match_destruct_struct_test_1(){
    let home = GeoPoint::home();
    match home {
        GeoPoint(0, 0) => println!("You're at home"),
        GeoPoint(long, 0) => println!("Longtitude = {}", long),
        GeoPoint(0, lat) => println!("Latitude = {}", lat),
        GeoPoint(long, lat) => println!("Longtitude = {}\nLatitude = {}", long, lat)
    }
}

#[test]
fn match_destruct_struct_test_2(){
    let uno = Person{
        first_name: String::from("Hybrid"),
        middle_name: String::from("UNO"),
        last_name: String::from("Daka"),
        age: 100
    };

    match uno{
        Person{first_name, last_name, middle_name, ..} => {
            println!("Hello {} {} aka {}", first_name, last_name, middle_name);
        }
    }
}

#[test]
fn match_ignoring(){
    let loc = GeoPoint::new(123, -321);
    match loc {
        GeoPoint(_, lat) => println!("Wellll, you're latitude seems at {}", lat)
    }
}

#[test]
fn match_ignoring_range_test(){
    let score = 100;
    match score {
        50..=100 => println!("Welcome at the half side Lad"),
        _ => println!("STFU, GTFO")
    }
}

#[test]
fn match_expression_test(){
    let rank = 10;
    let output = match rank {
        1 => "Perfect",
        _ => "TF you are???"
    };

    println!("{}", output);
}

#[allow(dead_code)]
type NIK = String;
#[allow(dead_code)]
type Age = u16;

#[allow(dead_code)]
struct Cutomer{
    id: NIK,
    name: String,
    age: Age
}

#[test]
fn alias_test(){
    let ahha = Cutomer{
        id: String::from("A31231B"),
        name: String::from("Ahsiaaaap"),
        age: 32
    };

    println!("{} {} {}", ahha.name, ahha.id, ahha.age);
}

trait GreetingHello {
    fn greeting(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
    fn say_hello(&self) -> String{
        String::from("Hello everyone")
    }
}

impl GreetingHello for Person{
    fn greeting(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}",name ,self.first_name)
    }
}

trait FarewellHello{
    fn farewall(&self) -> String;
    fn farewall_to(&self, name: &str) -> String;
}

impl FarewellHello for Person{
    fn farewall(&self) -> String {
        format!("Good Bye... {}", self.last_name)
    }

    fn farewall_to(&self, name: &str) -> String {
        format!("Good Bye {}, hope we meet again soon {}", name, self.last_name)
    }
}

#[allow(dead_code)]
fn greeting_person(trait_value: &impl GreetingHello){
    println!("{}", trait_value.greeting());
}

#[allow(dead_code)]
fn wave_person(trait_value: &(impl GreetingHello + FarewellHello)){
    println!("{}", trait_value.greeting());
    println!("{}", trait_value.farewall());
}

#[test]
fn trait_test(){
    let someone = Person{
        first_name: String::from("Lucian"),
        middle_name: String::from(""),
        last_name: String::from("High"),
        age: 34
    };

    println!("{}", someone.say_hello());
    greeting_person(&someone);
    wave_person(&someone);
}

struct SimplePerson{
    name: String
}

impl FarewellHello for SimplePerson{
    fn farewall(&self) -> String {
        format!("Good Bye... {}", self.name)
    }

    fn farewall_to(&self, name: &str) -> String {
        format!("Good Bye {}, hope we meet again soon {}", name, self.name)
    }
}

#[allow(dead_code)]
fn create_simple_person(name: String) -> impl FarewellHello{
    SimplePerson{name}
}

#[test]
fn trait_return_test(){
    let nobody = create_simple_person(String::from("Adrian"));
    println!("{}", nobody.farewall_to("Parker"));
}

#[test]
fn trait_conflict_name_test(){
    let someone = Person{
        first_name: String::from("Lucian"),
        middle_name: String::from(""),
        last_name: String::from("High"),
        age: 34
    };

    Person::greeting(&someone, "Black");
    GreetingHello::greeting(&someone);
}

trait Communication: GreetingHello + FarewellHello{
    fn action(&self) -> String;
}

struct SimpleMan{
    name: String
}

impl GreetingHello for SimpleMan{
    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello, {}! I'm {}.", name, self.name)
    }
}

impl FarewellHello for SimpleMan{
    fn farewall(&self) -> String {
        format!("Goodbye, {}!", self.name)
    }
    
    fn farewall_to(&self, name: &str) -> String {
        format!("Goodbye, {}! From {}.", name, self.name)
    }
}

impl Communication for SimpleMan{
    fn action(&self) -> String {
        format!("{} & {}", self.greeting(), self.farewall())
    }
}

#[test]
fn trait_super_test(){
    let person = SimpleMan { name: String::from("Alice") };
    
    println!("{}", person.say_hello());
    println!("{}", person.say_hello_to("Bob"));
    println!("{}", person.farewall());
    println!("{}", person.farewall_to("Bob"));
    println!("{}", person.action());
}

#[allow(dead_code)]
struct Point<T = i64>{
    x: T,
    y: T
}

trait GetValue<X> where X: PartialOrd {
    fn get_value(&self) -> &X;
}

#[allow(dead_code)]
impl<T> Point<T>{
    fn get_x(&self) -> &T{
        &self.x
    }

    fn get_y(&self) -> &T{
        &self.y
    }
}

impl<X> GetValue<X> for Point<X> where X: PartialOrd{
    fn get_value(&self) -> &X {
        &self.x
    }
}

#[test]
fn generic_test() {
    // Explicit
    let _loc: Point<i32> = Point::<i32>{
        x: 10,
        y: -10
    };

    // Explicit Type
    let _loc = Point::<i32>{
        x: 10,
        y: -10
    };

    // Smart Compiler
    let _loc = Point{
        x: 10,
        y: -10
    };
}

#[allow(dead_code)]
enum Data<L> {
    NONE, VALUE(L)
}

#[test]
fn generic_enum_test() {
    let backup = Data::VALUE("Cookies");
    match backup {
        Data::NONE => println!("Datas is None"),
        Data::VALUE(val) => println!("Datas is {}", val)
    }
}

#[allow(dead_code)]
struct Hello<V: GreetingHello>{
    value: V
}

#[test]
fn generic_type_bound_test() {
    let hi = Hello{
        value: SimpleMan{
            name: String::from("Bayley")
        }
    };

    println!("{}", hi.value.say_hello());
}

#[allow(dead_code)]
fn minimal<T: PartialOrd>(x: T, y: T) -> T{
    if x > y{
        y
    }else {
        x
    }
}

#[test]
fn generic_function_test() {
    let x = 10;
    let y = 20;

    let z = minimal(x, y);
    println!("{}", z);
}

#[test]
fn generic_method_test() {
    let loc = Point{
        x: 2.01,
        y: -10.34
    };

    println!("{}", loc.get_x());
    println!("{}", loc.get_y());
    println!("{}", loc.get_value());
}

struct Apple{
    quantity: i32
}

impl Add for Apple{
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple{
            quantity: self.quantity + rhs.quantity
        }
    }
}

#[test]
fn overloadable_test() {
    let apple_stock1 = Apple{quantity: 100};
    let apple_stock2 = Apple{quantity: 150};
    
    let total_stock_apple = apple_stock1 + apple_stock2;
    println!("Total Stock Apple = {}", total_stock_apple.quantity);
}

#[allow(dead_code)]
fn double_value(i: Option<i32>) -> Option<i32>{
    match i {
        None => None,
        Some(val) => Some(val * 2)
    }
}

#[test]
fn option_test() {
    let val = double_value(Some(100));
    println!("{:?}", val);
    // println!("{}", val.unwrap());

    let val_none = double_value(None);
    println!("{:?}", val_none);
    // println!("{}", val_none.unwrap());
}

impl PartialEq for Apple{
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }

    fn ne(&self, other: &Self) -> bool {
        self.quantity != other.quantity
    }
}

impl PartialOrd for Apple{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // if self.quantity > other.quantity{
        //     Some(Ordering::Greater)
        // }else if self.quantity < other.quantity{
        //     Some(Ordering::Less)
        // }else {
        //     Some(Ordering::Equal)
        // } MANUAL WAY
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn comparing_test() {
    let apple1 = Apple{quantity: 100};
    let apple2 = Apple{quantity: 200};

    println!("Apple 1 = {} == Apple 2 = {}: {}", apple1.quantity, apple2.quantity, apple1 == apple2);
    println!("Apple 1 = {} > Apple 2 = {}: {}", apple1.quantity, apple2.quantity, apple1 > apple2);
    println!("Apple 1 = {} < Apple 2 = {}: {}", apple1.quantity, apple2.quantity, apple1 < apple2);
}

#[test]
fn string_format_test() {
    let x = String::from("Fadhil Firmansyah");

    println!("{}", x.to_uppercase());
    println!("{}", x.to_lowercase());
    println!("{}", x.len());
    println!("{}", x.replace("Fadhil", "Rangga"));
    println!("{}", x.contains("Firmansyah"));
    println!("{}", x.starts_with("Fadhil"));
    println!("{}", x.ends_with("Firmansyah"));
    println!("{}", x.trim());
    println!("{}", &x[0..6]);
    println!("{:?}", x.get(0..6));
}

#[derive(Debug)]
#[allow(dead_code)]
struct Category{
    id: String,
    name: String
}

// impl Debug for Category{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Category")
//             .field("Id", &self.id)
//             .field("Name", &self.name)
//             .finish()
//     }
// }

#[test]
fn formatting_test() {
    let komputer = Category{
        id: String::from("01-Elektronik"),
        name: String::from("Asus Zepyrhus")
    };

    println!("{:?}", komputer);
}

#[test]
fn closure_test() {
    // Minimalist Type
    let _sum_minimalist = |val1: i32, val2: i32| -> i32{
        val1 + val2
    };

    // Explicit Type
    let sum: fn(i32, i32) -> i32 = |val1: i32, val2: i32| -> i32{
        val1 + val2
    };

    println!("{}", sum(100, 199));
}

#[allow(dead_code)]
fn print_with_upper(text: String, filter: fn(String) -> String){
    let result = filter(text);
    println!("{}", result);
}

#[test]
fn closure_parameter_test() {
    print_with_upper(String::from("Hello world"), |value: String| -> String {
        value.to_uppercase()
    });
}

#[allow(dead_code)]
fn text_uppercase(txt: String) -> String{
    txt.to_uppercase()
}

#[test]
fn closure_function_test() {
    let txt = String::from("welcome to half side lad");
    print_with_upper(txt, text_uppercase);
}

#[test]
fn closure_scope_test() {
    let mut counter = 0;
    
    let mut increment = ||{
        counter += 1;
        println!("Counter Incremented");
    };

    increment();
    increment();
    increment();

    println!("Value of counter: {}", counter);
}


#[allow(dead_code)]
struct Counter{
    n: i32
}

#[allow(dead_code)]
impl Counter{
    fn increment(&mut self){
        self.n += 1;
        println!("Increment");
    }
}

#[test]
fn closure_scope_alternate_test() {
    let mut counter = Counter{n: 0};
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Counter: {}", counter.n);
}

#[test]
fn vector_test() {
    // Implicit Typing
    let mut _names = Vec::new();
    _names.push(String::from("Fadhil"));
    _names.push(String::from("Firmansyah"));

    // Explicit Typing
    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Fadhil"));
    names.push(String::from("Firmansyah"));
    names.push(String::from("Priyatama"));
    names.pop();

    for name in  &names{
        println!("{}", name);
    }

    println!("{:?}", names);

    println!("{}", names[names.len() - 1]);
}

#[test]
fn vecdeque_test() {
    // Implicit Typing
    let mut names1 = VecDeque::new();
    names1.push_front(String::from("Fadhil"));
    names1.push_back(String::from("Firmansyah"));
    names1.push_front(String::from("Rangga"));
    names1.pop_front();

    // Explicit Typing
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_front(String::from("Fadhil"));
    names.push_back(String::from("Firmansyah"));
    names.push_front(String::from("Rangga"));
    names.pop_front();

    for name in &names{
        println!("{}", name);
    }

    println!("{:?}", names);

    println!("{}", names[names.len() - 1]);
}

#[test]
fn linked_list_test() {
    // Implicit Typing
    let mut names1 = LinkedList::new();
    names1.push_front(String::from("Fadhil"));
    names1.push_back(String::from("Firmansyah"));
    names1.push_front(String::from("Rangga"));
    names1.pop_front();

    // Explicit Typing
    let mut names: LinkedList<String> = LinkedList::<String>::new();
    names.push_front(String::from("Fadhil"));
    names.push_back(String::from("Firmansyah"));
    names.push_front(String::from("Rangga"));
    names.pop_front();

    for name in &names{
        println!("{}", name);
    }

    println!("{:?}", names);

    // println!("{}", names[names.len() - 1]); Error causing LinkedList cant read by index
}

#[test]
fn hash_map_test() {
    // Implicit
    let mut _names = HashMap::new();
    _names.insert(String::from(""), String::from(""));

    // Explicit
    let mut names: HashMap<String, String> = HashMap::<String, String>::new();
    names.insert(String::from("first_name"), String::from("Fadhil"));
    names.insert(String::from("last_name"), String::from("Firmansyah"));
    names.insert(String::from("middle_name"), String::from("Priyatama"));
    names.remove(&String::from("middle_name")).unwrap();

    //Manual Wayyyyy
    let _first_name = match names.get("first_name") {
        None => "Kosong",
        Some(name) => name
    };

    let _last_name = match names.get("last_name") {
        None => "Kosong",
        Some(name) => name
    };

    //Automate wayyyyyy
    let first_name = names.get("first_name").unwrap();
    let last_name = names.get("last_name").unwrap();

    println!("{}", first_name);
    println!("{}", last_name);

    for name in names{
        println!("{} {}", name.0, name.1);
     }
}

#[test]
fn btree_test() {
     // Implicit
     let mut _names = BTreeMap::new();
     _names.insert(String::from(""), String::from(""));
 
     // Explicit
     let mut names: BTreeMap<String, String> = BTreeMap::<String, String>::new();
     names.insert(String::from("first_name"), String::from("Fadhil"));
     names.insert(String::from("last_name"), String::from("Firmansyah"));
     names.insert(String::from("middle_name"), String::from("Priyatama"));
     names.remove(&String::from("middle_name")).unwrap();
 
     //Manual Wayyyyy
     let _first_name = match names.get("first_name") {
         None => "Kosong",
         Some(name) => name
     };
 
     let _last_name = match names.get("last_name") {
         None => "Kosong",
         Some(name) => name
     };
 
     //Automate wayyyyyy
     let first_name = names.get("first_name").unwrap();
     let last_name = names.get("last_name").unwrap();
 
     println!("{}", first_name);
     println!("{}", last_name);

     for name in names{
        println!("{} {}", name.0, name.1);
     }
}

#[test]
fn hashset_test() {
    // Implicit Typing
    let mut _names = HashSet::new();
    _names.insert(String::from("Fadhil"));
    _names.insert(String::from("Firmansyah"));

    // Explicit Typing
    let mut names: HashSet<String> = HashSet::<String>::new();
    names.insert(String::from("Fadhil"));
    names.insert(String::from("Fadhil"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Priyatama"));
    names.remove("Priyatama");

    for name in  &names{
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn btree_set_test() {
    // Implicit Typing
    let mut _names = BTreeSet::new();
    _names.insert(String::from("Fadhil"));
    _names.insert(String::from("Firmansyah"));

    // Explicit Typing
    let mut names: BTreeSet<String> = BTreeSet::<String>::new();
    names.insert(String::from("Fadhil"));
    names.insert(String::from("Fadhil"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Firmansyah"));
    names.insert(String::from("Priyatama"));
    names.remove("Priyatama");

    for name in  &names{
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn iterator_test() {
    let datas = [1,2,3,4,5,6,7,8,9,];
    let mut data = datas.iter();

    while let Some(value) = data.next(){
        println!("{}", value);
    }

    for value in data{
        println!("{}", value)
    }
}

#[test]
fn iterator_function_test() {
    let datas = vec![1,2,3,4,5,6,7,8,9];
    println!("{:?}", datas);

    let sum: i32 = datas.iter().sum();
    println!("{}", sum);

    let total = datas.iter().count();
    println!("{}", total);

    let length = datas.iter().len();
    println!("{}", length);

    let length = datas.len();
    println!("{}", length);

    let double_all: Vec<i32> = datas.iter().map(|data| data * 2).collect();
    println!("{:?}", double_all);

    let odd: Vec<&i32> = datas.iter().filter(|data| *data % 2 != 0).collect();
    println!("{:?}", odd);

    let even: Vec<&i32> = datas.iter().filter(|data| *data % 2 == 0).collect();
    println!("{:?}", even);
}

#[allow(dead_code)]
fn connect_server(host: Option<String>){
    match host {
        None => panic!("No Server found to online"),
        Some(host) => println!("Succsessful online to {}", host)
    }
}

fn connect_email(host: Option<String>) -> Result<String, String>{
    match host{
        None => Err(String::from("Can't connect to Email")),
        Some(val) => Ok(val)
    }
}

fn connect_database(host: Option<String>) -> Result<String, String>{
    match host{
        None => Err(String::from("Can't connect to Database")),
        Some(val) => Ok(val)
    }
}

#[allow(dead_code)]
fn connect_app_unoptimised(host: Option<String>) -> Result<String, String>{
    let result_email = connect_email(host.clone());
    match result_email {
        Ok(_) => {},
        Err(msg) => {return Err(msg)} 
    }

    let result_database = connect_database(host.clone());
    match result_database{
        Ok(_) => {},
        Err(msg) => {return Err(msg)} 
    }

    Ok(String::from("Successfull connect to application"))
}

#[allow(dead_code)]
fn connect_app_optimised(host: Option<String>) -> Result<String, String>{
    connect_database(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connected to application".to_string())
}

#[test]
fn unrecoverable_error_test() {
    // connect_server(Some(String::from("TOKIO")));
    connect_server(None);
}

#[test]
fn recoverable_error_test() {
    // let success = connect_email(Some(String::from("serde")));
    let success = connect_email(None);
    match success {
        Ok(msg) => println!("Email connect and found w/ {}", msg),
        Err(msg) => println!("Error message = {}", msg)
    }
}

#[test]
fn question_mark_operator_test() {
    // let result = connect_app_optimised(Some(String::from("Nii")));
    let result = connect_app_optimised(None);
    match result{
        Err(msg) => println!("Error Message = {}", msg),
        Ok(succ) => println!("Success Message = {}", succ)
    }
}

#[test]
fn dangling_reference_test() {
    let value: &i32;
    {
        let _x = 100;
        // value = &_x; variable x will drop after out of scope
    }
    value = &100;
    println!("{}", value);
}

// fn longest(n1: &str, n2: &str) -> &str{
//     if n1 > n2 {
//         n1
//     }else {
//         n2
//     }
// }

//Using Lifetime Annotation
#[allow(dead_code)]
fn longest<'a>(n1: &'a str, n2: &'a str) -> &'a str{
    if n1 > n2 {
        n1
    }else {
        n2
    }
}

#[test]
fn lifetime_annotation_test() {
    let name1 = "Fadhil";
    let name2 = "Firmansyah";
    let longest_name  =longest(&name1, &name2);
    println!("{}", longest_name);
}

#[test]
fn lifetime_annotation_dangling_test() {
    let name1 = String::from("Fadhil");
    let longest_name: &str;
    {
        let _name2 = String::from("Firmansyah");
        // longest_name  =longest(&name1.as_str(), &name2.as_str());
    }
    let name2 = String::from("Firmansyah");
    longest_name = longest(&name1.as_str(), &name2.as_str());
    println!("{}", longest_name);
}

struct Student<'a>{
    name: &'a str
}

#[allow(dead_code)]
impl<'a> Student<'a> {
    fn longest_name_name(&self, student2: &Student<'a>) -> &'a str{
        if self.name.len() > student2.name.len(){
            self.name
        }else {
            student2.name
        }
    } 
}

#[allow(dead_code)]
fn longest_name_student<'a>(student1: &Student<'a>, student2: &Student<'a>) -> &'a str{
    if student1.name.len() > student2.name.len(){
        student1.name
    }else {
        student2.name
    }
}

#[test]
fn lifetime_struct_test() {
    let budianto = Student{
        name: "Budianto"
    };

    let heru = Student{
        name: "Heru"
    };

    let result_election = longest_name_student(&budianto,&heru);
    println!("{}", result_election);
}

#[test]
fn lifetime_struct_method_test() {
    let budianto = Student{
        name: "Budianto"
    };

    let heru = Student{
        name: "Heru"
    };

    let result_election = heru.longest_name_name(&budianto);
    println!("{}", result_election);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Teacher<'a, V: Ord>{
    name: &'a str,
    sallary: V
}

#[test]
fn lifetime_struct_generic_test() {
    let math_teacher = Teacher{
        name: "Andrew State",
        sallary: 10000000
    };

    println!("{:?}", math_teacher);
}

#[allow(dead_code)]
fn print_w_reference(number: &i32){
    println!("{}", number);
}

#[allow(dead_code)]
fn print_w_dereference(number: i32){
    println!("{}", number);
}

#[test]
fn smart_pointer_test() {
    let value = Box::new(120);

    println!("{}", value);

    print_w_reference(&value);
    print_w_dereference(*value);
}

#[allow(dead_code)]
#[derive(Debug)]
enum CategoryInProduct{
    Of(String, Box<CategoryInProduct>),
    End
}

#[test]
fn smart_pointer_recursice_field_test() {
    let rog = CategoryInProduct::Of(
        String::from("Laptop"),
        Box::new(CategoryInProduct::Of(
            String::from("Asus"),
            Box::new(CategoryInProduct::Of(
                String::from("Republic of Gamers"), 
                Box::new(CategoryInProduct::End
            )))
        ))
    );

    println!("{:?}", rog);
}

#[test]
fn dereference_test() {
    let value1 = Box::new(100);
    let value2 = Box::new(100);

    let times = *value1 * *value2;
    println!("{}", times);
}

struct MyValue<T>{
    value: T
}

impl<T> Deref for MyValue<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn dereference_value_test() {
    let value = MyValue{value: 100};
    let values = *value;

    println!("{}", values);
    println!("{}", value.value);
}

#[allow(dead_code)]
fn out_value(val: &String){
    println!("{}", val);
}

#[test]
fn dereference_parameter_test() {
    let value = MyValue{value: String::from("Hijau")};
    out_value(&value);
}

struct Book<'a>{
    title: &'a str
}

impl<'a> Drop for Book<'a>{
    fn drop(&mut self) {
        println!("Memory Data Book: {}", self.title);
    }
}

#[test]
fn cleanup_test() {
   let calculus = Book{title: "Calculus for Idiot 101"};
   println!("Name Book: {}", calculus.title); 
}

#[allow(dead_code)]
enum Brand{
    Of(String, Rc<Brand>),
    End
}

#[test]
fn multiple_ownership_test() {
    let asus = Rc::new(Brand::Of(String::from("Asus"), Rc::new(Brand::End)));
    println!("Brand ASUS Reference Count(Rc) = {}", Rc::strong_count(&asus));
    let _laptop = Brand::Of(String::from("Laptop"), Rc::clone(&asus));
    println!("Brand ASUS Reference Count(Rc) = {}", Rc::strong_count(&asus));

    {
        let _smartphone = Brand::Of(String::from("Smartphone"), Rc::clone(&asus));
        println!("Brand ASUS Reference Count(Rc) = {}", Rc::strong_count(&asus));
    }
    println!("Brand ASUS Reference Count(Rc) = {}", Rc::strong_count(&asus));
}

#[allow(dead_code)]
#[derive(Debug)]
struct Seller{
    name: RefCell<String>,
    active: bool
}

#[test]
fn interior_mut_test() {
    let jual_rokok = Seller{
        name: RefCell::new(String::from("Gopal")),
        active: false
    };
    println!("{:?}", jual_rokok);

    let mut alter_name = jual_rokok.name.borrow_mut();
    *alter_name = String::from("Bagogo");

    println!("{:?}", jual_rokok);
}

#[allow(dead_code)]
static APPLICATION: &str = "My Environment";

#[test]
fn static_test() {
    println!("{}", APPLICATION);    
}

#[allow(dead_code)]
static mut COUNTER: i32 = 0;

#[allow(dead_code)]
unsafe fn increment(){
    COUNTER += 1;
}

#[test]
fn static_unsafe_test() {
    unsafe{
        increment();
        COUNTER += 1;
        println!("{}", COUNTER);
    }
}

#[allow(unused_macros)]
macro_rules! hello {
    () => {
        println!("Hello, World!!!");
    };
}

#[allow(unused_macros)]
macro_rules! say_hello {
    ($value: expr) => {
        println!("Hello, {}", $value);
    };
}

#[allow(unused_macros)]
macro_rules! repeat_please {
    ($val: expr) => {
        for i in $val{
            println!("{}", i);
        }
    };

    ($($vals: expr), *) =>{
        $(
            println!("{}", $vals);
        )*
    }
}

#[test]
fn macro_test() {
    let name = "Fadhil Firmansyah";
    let values = [100, 200, 300, 400, 500, 600, 700, 800, 900];

    hello!();

    say_hello!("Fadhil");

    say_hello!{
        "Firmansyah"
    }

    repeat_please!(1,2,3,4,5,6,7,8,9);
    repeat_please!([1,2,3,4,5,6,7,8,9]);

    say_hello!(name);
    repeat_please!(values);
}