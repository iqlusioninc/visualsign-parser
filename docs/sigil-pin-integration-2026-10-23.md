# Sigil parser integration

This downstream integration assembles applicable parser fixes, performance improvements, validation and the NEAR feature stack for Sigil. It does not land these PRs upstream or deploy an enclave, HSM, app or TestFlight release. Exact source revisions are recorded in the adjacent JSON manifest.

## Source and scope

- Original Sigil parser pin: `aede2ed2541938f61c0b0ceecd79b50ababd891f`.
- Current upstream main plus the latest reviewed split-Anza Solana and lightweight Sui branches; preserve canonical Ethereum handling, NEAR local wire types and current Fluid behavior.
- Parser fixes and conventions: #475, #589, #391, #558, #606.
- Single decode / IDL performance: #472, #593, #595.
- Independent Bitcoin and Zcash validation: #600, #602 (including the Zcash parser and aggregate 64 shielded-action/output resource cap).
- Shared intents, Solana off-chain envelope rendering, NEP-413 rendering, wrap.near and journey coverage: #489-494. Availability in the parser does not automatically authorize these signing modes in Sigil.
- Token-2022 confidential transfers #405 and DFlow nested layouts #394, ported to current types with compatibility repairs.
- Applicable security lock updates #601, documented canonical Ethereum examples #588, and supporting Rust/iOS/benchmark/fuzz/dependency CI #549/#587/#591/#564/#528.
- Consolidated `iqlusioninc/solana-parser` pin `563f7e03dcf5a34050156360ec6f678b839f7ca9` combines split crates, compact-u16 parsing, IDL precomputation and SizeGuard overflow protection. Its upstream prerequisites are not claimed merged.

## Deliberate exclusions

- #603's SPL program-crate version upgrade is superseded here by split SPL interface crates; do not reintroduce the old program crates.
- #282 is the superseded initial intermediate-output prototype. #405 is ported selectively onto the richer current schema rather than replacing it with #282.
- #99/#119 workshop prototypes, #89 old Jupiter heuristic, #241 hack PoC, #482 instrumentation prototype, #596 reference size PoC, #511 throwaway probe, #559 monetization, #404 production-release plan, #534 enclave-only server work, #385 blog and scanner-only #560/#561 are outside this Sigil library integration.

## Compatibility boundaries and repairs

- Keep split Solana types and a single consolidated parser dependency; independently selecting #595 or #606 pins loses APIs or fixes. Benchmarks and convention tests use current split types and explicit bincode message serialization.
- Intermediate schema is 6: retain the native-decoder capability tag and append typed confidential-transfer information. The optional wallet-supplied amount is explicitly base units and affects display only; changing it does not change policy Borsh bytes. It is not verified mint/proof data. Metadata serialization itself adds the hints map, so independently encoded metadata/payment preimages must update.
- Recognized malformed or unknown intents fail closed. Plain Solana messages keep their intentional rendering. Signed freeform intent and NEP-413 text is escaped reversibly, including literal percent and empty-marker collision coverage. Caller-provided signerAddress must be bound to the selected key by the consuming signer.
- Sigil's existing transaction-only boundary explicitly refuses the new Solana Message wrapper variant; parser availability is not a new signing authorization.
- DFlow retains unresolved ALT placeholders/roles, candidate parameters and Swap2 positive-slippage fee caps. Object key rendering is lexical even with serde_json/preserve_order.
- Preserve current Fluid deployment/extra-funds logic using lightweight types. The independent type oracle covers both current and historical fixture files (14 Fluid cases), rather than dropping historical coverage after the fixture split.
- All VisualSign dependencies in Sigil must be pinned to the same revision. Its dirty checkout and unrelated work are preserved; validation uses an isolated snapshot.

## Verification

All Cargo checks use `RUSTFLAGS='' CARGO_INCREMENTAL=0`; parser checks use pinned Rust 1.94.0. These are local source-tree results, not an upstream merge, deployment or device acceptance claim.

| Gate | Result |
| --- | --- |
| Workspace tests, excluding parser_cli and integration, all targets | 1,852 passed; 7 ignored |
| parser_cli all-target tests | 11 passed |
| Solana no-default library tests | 396 passed |
| Core diagnostics library tests | 113 passed |
| CLI-core diagnostics library tests | 47 passed |
| NEAR diagnostics library plus independent onchain snapshot | 132 passed |
| Solana diagnostics library tests | 446 passed |
| Strict workspace/excluding CLI and separate CLI all-target clippy | Passed |
| Codegen with protoc 21.4, repeated output hashes | Identical |
| Formatting/diff checks | Passed |
| Standalone Solana fuzz binaries, locked Rust 1.94 check | Passed |
| Six embedded-library iOS builds with CC/CXX=/usr/bin/false, no defaults | Passed |
| Consolidated solana-parser dependency | 59 unit +21 integration tests, strict clippy, formatting, no-C iOS passed |

The complete parser build and separate integration-test compilation pass. All 22 gRPC integration tests pass. The HTTP transport lane started its server but exceeded the bounded 60-second runtime and was terminated, reproducing the previously documented local hang. No unfiltered `make test` claim is made here. The node-backed Bitcoin/Zcash regtests are intentionally ignored and were not executed.

The isolated actual-dirty Sigil snapshot passes 899 workspace tests with one named pre-existing Apple Vision runtime failure filtered, plus final affected decode/FFI (333) and Solana/NEAR (32) checks. Strict workspace clippy and formatting pass. Final iOS device and simulator release staticlibs build and exclude test-support symbols, with a WireSession symbol positive control. These checks do not establish physical Secure Enclave, biometric, camera or radio acceptance.

Sigil's existing consumer lock already has security-fixed rand 0.8.8/0.9.5 and keccak 0.1.6; TLS crates are absent from its selected library graph. Updating the parser's lock alone would not propagate security versions to a consuming Git dependency.

Independent review verified repairs for recognized-intent text collisions and the omitted Swap2 fee cap. Compatibility findings were posted on #394/#405/#472/#489/#490/#491/#492/#576/#593/#595/#606.

Publication and exact Git-pin validation are separate final steps. Do not describe the local-path consumer checks as verification of a published Git revision.
