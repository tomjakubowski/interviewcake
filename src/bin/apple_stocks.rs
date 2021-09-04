fn max_profit(prices: &[f64]) -> Option<f64> {
    if prices.len() < 2 {
        return None;
    }
    let mut best_profit = -10000000000.0;
    let mut lowest_buy_price = prices[0];
    for &price in &prices[1..] {
        let potential_profit = price - lowest_buy_price;
        if potential_profit > best_profit {
            best_profit = potential_profit;
        }
        lowest_buy_price = lowest_buy_price.min(price);
    }
    if best_profit < 0.0 {
        None
    } else {
        Some(best_profit)
    }
}

fn main() {
    println!("{:?}", max_profit(&[10., 7., 5., 8., 11., 9.]));
    println!("{:?}", max_profit(&[5.0, 4., 3., 2., 1.]));
}
