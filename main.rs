fn input() -> String {
    println!("Enter an int:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    return input
}
