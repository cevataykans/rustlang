fn main() {
    println!("Hello, world!");

    /*

            pixels[r * bounds.0 + c] =
                match escapes(Complex { re: point.0, im: point.1 }, 255) {
                     None => 0,
                Some(count) => 255 - count as u8
            };

            TERNARY OP ( exp1 ? exp2 : exp3) is not needed
            IF covers both
            let status =
                if cpu.temperature <= MAX_TEMP {
                    HttpStatus::Ok
                } else {
                   HttpStatus::ServerError // server melted
                };

            println!("Inside the vat, you see {}.",
                match vat.contents {
                Some(brain) => brain.desc(),
                None => "nothing of interest"
            });
    */
    let x: Vec<isize> = vec![5, -10, 22, 0, 5];
    println!("{:?}", &x[1..x.len()]);
    println!("{:?}", &x[1..=x.len() - 1]);

    // notice that mut is not needed for initialization!
    let name;
    if 5 + 10 == 15 {
        name = "Cevat";
    } else {
        name = "Aykan";
    }
}
