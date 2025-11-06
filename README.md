# wsmd

This repository is part of a group Rust learning session. 
The goal is to collaboratively build and understand a simple real-time system: a webSocket server that performs computation and streams data, and a Bevy client that receives and visualizes the updates.

We work incrementally, discuss Rust concepts as we encounter them, and learn the ecosystem through hands-on experience. 
The focus is understanding fundamentals, not producing production-grade code.

This project is intentionally modest and iterative. We introduce concepts only when needed and build our understanding step by step.

## Project Overview

We are building the following architecture:

```
[ WebSocket Server (Rust + Tokio) ]  --->  [ Bevy Client (vizmat) ]
      heavy computation                        visualization
      sends data continuously                 displays updates
```

For the first milestone, the server generates random numbers and sends them over WebSocket. The client receives these values and displays them in a Bevy text element. This establishes basic data flow and integration between an async server and a Bevy rendering application.

Later milestones will extend this pipeline to handle structured simulation data, update entities in the scene on the bevy app we did in the previous learning session [vizmat](https://github.com/rs4rse/vizmat), and support bidirectional communication.


## Running the System

Start the server:

```
cargo run --bin viz-server
```

Then launch the client:

```
cargo run --bin viz-client
```

## Milestones

1. basic websocket scanfold, server streams random numbers, client displays them in stdout.
1. have bevy client (desktop) can display the update in a text box.
1. represent simulation states (trajactories)  and update Bevy entities
1. Add client-to-server messaging when needed for interaction (some restapi here to trigger or all websocket?)
1. Replace random data with meaningful compute, using a md library to run actual simulation.

## Contributing

We are at an early stage and welcome contributions.
If you are new to Rust, Bevy, or visualization, this is the perfect playground.

## License

All contributions must retain this attribution.

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
