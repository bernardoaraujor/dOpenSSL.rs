# dOpenSSL.rs

## Intro

Rust Crate to generate determinstic RSA keypairs and Pseudo-Random Numbers from the same seed. Based on FFI bindings from https://github.com./bernardoaraujor/dopenssl, which is the continuation of the [original implementation](https://github.com/infinit/dopenssl) by [Infinit](https://infinit.sh/).

The dOpenSSL library extends the OpenSSL Project cryptographic library so as to provide deterministic random generation functionalities. Basically, dOpenSSL guarantees that should a big number or a cryptographic key be generated, given a PRNG's state, the result would be always the same.

The OpenSSL random generator introduces entropy in many places, making it unusable in a deterministic way. Thus, some functions have been cloned and adapted in order to guarantee determinism.

The `libdopenssl` API allows deterministic generation of:
- [Big Number](https://www.openssl.org/docs/man1.0.2/man3/bn.html) (**BN**)
- [Pseudo-Random Number](https://www.openssl.org/docs/man1.0.2/man3/rand.html) (**PRNG**)
- [Rivest–Shamir–Adleman](https://www.openssl.org/docs/man1.0.2/man1/openssl-rsa.html) (**RSA**) keypairs

# dOpenSSL for IOTA
Due to historical reasons, the [IOTA protocol](https://iota.org) relies on seeds to generate signing keys in a deterministic way.

**IOTA 1.0** only supports the [Winternitz One-Time Signature](https://eprint.iacr.org/2011/191.pdf) (**W-OTS**) Scheme. This choice introduced the restriction of non-reusable addresses.

**IOTA 1.5** supports the legacy W-OTS, plus the reusable [ed25519](https://eprint.iacr.org/2020/823.pdf), as proposed in this [RFC](https://github.com/Wollac/protocol-rfcs/blob/ed25519/text/0009-ed25519-signature-scheme/0009-ed25519-signature-scheme.md).

**IOTA 2.0** introduces a layered message approach where **IOTA Access** establishes RSA-based keypairs.

## IOTA2RSA

The `libiota2rsa` API allows to create a RSA keypair starting from an IOTA seed.

### IOTA seed
The IOTA seed is composed of 81 trytes. Trytes can be mapped into an ASCII character in the [Tryte Encoding Scheme](https://docs.iota.org/docs/getting-started/0.1/introduction/ternary), which means we can think of an IOTA seed as something like this string:

```
TTKSPEVMHM9HWVAXZMJO9HNPUPPUDKVHQNLUUMROTEAK9QTW9SHWNNJMXNCEILESRKCJHKQQHJBRJHWFN
```

IOTA seeds are kept safe via [Hardware Security Modules (HSM)](https://en.wikipedia.org/wiki/Hardware_security_module) or [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs.git).

### RSA seed

The RSA seed is used to create the RSA private key, from which the public key is derived.

The bytesize of `rsa_seed` is given by the formula: `rsa_seed_len = rsa_bits / 8`. That means that `rsa_seed` is directly proportional to how many bits we choose for the RSA keypair.

2048-bit RSA is ([recommended by NIST](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-57Pt3r1.pdf)), so `rsa_bits = 2048` is the only value IOTA2RSA supports.

### IOTA to RSA seed derivation

The lengths of `iota_seed` and `rsa_seed` are always different.
So the seed derivation algorithm consists of:
```
for (i = 0; i < rsa_seed_len; i++) {
  copy iota_seed[i % iota_seed_len] into rsa_seed[i]
}
```

Assuming `rsa_bits = 2048`:

- `iota_seed = TTKSPEVMHM9HWVAXZMJO9HNPUPPUDKVHQNLUUMROTEAK9QTW9SHWNNJMXNCEILESRKCJHKQQHJBRJHWFN`
- `iota_seed_len = 81`

- `rsa_seed = TTKSPEVMHM9HWVAXZMJO9HNPUPPUDKVHQNLUUMROTEAK9QTW9SHWNNJMXNCEILESRKCJHKQQHJBRJHWFNTTKSPEVMHM9HWVAXZMJO9HNPUPPUDKVHQNLUUMROTEAK9QTW9SHWNNJMXNCEILESRKCJHKQQHJBRJHWFNTTKSPEVMHM9HWVAXZMJO9HNPUPPUDKVHQNLUUMROTEAK9QTW9SHWNNJMXNCEILESRKCJHKQQHJBRJHWFNTTKSPEVMHM9HW`
- `rsa_seed_len = 256`

## Dependencies
The library relies upon the following libraries:

 * [**OpenSSL**](http://www.openssl.org) which provides the fundamental cryptographic operations. Install v1.0.2 into Ubuntu 18.04 with:

```
$ sudo apt-get install libssl1.0-dev -y
```

## Clone, Build
Assuming you're on Ubuntu 18.04:

```
$ git clone http://github.com/bernardoaraujor/dopenssl.rs
$ cd dopenssl.rs
$ cargo build
$ cargo install
```

## Example
The following source code can be found in `src/sample.c`.

It shows how dOpenSSL can be used to generate cryptographic keys (in this case 2048-bit RSA keys) in a deterministic way from an IOTA Seed.

The program therefore assures that, given the same IOTA Seed, the exact same RSA key will be generated.

```C
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <dopenssl/rsa.h>
#include <dopenssl/rand.h>

#include "iota2rsa/iota2rsa.h"

int main(int argc, char **argv)
{
  if (argc != 2)
  {
    printf("usage: %s [iota_seed]\n", argv[0]);
    return(1);
  }

  unsigned char* iota_seed = argv[1];
  unsigned int const length = strlen(iota_seed);
  if(length != 81){
    printf("[error] IOTA seeds are 81 chars long.\n");
    return (1);
  }

  unsigned int const rsa_bits = 2048;
  unsigned char* rsa_seed;

  if (!seed_iota2rsa(iota_seed, rsa_seed, rsa_bits))
      return 0;

  /* Initialize the deterministic random generator */
  if (dRAND_init() != 1)
  {
    printf("[error] unable to initialize the random generator\n");
    return (1);
  }

  /* Generate the RSA key */
  RSA* rsa = NULL;

  if ((rsa = dRSA_deduce_privatekey(rsa_bits, rsa_seed)) == NULL)
  {
    printf("[error] unable to generate the RSA key pair\n");
    return (1);
  }

  /* Display the RSA components */
  RSA_print_fp(stdout, rsa, 0);

  /* Clean the resources */
  if (dRAND_clean() != 1)
  {
    printf("[error] unable to clean the random generator\n");
    return (1);
  }

  return (0);
}
```

Executing the binary:
```
$ ./iota2rsa_sample TTKSPEVMHM9HWVAXZMJO9HNPUPPUDKVHQNLUUMROTEAK9QTW9SHWNNJMXNCEILESRKCJHKQQHJBRJHWFN
Private-Key: (2048 bit)
modulus:
    00:e9:2f:bf:10:b2:16:5e:8a:69:f9:51:d5:f6:59:
    a8:57:b4:b3:33:84:f7:2c:eb:8b:aa:9a:2b:82:b7:
    3e:d4:7a:0e:94:58:9a:e0:b4:ff:d3:cf:fe:88:32:
    6a:bf:eb:9f:13:b8:ea:79:35:be:a5:cd:6a:8d:ae:
    24:74:5e:57:46:70:2d:0e:35:c0:fc:36:7e:97:6c:
    e8:b7:0f:eb:cb:de:2e:60:12:6d:66:5d:ed:e3:9d:
    44:ff:8c:df:f3:ab:7e:8c:e8:de:92:2c:cf:c6:92:
    d1:18:08:e1:f0:ef:a0:b9:13:23:81:48:c5:95:46:
    e1:f1:f4:6d:1f:b2:56:10:25:d7:e3:a4:f9:93:b5:
    74:5e:89:94:ba:03:df:00:9f:1f:e5:99:75:c9:ad:
    99:22:c4:b5:c1:83:dc:f8:43:bb:df:9e:98:ff:78:
    f5:6e:71:89:43:9f:98:19:6e:be:a6:80:6f:61:e6:
    2c:b9:db:ab:13:26:f9:7d:ee:a8:40:9f:97:2e:66:
    0d:2e:b2:86:eb:40:1b:a5:77:18:20:68:30:24:91:
    6b:55:8d:55:34:66:97:2b:24:dc:b9:04:7f:d4:f2:
    01:a2:11:28:8b:75:aa:23:56:d3:29:a5:6f:b6:99:
    b0:b1:a1:3a:e2:b0:27:9f:d7:95:d4:42:7f:1a:19:
    22:1b
publicExponent:
    01:e3:70:6a:a4:2c:88:d0:b0:e2:c4:8c:11:c2:7d:
    17:63:9f:13:14:cd:f3:92:d3:28:43:c5:64:86:1d:
    1e:1d:15:2c:16:d0:53:30:bc:ab:30:a0:6f:93:7d:
    f4:94:de:fd:df:d3:a4:4e:d8:17:85:ae:65:95:7e:
    8f:70:d6:7f:35:d6:d0:08:52:b1:1d:c4:b0:ec:35:
    f5:8d:e3:b5:30:30:1c:bc:76:08:95:2d:14:87:56:
    13:c8:06:7d:f6:d6:3d:63:d9:6b:74:12:13:c2:9c:
    00:7e:06:f6:0c:98:c0:f1:77:3a:23:af:1c:fe:d2:
    7b:24:7a:4e:3b:65:7b:0b:f5
privateExponent:
    4f:fc:18:54:e8:fe:66:6c:59:e6:0c:65:34:a0:94:
    2e:a1:04:ec:a0:fd:0c:1b:44:34:ce:62:ea:31:7a:
    1b:ea:8e:2e:c7:46:80:d2:6c:15:28:39:a2:7e:39:
    81:fa:bc:30:08:89:b4:80:82:40:d5:43:15:75:19:
    16:ee:12:5f:6b:65:5f:56:19:31:fe:5b:4d:d3:7d:
    c0:c8:04:0f:4f:ce:3b:e1:18:3d:34:aa:03:7b:30:
    3f:5a:84:14:09:64:37:19:1f:89:89:4e:de:0f:a9:
    45:db:ed:de:2e:c0:9b:e5:0e:e3:40:dc:4b:4a:22:
    08:77:e2:7f:4b:b7:7e:74:a1:ff:85:8a:59:a6:d0:
    75:52:76:44:a2:c9:f4:06:be:77:cf:94:40:a8:a8:
    d8:17:96:41:24:07:e5:b6:d4:b5:d2:a1:7c:76:98:
    6c:14:b3:f3:fd:24:cf:75:56:4f:a1:21:3b:62:b0:
    0e:b0:b0:64:8b:c4:df:49:77:bc:5a:52:17:e1:d1:
    b2:5a:99:98:4b:e2:ba:69:99:a7:57:77:5d:23:6f:
    5e:b4:8e:ed:48:a8:4c:e2:96:84:c8:99:ce:cb:52:
    86:36:24:20:1f:db:95:96:9f:fb:51:cd:44:82:27:
    b0:99:ea:29:4d:d9:a7:19:22:a6:ec:7b:37:b3:41:
    e5
prime1:
    00:f7:cd:ac:85:2b:bf:bc:a6:99:ff:ea:93:1a:6a:
    f8:00:17:d6:36:66:f0:00:a2:d9:e0:46:33:d9:25:
    73:68:43:c7:aa:ab:9c:77:6e:19:60:ed:26:f5:f6:
    2a:de:0b:26:63:b5:cc:05:68:3c:0d:7b:e4:44:0e:
    d8:20:ba:84:26:2c:24:1f:ec:db:46:c4:6e:f2:3e:
    05:6a:2e:b8:57:44:52:41:bf:b4:30:c7:65:c9:e9:
    14:1c:4f:2f:94:a0:a1:85:71:cc:e1:29:c5:ad:bc:
    73:dd:fb:e9:41:75:1d:92:a6:9b:a9:66:69:38:0b:
    de:df:cc:0e:73:ee:33:31:df
prime2:
    00:f0:e6:4d:02:0d:5b:63:88:76:f1:f5:b2:4a:c0:
    ee:47:ae:7b:61:bf:88:5f:b6:00:7a:88:82:fa:01:
    ce:5b:72:71:7a:f3:28:80:f6:82:a0:83:a6:63:19:
    a3:b1:9e:02:ad:08:65:1d:6e:41:d7:aa:43:c4:ab:
    67:af:a4:3c:b4:b5:2b:1a:b5:5f:08:de:11:18:1c:
    50:92:ff:ab:21:7e:f7:c7:72:b6:18:dc:8b:6e:f7:
    9b:0f:17:fd:6f:47:56:6a:91:65:24:ff:0a:ad:e9:
    bd:f6:bb:b0:f7:e3:19:20:8a:98:be:30:ee:c3:c1:
    7a:eb:a0:25:ee:43:f3:6f:45
exponent1:
    00:f6:0b:93:94:1d:0a:44:fb:44:ec:5a:d5:7e:ab:
    32:dd:51:1d:24:ce:b1:90:4e:67:09:49:51:47:ef:
    c6:04:8f:5e:f8:15:a0:8b:0c:5a:25:ff:e6:57:59:
    8b:ce:b2:1d:86:0f:36:4b:c3:25:14:de:c3:99:ab:
    3a:85:58:59:20:e9:49:cf:54:27:27:1e:eb:82:74:
    16:5e:6f:e4:06:15:15:f2:00:17:50:ec:84:f6:60:
    94:20:09:13:1d:83:82:16:a0:ff:74:84:cb:71:32:
    76:2f:d5:11:fa:66:b2:f8:b7:8d:74:f8:bc:c7:17:
    9f:81:55:4f:9d:34:42:38:11
exponent2:
    00:be:25:84:b8:93:f1:9c:fe:27:54:61:70:59:33:
    3f:8e:b6:b5:fc:b7:92:ee:80:cd:08:73:88:31:97:
    b9:02:58:99:34:75:10:39:39:f0:1c:fa:36:13:d8:
    d3:a1:15:0d:8a:b1:47:2b:9c:d2:c3:8e:87:84:e9:
    72:3a:2e:d5:33:64:ae:70:cc:ac:02:2e:f3:96:c9:
    04:4c:54:98:91:49:6b:6e:47:ff:7e:a3:de:12:49:
    fb:6b:1a:72:1c:1b:ca:2d:e1:41:64:b5:65:18:2b:
    5c:b2:57:bb:d7:32:74:2c:bf:d2:7c:2f:af:2c:5a:
    5b:10:b1:e2:b9:75:6c:3c:95
coefficient:
    35:b6:cf:cc:ed:44:4d:52:4c:06:f0:1f:3e:5f:b5:
    d0:67:6e:c1:eb:8f:a5:e7:15:f6:5d:50:a4:37:36:
    1f:4c:41:e2:a1:0a:e8:7c:1b:57:41:3b:f6:d0:98:
    83:7e:63:b1:a7:fa:9b:fa:0f:ef:71:35:4b:b2:be:
    08:ba:4a:1b:06:a8:ed:8b:6f:59:ce:2a:c9:76:dd:
    73:6a:8d:15:4c:c9:8f:c7:07:ca:2e:95:49:ec:1c:
    d3:4e:65:5f:1c:02:4d:89:e8:d5:3d:56:68:5a:4a:
    93:46:be:97:5c:8d:71:a9:51:2e:e6:b3:6d:b5:26:
    43:5c:b8:64:57:60:24:3c
```

## References
- [*The Mathematics of the RSA Public-Key Cryptosystem*](http://www.mathaware.org/mam/06/Kaliski.pdf). Burt Kaliski. **RSA Laboratories**.
- [*How does RSA work?*](https://hackernoon.com/how-does-rsa-work-f44918df914b) Jeronimo (Jerry) Garcia. **Hackernoon**.

## Maintainers
 * Website: https://about.me/bernardo_ar
 * Email: bernardoaraujor@gmail.com
