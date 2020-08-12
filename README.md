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

2048-bit RSA is ([recommended by NIST](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-57Pt3r1.pdf)), so it's the only option we support.

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

Note that you should get exactly the same output since dOpenSSL guarantees the result is deterministic.

## References
- [*The Mathematics of the RSA Public-Key Cryptosystem*](http://www.mathaware.org/mam/06/Kaliski.pdf). Burt Kaliski. **RSA Laboratories**.
- [*How does RSA work?*](https://hackernoon.com/how-does-rsa-work-f44918df914b) Jeronimo (Jerry) Garcia. **Hackernoon**.

## Maintainers
 * Website: https://about.me/bernardo_ar
 * Email: bernardoaraujor@gmail.com
