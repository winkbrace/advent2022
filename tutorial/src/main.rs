const HELLO_WORLD: &str = "Hello, world!";

fn main() {
    printing();
    type_casting();
    functions();
    primitives();
    conditionals();
    loops();

    // Slicing is something unique. You can reference a slice of another data structure and keep the references!
    let a: [i8; 4] = [1, 2, 3, 4];
    let _to_index_3 = &a[..3]; // ..to is always excluding, so: [1, 2, 3]
    let _from_index_1 = &a[1..];
    let _from_index_1_to_3 = &a[1..3];
}

fn loops() {
    // a loop is an infinite loop by design
    let mut a = 0;
    loop {
        if a == 0 {
            println!("Skip Value : {}", a);
            a += 1;
            continue;
        } else if a == 2 {
            println!("Break At : {}", a);
            break;
        }

        println!("Current Value : {}", a);
        a += 1;
    }

    // while works as expected

    // for
    let mut total = 0;
    for i in 1..=10 { // ..10 is exclusive. ..=10 makes it inclusive
        total += i;
    }
    println!("Total sum is {total}");

    // for with arrays/vectors
    let group: [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];
    for n in 0..group.len() { // group.len() = 4 -> 0..4 👎 check group.len()on each iteration
        println!("Current Person : {}", group[n]);
    }
    // With iter() we have basically a foreach
    for person in group.iter() { // 👍 group.iter() turn the array into a simple iterator
        println!("Current Person : {}", person);
    }

    // you can name loops, so you can break from any when nested
    'columns: for c in 1..6 {
        '_rows: for r in 1..6 {
            if c == 2 && r == 2 { break 'columns; }
        }
    }

    // mutating a vector in a loop
    let mut v = vec![1, 2, 3, 4, 5];
    for n in &v {
        println!("An immutable reference to {n}");
    }
    for n in &mut v {
        println!("A mutable reference to {n}");
        *n += 10;
    }
    println!("{:?}", v);
}

fn conditionals() {
    // if statements have no ()
    let age = 13;
    if age >= 18 {
        println!("You can order drinks");
    } else if age >= 12 {
        println!("You pay full price in public transport");
    } else {
        println!("You have no care in the world");
    }

    // ternary is also an if (I like that!)
    let _sleeping = if age <= 2 { true } else { false };

    // match
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S", // check 16
        17 | 18 => "M", // check 17 and 18
        19..=21 => "L", // check from 19 to 21 (19,20,21)
        22 => "XL",
        _ => "Not Available",
    };

    println!("{}", tshirt_size); // L
}

fn printing() {
    println!("{}", HELLO_WORLD);

    let (greeting, name) = ("Hi there", "Driehuis");
    println!("{greeting}, {name}!");
}

fn type_casting() {
    // variable shadowing: reuse same variable for different type
    let x: f64 = 2.2872349;
    let x: i64 = x.floor() as i64; // cast to int

    println!("x is of type {}", type_of(&x));
}

fn functions() {
    let x: i64 = 3;

    // Anonymous functions have a.. new syntax
    let square = |i: i32| -> i32 { // Input parameters are passed inside | | and expression body is wrapped within { }
        i * i
    };
    println!("The square of 2 is {}", square(x as i32)); // cast to i32 on the fly

    // You can even create a variable for a function that will always be called with a parameter
    let x_square = |i: i32| -> i32 { i * i }(x as i32);
    println!("The square of x is {}", x_square);
}

fn primitives() {
    // unsigned integers:
    let _y: u8 = 23;
    println!("The highest possible value for _y is {}", u8::max_value());

    // Arrays are immutable by default and even with mut, its element count cannot be changed.
    // The array length is fixed on creation. Growable arrays are called vectors.
    let mut a: [i32; 3] = [1, 2, 3]; // or: let mut a = [1, 2, 3];
    a[1] = 4;
    println!("{:?}", a);

    // Vectors
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

    // A tuple is a set of variables. It has a fixed structure determined on creation.
    let t = (1, 1.5, true, 'a'); // or: let a: (i32, f64, bool, char) = (1, 1.5, true, 'a');
    println!("{:?}", t);

    // Use String when you need ownership, else &str. You can cast one to the other.
    let s: &str = "Hello"; // &str

    let s = s.to_string(); // String
    let s = String::from(s); // String
    println!("{s}");

    let s = s.as_str(); // &str
    println!("{s}");
}

fn type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}