fn main() {
    println!("the hard way");
    the_hard_way();

    println!("the sugar way");
    for_suggar();

    println!("collectors");
    collect();

    println!("adapter");
    adapter();
}

fn the_hard_way() {
    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            }
            None => { break }
        }
    }
}

fn for_suggar() {
    let numbers = vec![1, 2, 3 ,4];
    for i in 0..numbers.len() {
        println!("{}", i);
    }

    for item in &numbers {
        println!("{}", item);
    }
}

fn collect() {
    println!("sum");
    let all = (0..101).collect::<Vec<_>>();
    println!("{:?}", all);
    
    println!("find");
    let matches = (0..101).find(|x| *x > 42);
    match matches {
        Some(x) => println!("found: {}", x),
        None => println!("nope")
    }

    println!("reduce");
    let res = (0..11).fold(0, |sum, x | x + sum);
    println!("{}", res);
}

fn adapter() {
    for i in (0..10).map(|x| x + 10) {
        println!("{}", i);
    }
}