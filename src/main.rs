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
fn static_test(){
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