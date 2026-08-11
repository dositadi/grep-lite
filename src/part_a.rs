/* 
let a = 13i32;
    let b = 15u16;

    let b_: i32 = Into::into(b);

    if b_ > a {
        println!("{b}");
    }

    let mut sample = vec![];

    for i in 0..=5 {
        if i % 2 != 0 {
            continue;
        }

        sample.push(i);
    }

    println!("sample: {sample:?}");

    let mut count = 0;
    let time_limit = time::Duration::new(1, 0);
    let start = Instant::now();

    while Instant::now() - start < time_limit {
        count += 1;
    }

    println!("{count}");

    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [6, 7, 8, 9, 10];
    let mut arr3 = vec![];

    for (a, b) in arr1.iter().zip(arr2) {
        arr3.push(a + b);
    }
    println!("{arr3:?}");

    //Rust like tenary operator
    let l = 123456;
    let description = if l % 2 == 0 { "even" } else { "odd" };
    println!("l is {description}");

    for item in &arr3 {
        print!("{}, ", *item);
    }

    println!();

    let needle = 0o204; // needle => the small value i am trying to find
    let haystack = [1, 1, 2, 132, 5, 15, 52, 203, 877, 4140, 21147]; // haystack => the large container where i want to lookup the needle

    for item in &haystack {
        if *item == needle {
            println!("Found the needle: {}", *item);
        } 
    }

    /* /* fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
} */

use std::{ ops::Add, time::Duration };
fn add<T>(i: T, j: T) -> T where T: Add<Output = T> {
    i + j
} */
*/