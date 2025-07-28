use super::state::RaftState;
use rand::{rng, Rng};
use std::time::Duration;
use tokio::{select, time};

pub struct RaftNode {
    pub state: RaftState,
    pub term: u64,
}

impl RaftNode {
    pub fn new() -> Self {
        Self {
            state: RaftState::Follower,
            term: 0,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                RaftState::Follower => self.run_follower().await,
                RaftState::Candidate => self.run_candidate().await,
                RaftState::Leader => self.run_leader().await,
            }
        }
    }

    async fn run_follower(&mut self) {
        println!("State: Follower, Term: {}", self.term);
        let timeout = rand_timeout();
        time::sleep(timeout).await;

        println!("Timeout! Becoming Candidate");
        self.state = RaftState::Candidate;
    }

    async fn run_candidate(&mut self) {
        self.term += 1;
        println!("State: Candidate, Term: {}", self.term);
        println!("(Simulated) Voted for self");

        // In a real system, you'd request votes from other nodes
        time::sleep(Duration::from_millis(500)).await;

        println!("(Simulated) Won election, becoming Leader");
        self.state = RaftState::Leader;
    }

    async fn run_leader(&mut self) {
        println!("State: Leader, Term: {}", self.term);
        let mut interval = time::interval(Duration::from_millis(300));

        loop {
            select! {
                _ = interval.tick() => {
                    println!("Leader heartbeat (Term {})", self.term);
                }
                _ = time::sleep(rand_timeout()) => {
                    println!("Simulated failure or partition. Reverting to Follower.");
                    self.state = RaftState::Follower;
                    break;
                }
            }
        }
    }
}

fn rand_timeout() -> Duration {
    let ms = rng().random_range(1500..3000);
    Duration::from_millis(ms)
}

