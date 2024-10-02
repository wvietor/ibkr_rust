# ibkr-tws-api
Rust port of Interactive Brokers' Trader Workstation API

# IBKR Platform Compatibility
Most of the functionality should work on any API version >10.16. However, there are some issues with some of the functionality (notably, executions and position_summary) with older versions. At present, tests are passing with API version 10.31. In general, the "Latest" version as specified on [IBKR 's website](https://interactivebrokers.github.io) will be prioritized.

# Important Notes
The `local` client types are currently unstable due to a weird bug where a tokio channel somehow gets closed. I'm working to fix the bug and will update this when done.
