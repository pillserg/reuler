use problems::utils;

pub fn largest_pallindrome_product(x: u32, y: u32) -> u32 {
    let mut best_match = 0;
    for _x in (0..x+1).rev() {
        for _y in (0..y+1).rev() {
            let product = _x * _y;
            if utils::is_pallindrom(product) && product > best_match {
                best_match = product
            }
        }
    }
    best_match
}
