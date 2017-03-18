


fn main() {
    // Shall yield: 1, 4, 9, 16, 25, ...
    // because we want to see something, take(5)
    let result: Vec<_> = (1..).map(|x| x * x).take(5).collect();
    println!("{:?}", result);

    // Add two list of numbers
    // list_a and list_b are both Vec<i32>
    let list_a = (1..11);
    let list_b = (99..110);
    println!("{:?}", list_a.zip(list_b).map(|(x , y)| x + y).collect::<Vec<_>>());
    // the shortes range defines the whole lenght
    let result: Vec<_> = (1..11).zip((99..)).map(|(x, y)| x + y).collect();
    println!("{:?}", result);
}
