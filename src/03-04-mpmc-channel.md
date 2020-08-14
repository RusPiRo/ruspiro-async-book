## Multi Producer Multi Consumer (MPMC) Channel

The MPMC channel allows adding new entries from any core or thread (multi producer), picking the next entry by any core or thread (multi consumer) and works like a FIFO queue. There are for sure multiple approaches possible to implement such a channel in a non-blocking way. I will present quite a simple one here that has proven to work at least for all my current use cases in a bare metal ``no_std`` environment. The source code can be found [here](https://github.com/RusPiRo/ruspiro-channel).

