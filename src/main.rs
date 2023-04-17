use itertools::Itertools;
use std::collections::{HashMap, BTreeSet};

const VALS: [usize; 7] = [1, 4, 9, 16, 25, 36, 49];

fn main() {

    let seed: Vec<[usize; 7]> = vec![[0;7], [1, 0, 0, 0, 0, 0, 0]];
    let disaggs = disagg(63, &seed, VALS);

    let base_coins = disaggs.iter().fold(0, |acc, x| acc.max(x.iter().sum::<usize>()));
    println!("Base number of coins required for a transaction with all denominations {}", base_coins);
    println!("");

    let mut base_sets: HashMap<[usize; 7], Vec<[usize; 7]>> = HashMap::new();
    let mut const_sets: HashMap<[usize; 7], Vec<[usize; 7]>> = HashMap::new();

    for no_deleted in 1..6 {
        for supps in (1..7).combinations(no_deleted) {
            let mut test_vals = VALS.to_owned();
            for idx in supps {
                test_vals[idx] = 64;
            }
            
            let disaggs = disagg(63, &seed, test_vals);

            let coins = disaggs.iter().fold(0, |acc, x| acc.max(x.iter().sum::<usize>()));

            if coins - base_coins == 0 {
                let x_actions = disaggs.into_iter()
                .filter(|x| x.iter().sum::<usize>() == base_coins)
                .collect::<Vec<[usize; 7]>>();
                base_sets.insert(test_vals, x_actions);  
            }
            else if coins - base_coins == 1 {
                let x_actions = disaggs.into_iter()
                .filter(|x| x.iter().sum::<usize>() > base_coins)
                .collect::<Vec<[usize; 7]>>();
                const_sets.insert(test_vals, x_actions);  
            }
        }
    }

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

    println!("Sets with one of these coins dropped requiring only two transactions with an extra coin:");
    for set in const_sets {
        let reduced_coins = set.0.into_iter().filter(|z| z != &64).collect::<BTreeSet<usize>>();
        let dropped = base_init.difference(&reduced_coins).collect::<Vec<&usize>>();
        if dropped.len() == 1 && set.1.len() == 2 {
            println!("  Coins dropped {:?}", dropped);
            println!("  Coins allowed {:?} give {} threshold transactions.", reduced_coins, set.1.len());
            println!();

            let order = coin_order(&reduced_coins);
            println!("Optimal coin order {:?}", order);
        }
    }
}


fn disagg(t: usize, precs: &Vec<[usize; 7]>, denoms: [usize; 7]) -> Vec<[usize; 7]> {

    let target: usize = precs.len();
    let mut root = precs.to_owned();
    let mut best = target;
    let mut best_vec: [usize; 7] = [0; 7];

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

fn centre_dist(rsqa: f32, rsqb: f32) -> f32 {
    2.0 * (rsqa * rsqb).powf(0.25)
}

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
    println!("Smaller coins {:?}", smalls);

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

    let mut avail_gaps = gaps.to_owned();
    for small in smalls.iter().rev() {
        for gap in avail_gaps.iter().rev() {
            let new_gap = centre_dist(**small as f32, **gap.0[0] as f32) + centre_dist(**small as f32, **gap.0[1] as f32);
            println!("Small coin {} in gap {:?} creates new gap {}", small, gap, new_gap);


        }
    }

    chosen

}