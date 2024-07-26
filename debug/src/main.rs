// use solana_runtime::snapshot_bank_utils::bank_fields_from_snapshot_archives;
// use solana_accounts_db::accounts_file::StorageAccess;
// use solana_ledger::leader_schedule::LeaderSchedule;
// use solana_sdk::pubkey::Pubkey;

// fn main() {
//     let epoch: u64 = 668;
    
//     let mut seed = [0u8; 32];
//     seed[0..8].copy_from_slice(&epoch.to_le_bytes());
    
//     let bank_fields = bank_fields_from_snapshot_archives(
//         "/home/hnewman/Documents/work/syndica/snaps/",
//         "~/does-not-exist/",
//         StorageAccess::Mmap
//     ).expect("Failed to load bank fields from snapshot archives");
    
//     let slots = bank_fields.epoch_schedule.get_slots_in_epoch(epoch);
    
//     let mut stakes = bank_fields.epoch_stakes
//         .get(&epoch)
//         .expect("Failed to get epoch from epoch stakes")
//         .stakes()
//         .staked_nodes()
//         .iter()
//         .map(|(pubkey, stake)| (*pubkey, *stake))
//         .collect();
//     sort_stakes(&mut stakes);

//     // println!("Epoch: {}", epoch);
//     // println!("Seed: {:?}", seed);
//     // println!("Slots in epoch: {}", slots);
//     // println!("Stakes:");
//     // for (pubkey, stake) in stakes.iter() {
//     //     println!("{}: {}", pubkey, stake);
//     // }
    
//     let leader_schedule = LeaderSchedule::new(
//         &stakes,
//         seed,
//         slots,
//         4,
//     );

//     println!("Leader Schedule:");
//     let leaders = leader_schedule.get_slot_leaders();
//     let leaders_size = leaders.len();
//     for i in leaders_size-50..leaders_size {
//         println!("Leader {}: {}", i, leaders[i]);
//     }
// }

// fn sort_stakes(stakes: &mut Vec<(Pubkey, u64)>) {
//     // Sort first by stake. If stakes are the same, sort by pubkey to ensure a
//     // deterministic result.
//     // Note: Use unstable sort, because we dedup right after to remove the equal elements.
//     stakes.sort_unstable_by(|(l_pubkey, l_stake), (r_pubkey, r_stake)| {
//         if r_stake == l_stake {
//             r_pubkey.cmp(l_pubkey)
//         } else {
//             r_stake.cmp(l_stake)
//         }
//     });

//     // Now that it's sorted, we can do an O(n) dedup.
//     stakes.dedup();
// }


use {
    rand::distributions::uniform::{SampleUniform, UniformSampler},
    // rand::Rng,
    rand_chacha::{rand_core::SeedableRng, ChaChaRng}
};

fn main() {
    let epoch: u64 = 668;
    
    let mut seed = [0u8; 32];
    seed[0..8].copy_from_slice(&epoch.to_le_bytes());

    let rng = &mut ChaChaRng::from_seed(seed);
    let sampler = <u64 as SampleUniform>::Sampler::new(0, 225582282719529290);

    for _ in 0..10000 {
        let sample: u64 = sampler.sample(rng);
        println!("{}", sample);
    }
}