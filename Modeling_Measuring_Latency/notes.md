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
