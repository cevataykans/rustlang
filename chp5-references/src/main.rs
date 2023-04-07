use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    println!("Hello, world!");

    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    println!("{:?}", &table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry); // their referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        // assert_eq!(*r, 1); throws error as the reference to the value is dropped :)
    }

    static WORTH_POINTING_AT: i32 = 1000_i32;
    f(&WORTH_POINTING_AT);

    // struct S<'a> {
    //     x: &'a i32,
    //     y: &'a i32,
    // }
    struct S<'a, 'b> {
        // create two different lifetimes
        x: &'a i32,
        y: &'b i32,
    }

    let a = 10;
    let b;
    {
        let c = 20;
        {
            let s = S { x: &a, y: &c };
            b = s.x;
        }
    }
    println!("{}", b);

    let mut wave: Vec<f64> = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    // extend(&mut wave, &wave); // error, cannot readonly borrow and mutable at the same time!
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut cur = &v[0];
    for num in &v[1..] {
        if *cur > *num {
            cur = num;
        }
    }
    cur
}

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
