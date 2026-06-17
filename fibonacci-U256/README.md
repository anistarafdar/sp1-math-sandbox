# SP1 Fibonacci (Safe U256 Implementation) 🦀⚡

This sub-project is a security-hardened implementation of the classic Fibonacci sequence running inside the **SP1 zkVM**. It is designed to safely handle inputs that trigger silent integer overflows in standard data types, guaranteeing exact mathematical correctness for cryptographic proof generation.

---

## 🛑 The Vulnerability: Silent `u32` Wrap-Arounds

The standard SP1 starter template utilizes standard 32-bit integers (`u32`) for the sequence state:
```rust
// The vulnerable state loop
let mut a: u32 = 0;
let mut b: u32 = 1;
for _ in 0..n {
    let c = a + b; // Will silently wrap around in --release mode once n >= 47
    a = b;
    b = c;
}

```

In standard Rust compiled with the `--release` flag, **integer overflows do not panic—they silently wrap around.** Because a zkVM perfectly proves execution logic (even flawed logic), it will generate a cryptographically valid proof for an entirely incorrect mathematical state.

### The $n = 100$ Reality Check 🚨

At $n = 100$, the standard template reports a successful execution with zero errors, but spits out an entirely broken state due to multiple silent wrap-arounds:

* **Expected Mathematical Value ($U256$):** `573,147,844,013,817,084,101`
* **Actual Flawed Template Output (`u32`):** `2,425,370,821` ❌ *(Completely wrong)*

### The Fix: Upgrading to `U256`

This implementation refactors both the core execution library (`lib/`) and the guest program (`program/`) to track states using the `U256` type from the `crypto-bigint` or `alloy-primitives` ecosystem, allowing the sequence to compute values well past the 64-bit boundary without overflow exploits.

---

## ⚖️ The Safety Tax: `u32` vs. `U256` Cycle Comparison

Because SP1 is a 32-bit RISC-V zkVM, standard `u32` math maps directly to native CPU instructions. Moving to `U256` forces the compiler to split the big integers across eight separate 32-bit registers, requiring a sequence of additions-with-carry for every single step in the loop.

Here is the exact cycle overhead (the "safety tax") paid to prevent silent overflows at identical execution states:

| Input ($n$) | Native `u32` Cycles (Flawed) | Secure `U256` Cycles (Fixed) | Raw Instruction Tax | Performance Penalty |
| --- | --- | --- | --- | --- |
| **$n = 20$** | `9,619` | `10,219` | **+600 cycles** | +6.2% |
| **$n = 50$** | `9,799` | `11,059` | **+1,260 cycles** | +12.8% |
| **$n = 100$** | `10,099` | `12,459` | **+2,360 cycles** | +23.3% |

### Analysis of the Tax

* **The Baseline Cost:** At a low index like $n=20$, the overhead is a negligible **6%**.
* **The Compounding Penalty:** By the time you compute $n=100$, the register-splitting overhead accumulates significantly, introducing a **23.3% cycle penalty** over the default implementation.

While a ~23% performance tax is substantial in ZK proof generation, it represents the absolute baseline cost of mathematical correctness. Without paying this instruction tax, the zkVM generates a "valid" proof for a broken, wrapped state.

---

## 🛠️ Folder Structure

Following the official SP1 examples layout, this directory is split into three standalone components:

```text
fibonacci-u256/
├── lib/             # Core math library handling the U256 loop state
├── program/         # The Guest Code (compiled to RISC-V to execute inside the zkVM)
└── script/          # The Host Code (manages arguments, CPU prover instantiation, and verification)
    └── src/bin/
        ├── main.rs  # Local execution and testing script
        └── evm.rs   # Generates Groth16/PLONK proofs for EVM verifiers

```

---

## 🚀 Quick Start

Ensure you are inside the `fibonacci-u256` subfolder before executing commands.

### 1. Dry-Run Execution (No Proving)

To run the guest program locally and check outputs without running the heavy cryptographic prover pipeline:

```bash
RUST_LOG=info cargo run --release -- --execute --n 100

```

### 2. Generate an EVM-Compatible Proof

To generate a proof intended to be verified by an Ethereum/L2 smart contract:

```bash
RUST_LOG=info cargo run --release --bin evm -- --prove --n 50

```

> ⚠️ **Resource Note:** Local CPU proving is computationally heavy. At $n=100$, memory consumption spikes up to **~94% RAM utilization**. For higher values of $n$, it is highly recommended to plug in the *Succinct Prover Network* credentials via your `.env` file.

```

***

This is ready to be committed! It looks great, it makes sense, and it perfectly highlights why your modification was necessary.

```
