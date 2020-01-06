# Compile Time Checks

Review this entire chapter and try to make directions, seems valuable

```rust
trait Referendum {
    type State: VotingPeriod;
    fn execute(Self) -> Self::State;
}

pub trait VotingPeriod {
    type F: VotePeriod;
    fn vote_logic(self);
}

struct Voting;
struct Gracing;
struct Executing;

impl Referendum for Voting {
    type next = Gracing;

    fn execute(self) -> Self::Next {
        unimplimented!();
    }
}
/// more like this...

impl<S> VotingPeriod for S
    where
        S: Referendum
    {
        fn vote_logic(self) {
            unimplemented!(); 
        }
    }
```