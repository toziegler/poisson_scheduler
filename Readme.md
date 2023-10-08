
# PoissonScheduler

`PoissonScheduler` provides a mechanism to execute a given function/closure at timestamps determined by a Poisson process.
This is particularly useful in simulation and testing scenarios where events or requests need to be generated at a rate
that follows the Poisson distribution.

The Poisson process is a stochastic model that describes events that occur independently and at a fixed average rate.
It's a popular model in various fields, including computer science, telecommunications, and finance.

Learn more about the importance and applications of this approach from [this article](https://www.scylladb.com/2021/04/22/on-coordinated-omission/).
## Examples

```
use poisson_scheduler::PoissonScheduler;
use std::time::Instant;
use std::time::Duration;

let rate = 100.0; // 100 events per second
let mut scheduler = PoissonScheduler::new(rate);

scheduler.run(Duration::new(1,0), |timestamp| {
    println!("Event scheduled at {:?}", timestamp);
});
```

