const HALVING_PERIOD: u32 = 210000;     // halving of block rewards happen every 210000 blocks (almost every 4 years at current pace)
const INITIAL_REWARD_SIZE: u128 = 50*100000000;     // initial block reward is 50 btc - every btc equals 100M satoshi

fn main() {

    let mut current_reward_size = INITIAL_REWARD_SIZE;
    let mut total_rewards: u128 = 0;
    let mut block_number: u32 = 0;
    let mut halving_number: u16 = 0;

    loop {
        current_reward_size = INITIAL_REWARD_SIZE >> halving_number;
        total_rewards += current_reward_size;
        block_number += 1;

        println!("block\t#{}, rewarded\t{}", block_number, current_reward_size);

        if block_number % HALVING_PERIOD == 0 {
            // every 210000 block, halving happens
            halving_number += 1;
            println!("halving occurred!");
        }
        if current_reward_size == 0 {
            // at some point block reward size will reach 0, we want to break the loop at that point
            break;
        }
    }

    println!("sum of mined rewards until block #{}: {}", block_number, total_rewards);
    println!("total halving: {}", halving_number);
}