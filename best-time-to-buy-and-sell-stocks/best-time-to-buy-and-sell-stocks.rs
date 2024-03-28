pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut minimum_agg = vec![];
    let mut maximum_agg: Vec<i32> = vec![0; prices.len()];

    let mut current_minimum = prices[0];

    for (_, &price) in prices.iter().enumerate() {
        if price < current_minimum {
            current_minimum = price;
        }

        minimum_agg.push(current_minimum);
    }

    let mut current_maximum = prices[prices.len() - 1];

    for (index, &price) in prices.iter().rev().enumerate() {
        if price > current_maximum {
            current_maximum = price;
        }

        maximum_agg[prices.len() - 1 - index] = current_maximum;
    }

    let mut max_profit_possible = 0;

    for i in 0..prices.len() - 1 {
        let profit = maximum_agg[i] - minimum_agg[i];

        if profit > max_profit_possible {
            max_profit_possible = profit;
        }
    }

    max_profit_possible
}

fn main() {
    dbg!(max_profit(vec![7, 6, 4, 3, 1]));
}
