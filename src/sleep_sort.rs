// Sleep sort is crowned the "King of Laziness" when it comes to sorting as it sorts while sleeping.
// The sorting algorithm works by starting a separate task for each item to be sorted, where each
// task sleeps for an interval corresponding to the item's sort key, then emits the item. Items are
// then collected sequentially in time. Sleep sort was first proposed anonymously on 4chan and later
// discussed on Hacker News and Reddit

// In terms of runtime complexity, a sleeping sort makes at most O(hightest_value_in_input + n) where
// the algorithm sleeps for n units of time for each element all dependent  upon the highest value
// inputed by the user

// This algorithm is different from the other sorting algorithms as, instead of having tests, it prints
// out the sorted list line by line as traditionally outlined in the original bash script via 4chan 

use std::thread;

fn sleep_sort<I: Iterator<Item=u32>>(nums: I) {
    let threads: Vec<_> = nums.map(|n| thread::spawn(move || {
        thread::sleep_ms(n);
        println!("{}", n);
    })).collect();

    for t in threads { t.join(); }
}

fn main() {
    sleepsort(std::env::args().skip(1).map(|s| s.parse().unwrap()));
}