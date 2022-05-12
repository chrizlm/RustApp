//first rust project
//import statement 
use std::fmt;

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
    
fn main() {
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("Hello, world! \n I'm a Rustacean!");

    //formatted println
    println!("{0}, this is {1}, {1} this is {0} and now lets talk about {subject} ", "alice", "john", subject="programing");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);

    
    
    /* #[allow(dead_code)]
    struct Structure(i32);
    println!("This struct `{}` wont print...", Structure(3));
    */

  println!("This struct `{:?}` will print...", Structure(3));
  println!("This struct `{:?}` will print...", Deep(Structure(8)));


    let pi = 3.141592;

    println!("Pi is roughly{:.03}", pi);

    //struct person
    let name = "cool";
    let age = 24;
    let cool = Person{name, age};
    println!("{:#?}", cool);

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D {x:3.3, y:7.2};
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let realpoints = RealNum{real:3.3, image:7.2};
    println!("Display: {}", realpoints);
    println!("Debug: {:?}", realpoints);

    //write!(f, "{}", value)?;
    
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    /* let w = ListEdited(vec![0: 1, 1: 2, 2: 3]);
    println!("{}", w); */

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

    for name in names.iter(){
        match name {
            &"you" => println!("Person of interest"),
            _=>println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}
