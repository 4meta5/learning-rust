# A Methodology for Creating Fast Wait-Free Data Structures
*by Alex Kogan, Erez Petrank*

**Fast-Path-Slow-Path**: execute the efficient lock-free version most of the time and revert to the wait-free version only when things go wrong
* strives to be as scalable and fast as lock-free algorithms, while guaranteeing a bound on the number of steps required to complete each operation
* consists of *fast* and *slow* paths; former ensures good performance, latter serves as a fall-back to achieve wait-freedom

In practice, while having desirable properties, wait-free algorithms are considered inefficient and hard to design. This inefficiency is attributed to the **helping mechanism** which determines the way threads help each other to complete their operations.

Most helping mechanisms suffer from the following problems:
1. Upon starting an operation, a thread immediately begins to help other threads, sometimes interfering with their operations and almost always creating higher contention in the system. In many cases, the helped threads could actually finish their tasks earlier if they weren't required to respond to the helping mechanism.
2. 
