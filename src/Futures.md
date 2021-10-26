## Futures

So what is a future?

A future is a representation of some operation which will complete in the future.

Async in Rust uses a Poll based approach, in which an asynchronous task will have three phases.

- The Poll phase. A Future is polled which results in the task progressing until a point where it can no longer make progress. We often refer to the part of the runtime which polls a Future as an executor.
- The Wait phase. An event source, most often referred to as a reactor, registers that a Future is waiting for an event to happen and makes sure that it will wake the Future when that event is ready.
- The Wake phase. The event happens and the Future is woken up. It's now up to the executor which polled the Future in step 1 to schedule the future to be polled again and make further progress until it completes or reaches a new point where it can't make further progress and the cycle repeats.

## Implementing our own Futures

### The Executor
The executors responsibility is to take one or more futures and run them to completion.
The first thing an executor does when it gets a Future is polling it.

When polled one of three things can happen:
- The future returns Ready and we schedule whatever chained operations to run
- The future hasn't been polled before so we pass it a Waker and suspend it
- The futures has been polled before but is not ready and returns Pending