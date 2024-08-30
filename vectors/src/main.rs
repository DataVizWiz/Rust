fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vec);

    for i in &mut vec {
        *i += 5;
    }
    println!("{:?}", vec);

    vec.pop();
    println!("{:?}", vec);

    vec.insert(0, 100);
    println!("{:?}", vec);

    let min = vec.iter().min();
    match min {
        Some(x) => println!("{}", x),
        None => println!("Vector is empty")
    }

    let max = vec.iter().max();
    match max {
        Some(x) => println!("{}", x),
        None => println!("Vector is empty")
    }

    vec.sort();
    println!("{:?}", vec);
}
