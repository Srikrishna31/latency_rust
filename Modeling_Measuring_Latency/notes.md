## Laws of latency

Two fundamental theoretical frameworks provide essential guidance: Little's Law, which describes the relationship between concurrency, latency and throughput, and Amdahl's Law, which quantifies the theoretical speedup possible through parallelism.
These principles serve as the essential tools for establishing realistic latency expectations for a system.

### Little's Law

We use Little's Law to relate concurrency, latency and throughput:

$$
T = \frac{L}{C}
$$

where $T$ is throughput, $L$ is latency and $C$ is concurrency.

In the equation, C signifies the system's concurrency level, T represents throughput, and L represents average latency.
Concurrency typically refers to the number of events or operations that can occur at the same time.
For example, if you have a system with a throughput of 1000 requests/secont and an average latency of 50ms, the number of events or operations that can occur at the same time is, as per Little's Law, 1000 requests/second * 50ms which is a concurrency of 50 requests.

As we are measuring latency, we are interested in the average time it takes for a request to be processed, not the number of requests that can be processed at the same time. The equation can be rewritten as:

$$
L = \frac{T}{C}
$$

As you can see, latency increass as concurrency increases, which is intuitive: the more work you have to do (concurrency), the longer it takes to process each request(average latency). As concurrency increases, you must reduce latency to improve throughput (and vice versa) or
manage the queue length.

A concrete way to visualize Little's law is through processing work in queues. Figure 2.2 shows a queue with two tasks arriving per time step and a system processing at a concurrency of one. In the first time step, the queue has two tasks per time unit, and the processor executes the first task. In the second time step, the queue has two new tasks, totaling three tasks, and the processor executes the second task that arrived in the first time step, and so on. In the fourth time step, no new items arrive in the system; otherwise the queue would keep growing indefinitely. From the pov of Little's law, latency is fixed at one timestep, and th

![Figure 2.2 Work arriving on a queue at a rate of two tasks per time step with system processing at a concurrency of one](./Figure%202.2%20Work%20arriving%20on%20a%20queue%20at%20a%20rate%20of%20two%20tasks%20per%20time%20step%20with%20system%20processing%20at%20a%20concurrency%20of%20one.png)

Figure 2.3 shows the same scenario, but this time with two processors executing the tasks. Two tasks arrive in the system in all the time steps, and the processors execute both. As in previous figure, the latency is one time step, but throughput is doubled to two tasks per time step, resulting in a concurrency of two. As this is the same as the rate of tasks arriving in the system, the queue length remains constant. You would also get the same result if you reduced latency to half a time step, meaning that one processor could execute two tasks per time step.

![Figure 2.3 Work arriving on a queue at a rate of two tasks per time step with system processing at a concurrency of two](./Figure%202.3%20Work%20arriving%20on%20a%20queue%20at%20rate%20of%20two%20tasks%20per%20time%20step%20with%20system%20processing%20at%20a%20concurrency%20of%20two.png)

As you can see, Little's law is an excellent tool for modeling the relationship between latency, throughput, and concurrency in a system. Still, as with most models, there are cases where it is inaccurate. One primary assumption behind Little's law is that the mean throughput and latency are independent over time. However, latency tends to increase in real-world systems due as concurrency increases because of various factors such as context switthing, lock serialization, and algorithmic complexity. The variability in latency can result in an underestimation of concurrency when applying Little's law in real-world scenarios.

## Amdahl's Law

Amdahl's law is a framework that shows you the theoretical speedup when parallelizing the execution of a fixed-size problem. The law is defined as follows:

$$
speedup = \frac{1}{1 - P + \frac{P}{N}}
$$

where $P$ is the part of your algorithm that can run in parallel, $N$ is the number of execution units, such as CPUs, that can execute in parallel.

![Figure 2.4 Amdahl's law illustrated. The graph shows theoretical speedup (y axis) as a function of parallelism (x axis) given how much of the program is parallelizable](./Figure%202.4%20Amdahl%27s%20law%20illustrated.%20The%20graph%20shows%20theoretical%20speedup%20%28y%20axis%29%20as%20a%20function%20of%20parallelism%20%28x%20axis%29%20given%20how%20much%20of%20the%20program%20is%20parallelizable.png)

Figure 2.4 shows examples of theoretical speedup potential. If 50% of your application logic is parallelizable, the maximum speedup is 2x. The speedups level off quickly because you get diminishing returns as you increase the number of execution units (benefits upto 8 cores). If 95% of your application logic is parallelizable, you can achieve speedups of up to 200 cores before diminishing returns set in.
As an example, consider a backend endpoint that processes customer orders with the following steps:

1. Request parsing and validation: 5ms
2. Database lookup (query database for shipping address and pricing): 15ms
3. Order calculation and insertion: 30ms
   Total time to execute sequentially: 30ms

Step 2 has a dependency on step 1, because we need to parse and validate the request before querying the database. Similarly, step 3 is dependent on step 2 because we cannot perform order calculations before we know the price. However, we can parallelize the database lookup in step 2 by running different queries in parallel or simply increasing the number of CPUs allocated to the database for query execution. We therefore have P = 0.3 (30% of the application logic is parallelizable) (15ms out of 50ms).

Plugging into Amdahl's Law, we can see that with 2CPUs, we achieve a modest speedup of 1.2. As we continue to increase the CPU count, we reach a speedup of 1.35 at 8 CPUs, and then hit a ceiling of 1.4 at 16 CPU, after which adding more CPUs does not yield further speedup. That's because the serial execution time of steps 1 and 3, which include parsing, validation, and order calculation, dominates the runtime. Furthermore, it's not immediately apparent whether the benefits of 8CPUs are worth it, as you need to double the number of CPUs for very modest gains.

A key takeaway from Amdahl's law is that increasing parallelism does not automatically reduce latency, and even when it does, the benefits have diminishing returns. However, in cases where your application logic is highly parallelizable, such as in a batch processing or data analysis application, there's a clear latency advantage to breaking up work across and executing in parallel.

## Latency Distribution

In reality, latency is a distribution, which turns out to be really important. Most of the time, our systems have latency variability, meaning that latency differs depending on the state of the system and the workload. For example, latency can also increase due to algorithmic complexity, queuing, and other factors as throughput increases. If we report only the average latency, we hide the variability with a number that doesn't reflect how the system behaves. Minimum and maximum latency also mislead you on how the system behaves because they represent the two extremes of the latency distribution: best-case and worst-case.

That's why, when building for low latency, you must seek to understand the latency distribution, particularly the `tail latency`. Typically, we talk about latency percentiles when describing a latency-sensitive system: the 95th, 99th, or 99.9th percentile latency, representing that 95%, 99%, or 99.9% of requests have. Sometimes, the 100th percentile latency is also reported, representing the maximum latency observed. These high percentiles are called `tail latency` because the percentiles are at the tail end of the latency distribution, where the latency is higher than expected. It's easy to dismiss tail latency as an outlier or benchmark noise, but users experience these outliers much more often than you might expect.

You can reduce latency by executing subtasks of your service work in parallel, but that means your service latency will be bounded by the slowest subtask, not the overall latency of your service. How dominant is tail latency? Exploring an example from "Dean and Barrow's" "Tail at Scale" paper, where you have a service that sends subrequests to multiple servers in parallel and has to wait for all of them to respond to complete the request. Typically, each subrequest takes only ten milliseconds to respond, but there is an occasional delay, resulting in a tail latency of one second. In a situation where only 1 out of every 100 requests experiences this delay, and with a fanout count of 100, approximately 63% of users will encounter service latency exceeding 1 second, as shown in the figure below. Interestingly, even when such delays are much rarer, with only 1 out of 10,000 requests affected, with a fanout count of 2000, approximately 18% of users will still experience this tail latency.

![Tail Latency illustrated](./Figure%202.5%20Tail%20latency%20illustrated.png)

**NOTE**: When we're processing a request and need to communicate with N other components, we call N the `fanout count`. It represents how much we can fan out work to other components. From a latency perspective, if we have to wait for all those components to finish before we can compute the request processing, the fanout count impacts tail latency.

The key thing to remember is that users are much more likely to experience the tail latency of your component than you might intuitively think. Tail latency exists because of variability in your component's performance.

## Common sources of latency

There are a wide variety of known sources, ranging from global and local networking, resource sharing, background and maintenance tasks, queuing power limits and saving optimizations, interrupt processing, non-uniform memory access (NUMA) effects, garbage collection, and more.

### Physics

The lowest-level source of latency is the speed of light, which determines the maximum speed at which information can travel. Specifically, you have to consider the geographical distance between the different components of your system. Application locig can run on a serverless platform, close to your user. But databases are still usually provisioned in the cloud, far from your user, which means any business logic that needs to fetch data is latency-bound by the time required for information to travel form the database to the application.

### CPU and hardware

The CPU and hardware can be significant sources of latency variance, impacting tail latency, because of all the optimization they employ to make our systems typically run faster. The latency variability caused by the CPU and hardware can be particularly challenging when building for low latency because the software has no direct control over the hardware. Instead, as a programmer, you need to be aware of these things as you write and tune your code. There are many sources of latency, but the main ones to focus on are CPU cache hierarchy, CPU speculative excecution, and CPU frequency scaling and hardware power saving.

**Simultaneous multithreading (SMT)**

Simultaneous multithreading (SMT), also known as hyperthreading, is a technique that allows multiple threads to run on a single CPU core. This happens by duplicating architectural state (registers, program counters) while sharing execution resources. This can improve performance by allowing the CPU to switch between threads more quickly, but it can also increase latency, as the CPU must switch between threads more often. From an operating system perspective, a physical CPU core is partitioned into multiple logical cores, allowing independent thread execution. However, SMT can harm latency due to resource contention as logical cores compete for the shared physical execution units, memory bandwidth, and caches. A thread running on a logical core can stall another thread by heavily using resources. SMT represents a fundamental tradeoff: improved throughput and efficiency versus potential latency degradation for individual threads. For ultra-low latency applications, consider disabling SMT.

---
