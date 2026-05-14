# Secure Online Voting Roadmap

This roadmap tracks the first implementation path for `arm_vote`: move from the
current math work toward a verifiable online voting prototype built on clear
cryptographic boundaries.

## 1. Stabilize `crypto_core`

- Fix current compile errors.
- Implement `ConstModInt::inverse()`.
- Decide whether `ConstModInt` should be `Copy`.
- Add arithmetic trait implementations where useful.
- Add focused unit and property-style tests for modular arithmetic, vectors,
  matrices, and inverse behavior.

## 2. Choose and Document the Voting Protocol

- Start with a homomorphic tallying protocol for a simple single-choice contest.
- Document the protocol shape before building the application layer.
- Identify which security properties are in scope for the prototype.

## 3. Add Field, Group, and ElGamal Abstractions

- Introduce explicit field/group modules above the raw modular arithmetic layer.
- Add ElGamal key generation, encryption, homomorphic combination, and
  decryption.
- Keep protocol types separate from low-level arithmetic types.

## 4. Build Ballot Encryption

- Model election configuration, candidates, plaintext ballots, encrypted
  ballots, and tallies.
- Start with one contest where voters choose exactly one candidate.
- Represent votes as one-hot vectors and encrypt each component.

## 5. Add Ballot Validity Proofs

- Prove encrypted choices are well-formed without revealing the vote.
- First target: each encrypted value is 0 or 1, and the encrypted row sums to 1.
- Use transcript hashing and canonical encodings once the proof format settles.

## 6. Add a Public Bulletin Board

- Store election setup, public keys, encrypted ballots, proofs, tally output, and
  decryption proofs in an append-only public record.
- Make the record replayable by independent verifiers.

## 7. Create a Demo CLI

- Replace the placeholder demo with commands for setup, casting, tallying, and
  verification.
- Keep the first end-to-end flow local before adding web/API complexity.

## 8. Write a Threat Model

- Document trust assumptions, attacker capabilities, and explicit non-goals.
- Cover authentication, malicious servers, compromised voter devices, coercion,
  administrator trust, and auditability.

## 9. Add Web/API Layer

- Add endpoints for election setup, public-key discovery, encrypted ballot
  submission, bulletin-board reads, tally publication, and verification.
- Ensure the server never needs plaintext votes.

## 10. Security Hardening

- Replace toy randomness with production-grade randomness.
- Add canonical serialization, fuzzing, transcript tests, and broader
  verification tests.
- Prepare the protocol and implementation for external review.
