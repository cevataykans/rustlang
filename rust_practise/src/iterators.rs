pub fn simulate_iterators() {
    println!("\n**** Iterators ****\n");

    let text = " ponies \n giraffes\niguanas \nsquid".to_string();
    let mut text2: String = String::new();
    text2.push('5');
    text2 = text2.trim().to_string();
    println!("{}", text2);

    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    println!("\n**** End Iterators ****\n");
}
