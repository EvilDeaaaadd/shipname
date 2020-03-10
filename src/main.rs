use text_io::read;

fn main() {
    println!("Enter names (space separated): ");
    let names: String = read!("{}\n");
    let names = names.split_whitespace().map(|x| x.to_owned()).collect::<Vec<String>>();
    let shipname: Result<String, ()> = match names.len() {
        0 => Err(()),
        1 => Ok(names[0].clone()),
        _ => {
            let mut first_halves = String::new();
            for name in &names[0..names.len()-1] {
                first_halves.push_str(&name[0..name.len() / 2]);
            }
            let first_halves = first_halves.as_str();
            let last_half = &names[names.len()-1][(names.len() / 2)+1..];

            Ok(format!("{}{}", first_halves, last_half))
        }
    };
    println!("Ship name: {}", shipname.expect("Invalid input!"));
}
