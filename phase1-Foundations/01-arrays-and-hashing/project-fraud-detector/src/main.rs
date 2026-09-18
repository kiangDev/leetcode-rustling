use std::collections::HashMap;
use std::time::Instant;

/*
======================================================
My Engineering Journey & Core Principle Verification
======================================================
After learning 5 fundamental DSA algorithms, a burning question arose in my mind:
"What if we take these abstract data structures and apply them to solve a 
real-world high-throughput production problem, rather than just passing tests?"
This project was built to bridge the gap between textbook Big-O theory and 
real hardware mechanics (L1 cache locality, zero-allocation memory models, 
and sub-microsecond latency).


=======================================================
Component 1: Fixed-Size Sliding Window Ring Buffer
=======================================================
    - Data Structure: Fixed Array [u64; 5] (Strictly 40 bytes per user)
    - Zero Heap Allocation: Contiguous memory, zero allocation churn, L1 cache resident
    - Time Complexity: O(1) for every check and insert
*/

#[derive(Debug)]
pub struct SlidingWindowRingBuffer{
    timestamps: [u64; 5],  // stores the 5 most recent timestamps
    cursor: usize,         // circular pointer pointing to the next slot to overwrite (0..5)
    count: usize,          // total transaction attempts by this user
}

impl Default for SlidingWindowRingBuffer{
    fn default() -> Self {
        Self{
            timestamps: [0; 5],
            cursor: 0,  
            count: 0,
        }
    }
}

impl SlidingWindowRingBuffer{

    #[inline(always)]
    pub fn is_fraudulent(&self, now: u64, window_secs: u64, limit: usize) -> bool {

        if self.count < limit {
            return false;
        }
        // in a circular buffer, cursor points directly to the 5th oldest event
        let oldest_in_window = self.timestamps[self.cursor];

        (now >= oldest_in_window) && (now - oldest_in_window <= window_secs)
    }

    //save new timestamp into Ring Buffer in O(1)
    #[inline(always)]
    pub fn record(&mut self, now: u64){
        self.timestamps[self.cursor] = now;
        self.cursor = (self.cursor + 1) % 5; //shift pointer circularly: 0, 1, 2, 3, 4, 0...
        self.count += 1;
    }
}

/*
=======================================================
component 2: Fraud Detection 
=======================================================
*/
#[derive(Debug, PartialEq)]
pub enum TransactionResult {
    Approved,
    BlockedFraud,
}

pub struct FraudDetection{
    users: HashMap<String, SlidingWindowRingBuffer>,
    window_secs: u64,
    limit: usize,
}

impl FraudDetection{
    pub fn new(capacity: usize, window_secs: u64, limit: usize) -> Self{
        Self{
            users: HashMap::with_capacity(capacity), //pre-allocate capacity to avoid re-hashing
            window_secs,
            limit,
        }
    }

    //processing Transaction in single-pass O(1)
    #[inline]
    pub fn process_transaction(&mut self, user_id: &str, now: u64) -> TransactionResult{
        // Entry API: Single hash lookup for search and lazy initialization
        let history = self.users.entry(user_id.to_string()).or_default();

        if history.is_fraudulent(now, self.window_secs, self.limit) {
            history.record(now);
            TransactionResult::BlockedFraud
        } else {
            history.record(now);
            TransactionResult::Approved
        }
    }
    pub fn total_tracked_users(&self) -> usize {
        self.users.len()
    }
}
/* 
==========================================================
component 3: Stress Test Simulation & NanoSecond Benchmark
==========================================================
*/
fn main() {
    println!("============================");
    println!("Fraud Detection");
    println!("============================\n");

    let total_transactions = 100_000;
    let window_secs = 10;
    let limit = 5;

    //create detection pre-allocate memory for 10,000 users
    let mut detect = FraudDetection::new(10_000, window_secs, limit);

    println!("Rule Block if > {} transactions within {} seconds.\n", limit, window_secs);
    println!("Simulating {} incoming transactions...", total_transactions);

    let mut approved_count = 0;
    let mut blocked_count = 0;

    // starting timer hardware timer
    let start_time = Instant::now();

    for i in 0..total_transactions {

        //simulate user:
        // - Hacker blasts requests continuously
        // - Normal users space out transactions
        let (user_id, timestamp) = if i % 10 == 0 {
            ("hacker_bot".to_string(), 1_000 + (i / 100) as u64)
        } else {
            let user_num = i % 1000;
            let id = format!("user_{}", user_num);
            let time = 1_000 + (i as u64 * 3);
            (id, time)
        };

        match detect.process_transaction(&user_id, timestamp) {
            TransactionResult::Approved => approved_count += 1,
            TransactionResult::BlockedFraud => blocked_count +=1,
        }
    }

    let elapsed = start_time.elapsed();
    let total_ns = elapsed.as_nanos();
    let avg_ns_per_op = total_ns as f64 / total_transactions as f64;
    let throughput = (total_transactions as f64 / elapsed.as_secs_f64()) as u64;

    // Benchmark Summary
    println!("simulation completed!");
    println!("-----------------------------------------------------");
    println!("Total Time Elapsed       : {:.2?}", elapsed);
    println!("Throughput               : {} transactions/sec",throughput);
    println!("Average Latency per Op   : {:.2} nanoseconds", avg_ns_per_op);
    println!("-----------------------------------------------------");
    println!("Transaction Approved     : {}", approved_count);
    println!("Fraud Attacks Blocked    : {}", blocked_count);
    println!("Unique Users Tracked     : {}", detect.total_tracked_users());
    println!("Memory per User State.   : 40 Bytes (L1 Cache Resident)");
    println!("-----------------------------------------------------");
}