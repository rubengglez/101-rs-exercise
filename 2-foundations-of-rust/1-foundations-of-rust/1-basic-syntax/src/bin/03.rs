fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut bigger = input[0];
    let mut smaller = input[0];
    for i in input {
        if i > bigger {
            bigger = i;
        }

        if i < smaller {
            smaller = i;
        }
    }

    println!("{} is largest and {} is smallest", bigger, smaller);
}
