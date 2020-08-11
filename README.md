# Sarga

My attempt to make some kind of API-grouping server based on the [Saga pattern](https://youtu.be/xDuwrtwYHu8)

Since it's written in Rust, the name must somehow include an "r", for obvious reasons.

## Goals

- Get sequence of instructions (saga)
  Every instruction has:
  - Something to be accomplished
  - A way to undo it (when it needs to be rolled back)
- Execute the tasks in order
- Abort tasks when needed
  - Backward recovery
    - Undo all instructions (using the provided undo instructions)
    - Choose when a best-effort approach is sufficient
  - Forward recovery
    - Execute the saga again
    - Choose when the result must be successful
- Tasks can be
  - Making an HTTP(S) request (webhooks, etc...)
  - Write a file
  - Run a callback function
