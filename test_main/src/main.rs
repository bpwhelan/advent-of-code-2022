fn main() {
    for (i, c) in "[S] [P]     [G] [L] [H] [Z]     [T]".chars().enumerate() {
        if c.is_uppercase() {
            println!("{} : {}", c, i/4);
        }
    }
}
