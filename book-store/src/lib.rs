const PRICE: u32 = 800;

const DISCOUNTS: &[Discount] = &[
    Discount {
        required_books: 2,
        discount_amount: 40,
    },
    Discount {
        required_books: 3,
        discount_amount: 80,
    },
    Discount {
        required_books: 4,
        discount_amount: 160,
    },
    Discount {
        required_books: 5,
        discount_amount: 200,
    },
];

struct Discount {
    required_books: u32,
    discount_amount: u32,
}

fn get_biggest_discount(counts: &[u32]) -> u32 {
    let num_distinct = counts.iter().filter(|i| **i > 0).count();

    DISCOUNTS
        .iter()
        .filter(|discount| num_distinct >= discount.required_books as usize)
        .map(|discount| {
            let mut items_after_discount = counts.to_vec();
            items_after_discount.sort_unstable_by(|a, b| b.cmp(&a));
            (0..discount.required_books).for_each(|i| items_after_discount[i as usize] -= 1);

            discount.required_books * discount.discount_amount
                + get_biggest_discount(&items_after_discount)
        })
        .max()
        .unwrap_or(0)
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let basket_price = books.len() as u32 * PRICE;

    let mut counts = vec![0, 0, 0, 0, 0];
    books.iter().for_each(|i| counts[*i as usize - 1] += 1);

    basket_price - get_biggest_discount(&counts)
}
