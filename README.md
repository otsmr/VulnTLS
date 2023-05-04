# VulnTLS

VulnTLS is a series of CTF challenges. The series implements several
vulnerabilities. Most of the vulnerabilities were already present in common TLS
implementations. The series provides a basic understanding of TLS and typical
implementation vulnerabilities, with a focus on cryptography.

In this repository there are different setups to exploit the different
vulnerabilities. For this it uses the vulnerable implementation, which has its
own [branch](https://github.com/otsmr/AnotherTLS/tree/vulntls) in the
AnotherTLS repository.


# Vulnerabilities

The vulnerabilities are marked with there difficulty (EASY, HARD, EXTREME). The
evaluation is of course subjective and designed for a bachelor student.


## Psychic signatures (EASY)

Bypass the client certificate authentication!
Run the [challenge](./psychic_signatures/README.md).
```sh
cargo run --bin psychic_signatures
```

## Timing issues (HARD)

Get the private key of the server!
Run the [challenge](./timing_issues/README.md).
```sh
cargo run --bin timing_issues
```

## Dual_EC (HARD)

Decrypt the captured connection!
See [challenge](./dual_ec/README.md) for more.
