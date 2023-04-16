# VulnTLS

To better understand attacks and their impact on TLS, it is useful to exploit
them on a real TLS server.

In this repository there are different setups to exploit the different
vulnerabilities. For this it uses the vulnerable implementation, which has its
own [branch](https://github.com/otsmr/AnotherTLS/tree/vulntls) in the
AnotherTLS repository.



# Vulnerabilities

The vulnerabilities are marked with there difficulty (EASY, HARD, EXTREME).

## Psychic signatures (EASY)

Run the [challenge](./psychic_signatures/README.md).
```sh
cargo run --bin psychic_signatures
```
