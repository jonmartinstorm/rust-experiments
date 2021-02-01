fn main()  {
    println!("Hello, world!");

    let f = String::from("Hergheg");

    loop {
        let value = &f;
        let v2 = f.clone();

        println!("{} {}", value, v2);
    }
}
