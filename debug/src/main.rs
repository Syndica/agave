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


// use {
//     rand::distributions::uniform::{SampleUniform, UniformSampler},
//     // rand::Rng,
//     rand_chacha::{rand_core::SeedableRng, ChaChaRng}
// };

// fn main() {
//     let epoch: u64 = 668;
    
//     let mut seed = [0u8; 32];
//     seed[0..8].copy_from_slice(&epoch.to_le_bytes());

//     let rng = &mut ChaChaRng::from_seed(seed);
//     let sampler = <u64 as SampleUniform>::Sampler::new(0, 225582282719529290);

//     for _ in 0..10000 {
//         let sample: u64 = sampler.sample(rng);
//         println!("{}", sample);
//     }
// }

use std::str::FromStr;
use bincode::serialize;
use solana_sdk::hash::Hash;
use solana_sdk::{ transaction, message};
use solana_sdk::system_instruction::transfer;

fn main() {
    let from_keypair = solana_sdk::signature::Keypair::from_bytes(&[76, 196, 192, 17, 40, 245, 120, 49, 64, 133, 213, 227, 12, 42, 183, 70, 235, 64, 235, 96, 246, 205, 78, 13, 173, 111, 254, 96, 210, 208, 121, 240, 159, 193, 185, 89, 227, 77, 234, 91, 232, 234, 253, 119, 162, 105, 200, 227, 123, 90, 111, 105, 72, 53, 60, 147, 76, 154, 44, 72, 29, 165, 2, 246]).unwrap();
    let from = solana_sdk::pubkey::Pubkey::from_str("Bkd9xbHF7JgwXmEib6uU3y582WaPWWiasPxzMesiBwWm").unwrap();
    let to = solana_sdk::pubkey::Pubkey::from_str("GDFVa3uYXDcNhcNk8A4v28VeF4wcMn8mauZNwVWbpcN").unwrap();
    let lamports = 100;
    let recent_blockhash = Hash::from_str("39AbEHBaSrbRH8dGN2LkYg4rDGXU1ziCQMVvASsiWNSN").unwrap();
    let instruction = transfer(&from, &to, lamports);
    let mut message = message::Message::new(&[instruction], Some(&from));
    message.recent_blockhash = recent_blockhash;
    let transaction = transaction::VersionedTransaction::try_new(message::VersionedMessage::Legacy(message), &[from_keypair]).unwrap();
    
    let serialized = serialize(&transaction).unwrap();
    let encoded = bs58::encode(serialized).into_string();

    println!("Transaction: {:?}", transaction);
    println!("Encoded Transaction: {}", encoded);

    // println!("TRANSACTION");
    // println!("Message Bytes: {:?}", &transaction.message_data());
    // for s in transaction.signatures.iter() {
    //     println!("Signature: {}", s);
    // }
    // println!("MessageHeader: {:?}", &transaction.message.header);
    // for k in &transaction.message.account_keys {
    //     println!("AccountKey: {}", k);
    // }
    // println!("RecentBlockhash: {}", &transaction.message.recent_blockhash);
    // for i in &transaction.message.instructions {
    //     println!("Instruction: {:?}", i);
    // }

    // println!("Transaction Bytes: {:?}", serialize(&transaction));

    // let transaction_bytes: [u8; 215] = [1, 244, 3, 204, 238, 82, 27, 27, 162, 72, 117, 247, 12, 16, 239, 171, 168, 8, 17, 8, 49, 180, 220, 216, 44, 243, 97, 10, 235, 226, 17, 51, 141, 124, 242, 151, 17, 72, 123, 147, 95, 119, 197, 202, 139, 67, 82, 166, 74, 78, 79, 81, 55, 22, 98, 34, 97, 60, 195, 212, 77, 26, 24, 91, 7, 1, 0, 1, 3, 159, 193, 185, 89, 227, 77, 234, 91, 232, 234, 253, 119, 162, 105, 200, 227, 123, 90, 111, 105, 72, 53, 60, 147, 76, 154, 44, 72, 29, 165, 2, 246, 3, 229, 144, 40, 5, 196, 50, 27, 92, 158, 34, 76, 254, 41, 252, 235, 115, 67, 211, 94, 236, 153, 105, 36, 107, 71, 146, 102, 126, 58, 45, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 147, 32, 255, 65, 72, 38, 140, 55, 134, 52, 46, 246, 178, 29, 156, 68, 60, 135, 170, 247, 210, 39, 61, 211, 114, 241, 44, 59, 107, 253, 253, 1, 2, 2, 0, 1, 12, 2, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0];
    // let deserialized_transaction: transaction::VersionedTransaction = bincode::deserialize(&transaction_bytes).unwrap();
    
    // println!("Transaction Bytes: {:?}", serialize(&deserialized_transaction));

    // println!("DESERIALIZED TRANSACTION");
    // println!("Message Bytes: {:?}", &deserialized_transaction.message.serialize());
    // for s in deserialized_transaction.signatures.iter() {
    //     println!("Signature: {}", s);
    // }
    // println!("MessageHeader: {:?}", &deserialized_transaction.message.header());
    // for k in deserialized_transaction.message.static_account_keys() {
    //     println!("AccountKey: {}", k);
    // }
    // println!("RecentBlockhash: {}", &deserialized_transaction.message.recent_blockhash());
    // for i in deserialized_transaction.message.instructions() {
    //     println!("Instruction: {:?}", i);
    // }

}
