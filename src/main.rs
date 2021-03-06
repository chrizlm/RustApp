//first rust project
//import statement 
use std::fmt;
//use std::io;

//#[derive(Debug)]
struct Colors {
    red:u8,
    green:u8,
    blue:u8,
}



impl fmt::Display for Colors{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let hex = format!("{:02x}{:02x}{:02x}", self.red as u8, 
                                    self.green as u8,
                                     self.blue as u8);
        write!(f, "RGB ({}, {}, {}, {})", self.red, self.green, self.blue, hex)
    }
}

/* struct ListEdited(Vec<i32>);

impl fmt::Display for ListEdited{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let vec = &self.0;
        write!(f, "[")?;

        for(count, v) in vec.iter().enumerate(){
            if count !=0 {
                write!(f, ", ")?;
            } write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
} */

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let vec = &self.0;

        write!(f, "[")?;

        for(count, v) in vec.iter().enumerate(){
            if count != 0 { write!(f, ",")?;}
            write!(f, "{}",v)?;
        }
        write!(f,"]")
    }
}

#[derive(Debug)]
struct RealNum{
    real: f64,
    image: f64,
}



impl fmt::Display for RealNum{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real: {} + image: {}", self.real, self.image)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

//implement display for MinMax
impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real : {}, image : {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {} , y: {}", self.x, self.y)
    }
}

  #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    fn getting_known()-> String{
        let convo = "good programmer";
        convo.to_string()
    }

    fn hello_rust_prog(){
        let x = 5 + 5;
        println!("Is `x` 10 or 100? x = {}", x);
        println!("Hello rustacean!");

        getting_known();

        //formatted println
        println!("{0}, this is {1}, {1} this is {0} and now lets talk about {subject}", "alice", "john", subject="programing");
        println!("{} of {:b} people know binary, the order half doesn't ", 1, 2);

        println!("{number:>width$}", number=1, width=6);
        println!("{number:0>width$}", number=1, width=6);

    }

    fn person_struct(){
        //struct person
        let name = "cool";
        let age = 24;
        let cool = Person{name, age};
        println!("{:#?}", cool);
    }

    fn number_struct(){
        let minmax = MinMax(0, 14);
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let point = Point2D {x:3.3, y:7.2};
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        let realpoints = RealNum{real:3.3, image:7.2};
        println!("Display: {}", realpoints);
        println!("Debug: {:?}", realpoints);
    }

    fn loop_function(){
        //for loop
    //exclusive ..
    //inclusive ..=
         for n in 1..11{
        if n % 15 == 0{
            println!("fizzbuzz");
        }else if n % 3 == 0{
            println!("fizz");
        }else if n % 5 == 0{
            println!("buzz");
        }else{
            println!("{}", n);
        }
    }



    let names = vec!["who", "are", "you"];

    //iter allow reuse of the array
    for name in names.iter(){
        match name {
            &"you" => println!("Person of interest"),
            _=>println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);


    let cars = vec!["audi", "subaru", "bmw", "toyota"];

    //into_iter doesnt allow reuse of array
    for car in cars.into_iter(){
        match car{
            "toyota" => println!("there exits a huge number"),
            _=>println!("its a great {}", car),
        }
    }

    let mut sizes = vec!["big", "medium", "small", "others"];

    //iter_mut allow mutation of array and doesnt allow println in its args
    for size in sizes.iter_mut(){
        *size = match size{
            &mut "others" => "all sizes are great",
            _=> "type size great",
        }
    }

    println!("sizes: {:?}", sizes);

    for color in [
        Colors {red: 128, green: 255, blue: 90},
        Colors {red: 0, green: 3, blue: 254},
        Colors {red: 0, green: 0, blue: 0},
    ].iter(){
        println!("{}", *color);
    }

    }
    
fn main() {

    /* #[allow(dead_code)]
    struct Structure(i32);
    println!("This struct `{}` wont print...", Structure(3));
    */

  println!("This struct `{:?}` will print...", Structure(3));
  println!("This struct `{:?}` will print...", Deep(Structure(8)));


    let pi = 3.141592;

    println!("Pi is roughly{:.03}", pi);

    person_struct();

    number_struct();

    

    //write!(f, "{}", value)?;
    
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    /* let w = ListEdited(vec![0: 1, 1: 2, 2: 3]);
    println!("{}", w); */


    loop_function();
    
   hello_rust_prog();

    
    let x = 5;
    let x = x + 2;
    println!("{}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 4);
    let (x,y,z) = tup;
    //println!("{},{},{}",tup.0,tup.1,tup.2);
    println!("{},{},{}", x,y,z);
    expression_function();
    
}


fn expression_function(){
    let x = 7;

    let y = {
        let x = 4;
        x+1
    };
    println!("x:{} y:{}",x,y);

    let z = return_value(5);
    println!("z:{}", z);
}

fn return_value(x: i32) -> i32 {
    x + 2
}