use std::str::FromStr;


fn main() {
    let input = "1, 2, 3; 7, 8,9; 6;3,5;";
    println!("{:#?}", parse_list(input));
}


// "1, 2, 3; 7, 8,9; 6;3,5;"
fn parse_list(input: &str)
    -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err>
{
    input.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse())
                .collect()
        })
        .collect()
}
