# Motion controller test

- A main program that
  - Loads all plugins into their appropriate slots and sets up messaging between them
  - Orchestrates starting/stopping and pulling motion commands from the input parser
- A plugin that produces motion commands (standin for GCode parser)
- A plugin that queues up motion commands and produces a trajectory (standin for proper traj
  planner)
- A plugin that just logs things to the terminal (standin for a UI)

## Vague ideas

- Can have multiple of each kind of plugin (e.g. local UI and WSS server)
