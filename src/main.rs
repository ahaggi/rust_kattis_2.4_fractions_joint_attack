use std::io;

fn main() {
    let mut res: (u64, u64) = (1, 0);

    

    // let mut data: Vec<i32> = vec![5, 1, 1, 2];
    // let mut data:[i32;4] = [5, 1 ,1, 2];

    
    let stdin = io::stdin();
    // we could use this value to create an array instead of vec
    //-----------------unused------------
    let mut data_size = String::new();
    stdin
        .read_line(&mut data_size)
        .expect("Failed to read line");
    let data_size: i32 = data_size.trim().parse::<i32>().unwrap();
    //-----------------------------------

    let mut data = String::new();
    stdin.read_line(&mut data).expect("Failed to read line"); // reading the number of sheets he has of each paper size starting with A2 and ending with An.
    let mut data: Vec<i32> = data
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    gen_simple_fraction_down_up(&data[..]);
    gen_simple_fraction_top_down(&data[..]);

}

fn gen_simple_fraction_down_up(data: &[i32]) {
    fn switch((a, b): (i32, i32)) -> (i32, i32) {
        (b, a)
    }

    let mut ind = data.len() as i32 - 1;

    let mut res: (i32, i32);
    if data.len() > 0 {
        res = (data[ind as usize], 1);
        ind -= 1;
    } else {
        res = (0, 1);
    }

    while ind >= 0 {
        let mut temp = data[ind as usize];
        println!("{:#?}/{:#?}", res.0, res.1);
        res = switch(res);
        temp = temp * res.1 + res.0;
        res.0 = temp;
        ind -= 1;
    }
    println!("{:#?}/{:#?}", res.0, res.1);
    println!("--------------");
}

fn gen_simple_fraction_top_down(data: &[i32]) {
    // For a given term a(k), we can compute a result p(k) / q(k)
    // We always start by defining p and q for k=0 and k=-1
    // p(-1) = 0, q(-1) = 1
    // p( 0) = 1, q( 0) = 0
    //
    // p(k) = a * p(-1) + p(-2)
    // q(k) = a * q(-1) + q(-2)

    let mut res = [(0, 1), (1, 0)];

    for term in data.iter() {
        println!("{:#?}/{:#?}", res[1].0, res[1].1);

        let p = term * res[1].0 + res[0].0;
        let q = term * res[1].1 + res[0].1;
        res[0] = res[1];
        res[1] = (p, q);
    }

    println!("{:#?}/{:#?}", res[1].0, res[1].1);

}
