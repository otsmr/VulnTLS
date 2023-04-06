# VulnTLS

To better understand attacks and their impact on TLS, it is useful to exploit
them on a real TLS server.

In this repository there are different setups to exploit the different
vulnerabilities. For this it uses the vulnerable implementation, which has its
own [branch](https://github.com/otsmr/AnotherTLS/tree/vulntls) in the
AnotherTLS repository.



# Vulnerabilities

The vulnerabilities are marked with there difficulty (EASY, HARD, EXTREME).

## ECDSA missing signature checks (EASY)

Run the [challenge](./ecdsa_missing_checks/README.md).
```sh
cargo run --bin ecdsa_missing_checks
```
