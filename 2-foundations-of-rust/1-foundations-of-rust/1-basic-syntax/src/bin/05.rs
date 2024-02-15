fn main() {
    let mut data: [i32; 5] = [22, 12, 13, 17, 18];
    for i in 0..5 {
        data[i]= floored_half(data[i]);
    }

    println!("new data is {:?}", data);
}

fn floored_half(data: i32) -> i32 {
    data / 2
}
