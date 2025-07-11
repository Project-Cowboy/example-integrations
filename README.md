# Cowboy Example Integrations

### Quick Start
1. Run
```shell
docker run -p 1881:1881 --platform linux/amd64   ghcr.io/project-cowboy/cowboy-prover:latest
```
2. Separate terminal:
```shell
cargo run run
```

### Guide to the Examples
The Cowboy example integrations showcase how integrations can be built on top of the Cowboy network to get authenticated web data, and compute on top of it. Each integration example is made up of a RISC Zero Guest program, and native code for interacting with it, or sending the program to the local prover for generating a zk proof.

As a starting point, we recommend first checking out the x-account example, due to the simplicity, and fewer steps involved in proving.

#### Capabilities
 Examples showcase a number of capabilities for developers building on Cowboy:
- Composition, or verifying a previous integration's proof within your program
- Executing statements on authenticated web data in zero knowledge(e.g. I can prove I am over 21, without revealing my age, given some request to a government website)
- Outputting data from an integration's execution for onchain consumption, or consumption from someone else's proof.

#### Structure

Each integration includes:

1. **Guest Program** (e.g., `/methods/guest/src/main.rs`)  
   This contains the zero-knowledge logic. It:
   - Verifies the TLS Notary proof (core proof).
   - Computes on the authenticated data.
   - Emits public outputs.

   All execution here is proven in zero knowledge using RISC Zero. The compiled ELF binary is stored onchain.

2. **Host Code**  
   This runs on your local machine and:
   - Defines CLI behavior
   - Submits proofs to the prover
   - Interacts with the Cowboy chain

3. **Primitives**  
   Shared types and logic used by both guest and host.

4. **Helpers**  
   Utility functions for interacting with the prover and chain.
