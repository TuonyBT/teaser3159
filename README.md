# teaser3159

Sunday Times Teaser from 9 April 2023

Routine first finds the optimum subset of all available denominations that can produce all required transaction values with the fewest coins.
This takes a recursive approach to breaking each value down, testing combinations of individual denominations and already-established optimal coin-sets for smaller values

Once we have this optimum subset, we look for ways to produce the same maximum number of coins for any transaction, but with fewer available denominations.
This would be the King's initial choice.

We also look for subsets that have one denomination less than the King's choice, but have a coin count larger than optimal for only two transactions.
We do this in a way that demonstrates that there is only one such subset, which must be the one suggested by the King's treasurers.

Given this sub-set, we simulate the King's layout on his rectangular board, but interweaving the smaller coins among the larger ones in a form of close-packing.
The general principle is that the distance required to fit two coins, as a proportion of the sum of their diameters, reduces as the difference in their diameters increases.
