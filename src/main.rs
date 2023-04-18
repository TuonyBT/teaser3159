use itertools::Itertools;
use std::collections::{HashMap, BTreeSet};


//The denominations of coins are proportionate to the square of their diameter, and we need to choose from integer diameters
//whose squares are less than 64
//Choosing to work on the areas/denominations and convert back to diameters at the end
const VALS: [usize; 7] = [1, 4, 9, 16, 25, 36, 49];

fn main() {

    // First calculate the numbers of each denomination that produce a given transaction value with the fewest coins
    // We do this recursively, starting with transaction value 1, equivalent to a single coin of denomination 1
    // The work is done in the function disagg

    let seed: Vec<[usize; 7]> = vec![[0;7], [1, 0, 0, 0, 0, 0, 0]];
    let disaggs = disagg(63, &seed, VALS);

    let base_coins = disaggs.iter().fold(0, |acc, x| acc.max(x.iter().sum::<usize>()));
    println!("Base number of coins required for a transaction with all denominations {}", base_coins);
    println!("");

    let mut base_sets: HashMap<[usize; 7], Vec<[usize; 7]>> = HashMap::new();
    let mut const_sets: HashMap<[usize; 7], Vec<[usize; 7]>> = HashMap::new();

//  Once we know the minimum coin count in any transaction with all available denominations, we try to achieve
//  that minimum with fewer denominations available

    for no_deleted in 1..6 {
        for supps in (1..7).combinations(no_deleted) {
            let mut test_vals = VALS.to_owned();
            for idx in supps {
                test_vals[idx] = 64;
            }
            
            let disaggs = disagg(63, &seed, test_vals);

            let coins = disaggs.iter().fold(0, |acc, x| acc.max(x.iter().sum::<usize>()));

//  Collect all subsets of available denominations that give the minimum coin count in all transactions
            if coins - base_coins == 0 {
                let x_actions = disaggs.into_iter()
                .filter(|x| x.iter().sum::<usize>() == base_coins)
                .collect::<Vec<[usize; 7]>>();
                base_sets.insert(test_vals, x_actions);  
            }

//  Collect all subsets of available denominations that give one more than the minimum coin count in all transactions
            else if coins - base_coins == 1 {
                let x_actions = disaggs.into_iter()
                .filter(|x| x.iter().sum::<usize>() > base_coins)
                .collect::<Vec<[usize; 7]>>();
                const_sets.insert(test_vals, x_actions);  
            }
        }
    }

//  There are more than one subset of denominations that give the optimal count, so we pick the one that 
//  requires that count for the fewest transactions

    println!("All sets with base number of coins in transaction:");
    let mut base_max: usize = VALS.len();
    let mut base_change: usize = 63;
    let mut base_init = BTreeSet::<usize>::new();
    for set in base_sets {
        println!("  Coins allowed {:?} give {} threshold transactions.", set.0, set.1.len());
        let base_coins = set.0.into_iter().filter(|z| z != &64).collect::<BTreeSet<usize>>();
        if base_coins.len() <= base_max && set.1.len() < base_change {
            base_max = base_coins.len();
            base_change = set.1.len();
            base_init = base_coins;
        }
    }
    println!("Smallest set of denominations to give the smallest number of coins in any transaction is {:?}", base_init);
    println!();

//  The puzzle implies that only one subset of denominations has one fewer than the optimum and produces exactly two
//  transactions requiring more than the baseline number of coins. However, we test all possibilities and in passing 
//  demonstrate that that is the case
    println!("Sets with one of these coins dropped requiring only two transactions with an extra coin:");
    for set in const_sets {
        let reduced_coins = set.0.into_iter().filter(|z| z != &64).collect::<BTreeSet<usize>>();
        let dropped = base_init.difference(&reduced_coins).collect::<Vec<&usize>>();
        if dropped.len() == 1 && set.1.len() == 2 {
            println!("  Coins dropped {:?}", dropped);
            println!("  Coins allowed {:?} give {} threshold transactions.", reduced_coins, set.1.len());
            println!();

//  We have idenitified denomination subsets that meet the puzzle requirements, and now we need to fit them on the rectangle
//  The work is done in the function coin_order, and finally we convert the denominations into diameters for the puzzle answer

            let order = coin_order(&reduced_coins).iter()
                                                    .map(|&&z| (z as f32).powf(0.5) as usize)
                                                    .collect::<Vec<usize>>();
            println!("Optimal coin order {:?}", order);
        }
    }
}

// Recursive approach to building optimal coin sets for all transactions up to value t, given demoninations and a start point
fn disagg(t: usize, precs: &Vec<[usize; 7]>, denoms: [usize; 7]) -> Vec<[usize; 7]> {

//  Initialise values for this recursion, in which we are aiming for a number one larger than the last one we calculated
    let target: usize = precs.len();
    let mut root = precs.to_owned();
    let mut best = target;
    let mut best_vec: [usize; 7] = [0; 7];

//  Work downwards through the coin denominations. For each denomination try every possible count that sums to less
//  than our target, and add this count to the optimum coin set for the remainder
//  The optimal set of coins for this target gives the smallest sum of coin counts
    for (idx, v) in denoms.iter().enumerate().rev() {
        let x_max = target / v;
        if x_max == 0 {continue}

        for x in 1..x_max + 1 {
            let y: usize = precs[target - v * x].iter().sum();

            if x + y <= best {
                best = x + y;
                best_vec = precs[target - v * x].to_owned();
                best_vec[idx] = x;
            }
        }
    }
    root.push(best_vec);


    if target < t {
        root = disagg(t, &root, denoms);
    }
    root
}

// Find the horizontal distance between centres of two touching circles that are also touching the horizontal axis
// Can be derived using Pythagoras
fn centre_dist(rsqa: f32, rsqb: f32) -> f32 {
    2.0 * (rsqa * rsqb).powf(0.25)
}


//  Work out the order of placement on a line, of a set of coins with given areas, to minimise the distance end-to-end
//  Splits the group into large and small, and then places the largest in a line to create interstices, into which we fit the smallest
//  By putting the larger of the small coins into the larger interstices, we minimise the extra space required
//  If there are as many small coins as large, the smallest is put at the end next with the larger gap from the wall
//  All sort orders are controlled so that the result naturally places the smallest coin to the right of the centre, as per puzzle

fn coin_order(s: &BTreeSet<usize>) -> Vec<&usize> {
    let mut chosen = Vec::<&usize>::new();
    let mut smalls = Vec::<&usize>::new();
    for (idx, lge) in s.iter().rev().enumerate() {
        if idx < (s.len() + 1) / 2 {
            if idx % 2 == 0 {
                chosen.push(lge);
            }
            else {
                chosen.insert(0, lge);
            }
        }
        else {
            smalls.push(lge);
        }
    }

    let mut gaps = vec![(vec![&&0, &chosen[0]], (*chosen[0] as f32).powf(0.5))];
    for (idx, pr) in chosen.iter().enumerate() {
        if idx < chosen.len() - 1 {
            gaps.push((vec![&pr, &chosen[idx + 1]], centre_dist(**pr as f32, *chosen[idx + 1] as f32)));
        }
        else {
            gaps.push((vec![pr, &&0], (**pr as f32).powf(0.5)));
        }
    }
    gaps.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());


    println!("Gaps between larger coins {:?}", gaps);

    let avail_gaps = gaps.iter().rev().zip(smalls)
                                                                    .collect::<Vec<(&(Vec<&&usize>, f32), &usize)>>();
    println!("Small coins matched to these gaps {:?}", avail_gaps);

    let mut full_chosen = chosen.clone();
    for small in avail_gaps.iter() {
        let coin = small.1;
        let left_neighbour = small.0.0[0];
        let intl = full_chosen.iter().position(|x| x == left_neighbour).unwrap();
        full_chosen.insert(intl + 1, coin);
    }
    full_chosen

}