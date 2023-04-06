# ECDSA missing signature checks

Using the signed client certificate.

```sh
curl --cert client.signed.cert --key client.key -iv --insecure https://localhost:4000
```


## Resources

- [CVE-2022-21449: Psychic Signatures in Java](https://neilmadden.blog/2022/04/19/psychic-signatures-in-java/)
