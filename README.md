

`markdown
<p align="center">
  <img src="https://raw.githubusercontent.com/high-cde/high-bridgechain/main/docs/logo.png" width="180" />
</p>

<h1 align="center">🌉 HighBridgeChain</h1>
<h3 align="center">A ZLang‑Native Blockchain for Deterministic, Persistent, Hardware‑Bridging Systems</h3>

<p align="center">
  <a href="https://github.com/high-cde/high-bridgechain/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/high-cde/high-bridgechain/build.yml?label=Build&style=for-the-badge">
  </a>
  <a href="https://github.com/high-cde/high-bridgechain">
    <img src="https://img.shields.io/github/repo-size/high-cde/high-bridgechain?style=for-the-badge">
  </a>
  <a href="https://github.com/high-cde/high-bridgechain">
    <img src="https://img.shields.io/github/license/high-cde/high-bridgechain?style=for-the-badge">
  </a>
  <a href="https://github.com/high-cde/high-bridgechain">
    <img src="https://img.shields.io/github/languages/top/high-cde/high-bridgechain?style=for-the-badge">
  </a>
</p>

---

🇮🇹 Descrizione (Italiano)

HighBridgeChain è una blockchain linguaggio‑centrica, costruita attorno a:

- ZLang, linguaggio deterministico nativo  
- BridgeZ, la Z‑Unit interna per fee, staking e reward  
- un layer di interoperabilità hardware, per sistemi moderni e legacy  

È progettata per essere:

- leggera (compatibile con hardware datato)  
- deterministica (grazie alla VM ZLang)  
- persistente (storia verificabile a lungo termine)  
- modulare (nodi gateway, nodi legacy, nodi full)  

---

🇬🇧 Description (English)

HighBridgeChain is a language‑centric blockchain, built around:

- ZLang, a deterministic native execution language  
- BridgeZ, the internal Z‑Unit for fees, staking and rewards  
- a hardware‑bridging layer, enabling modern ↔ legacy interoperability  

It is designed to be:

- lightweight (legacy‑friendly)  
- deterministic (ZLang VM)  
- persistent (long‑term verifiable history)  
- modular (gateway nodes, legacy nodes, full nodes)  

---

🧠 Architecture Overview

`
┌──────────────────────────────────────────────────────────────┐
│                        HighBridgeChain                        │
├──────────────────────────────────────────────────────────────┤
│                        ZLang VM Layer                         │
│  - Deterministic execution engine                             │
│  - System scripts (validator, apply, reward)                  │
├──────────────────────────────────────────────────────────────┤
│                        Ledger Layer                           │
│  - BridgeZ balances                                           │
│  - Accounts, nonces, staking                                  │
│  - Hardware mappings                                          │
├──────────────────────────────────────────────────────────────┤
│                        Consensus Layer                        │
│  - Hybrid PoA/PoS                                             │
│  - Round‑robin validator rotation                             │
│  - Block reward + fee distribution                            │
├──────────────────────────────────────────────────────────────┤
│                        Networking Layer                       │
│  - Lightweight P2P                                            │
│  - Block/Tx propagation                                       │
│  - Legacy‑friendly gateways                                   │
└──────────────────────────────────────────────────────────────┘
`

---

🔧 Build & Run

Build

`bash
cargo build --release
`

Run node

`bash
cargo run -p node
`

RPC

The node exposes a minimal RPC server at:

`
127.0.0.1:8545
`

---

🧩 Repository Structure

`
high-bridgechain/
├─ node/                 → Full node (consensus, networking, VM)
├─ protocol/             → Block, transaction, encoding
├─ runtime/              → ZLang system scripts
├─ genesis/              → Genesis configuration
├─ rpc/                  → RPC server
├─ tools/                → CLI + explorer
└─ docs/                 → Technical documentation
`

---

🪙 BridgeZ — Native Z‑Unit

BridgeZ is the native unit of value inside HighBridgeChain.

Used for:

- transaction fees  
- validator staking  
- block rewards  
- execution priority  
- hardware/config anchoring  

BridgeZ is not a smart contract token:  
è parte del runtime, come il gas di una VM nativa.

---

🧬 ZLang Runtime Scripts

Located in:

`
runtime/system.tx.validator.zlang
runtime/system.tx.apply.zlang
runtime/system.reward.zlang
`

They define:

- transaction validation  
- state application  
- block reward logic  

---

🛠️ Tools

BridgeZ CLI

`bash
bridgez-cli balance acc:high
bridgez-cli transfer acc:high acc:node1 1000
`

Block Explorer Lite

`bash
block-explorer-lite
`

---

🗺️ Roadmap

✔ Phase 1 — Core (Completed)
- Node base  
- PoA/PoS consensus  
- BridgeZ ledger  
- ZLang runtime  
- Minimal RPC  

🔄 Phase 2 — In Progress
- Real P2P networking  
- JSON‑RPC (Ethereum‑style)  
- Web explorer  
- Optimized ZLang VM  

🔮 Phase 3 — Future
- ZLang sidechains  
- ZDOS integration layer  
- Hardware attestation  
- ZLang package registry  

---

🤝 Contributing

Pull requests and proposals are welcome.  
HighBridgeChain is designed to grow as a modular, open ecosystem.

---

📄 License

MIT License (or chosen by the maintainer).

