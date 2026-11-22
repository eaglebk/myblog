# Rake of the Day: two CryptoProviders in rustls and a panic out of nowhere

> Why a Rust server can start up fine and then crash on its very first TLS connection, and how one line in main() picks the crypto backend for rustls.

- **URL:** http://localhost:1313/en/rust/rustls-crypto-provider-conflict/
- **Date:** 2026-11-19

- **Tags:** rust, dev, tls, cargo, debugging

---{{< lead >}}
"Rake of the Day" - short breakdowns of non-obvious bugs I stepped on myself, so you don't have to.
{{< /lead >}}

Today's rake is **`rustls`** and its `CryptoProvider`. Neither the compiler, nor `clippy`, nor local tests will catch this one.

---

## What it looked like

I added login-code emails to my Axum backend using the `mail-send` crate (SMTP + STARTTLS + DKIM). Locally `cargo clippy` passed without a single warning, the Docker image built, the deploy went through, and the healthcheck returned `200 OK`.

But the very first request to send a code returned:

```text
HTTP/1.1 502 Bad Gateway
Server: nginx/1.24.0 (Ubuntu)
```

And the server logs had a panic:

```text
thread 'tokio-rt-worker' panicked at rustls-0.23.46/src/crypto/mod.rs:249:14:

Could not automatically determine the process-level CryptoProvider from Rustls crate features.
Call CryptoProvider::install_default() before this point to select a provider manually,
or make sure exactly one of the 'aws-lc-rs' and 'ring' features is enabled.
```

---

## What a CryptoProvider is

`rustls` used to take all of its cryptography (TLS handshake, AES-GCM, ChaCha20, signatures) from the **`ring`** crate.

Starting with **0.23**, the crypto backend lives behind the `CryptoProvider` interface and is selected via features:

- **`ring`** - the classic compact crypto library (Rust + assembly);
- **`aws-lc-rs`** - AWS cryptography based on a BoringSSL fork, with a FIPS mode. This is what `rustls` enables by default today.

{{< alert "comment" >}}
**CryptoProvider** is a struct holding a set of implementations: ciphers, key exchange algorithms, signature verification, a random number generator. `rustls` implements the TLS protocol itself and delegates all the math to the provider.
{{< /alert >}}

If **only one** of these features ends up in the binary, `rustls` picks it as the default provider and the developer never notices.

---

## Where the second provider came from

The root cause is **Cargo feature unification**. When several dependencies pull in the same crate with different features, Cargo builds **one** copy of that crate with **all of the features combined**.

In my project it went like this:

- `aws-sdk-s3` (MinIO / S3 access) enabled the **`aws-lc-rs`** feature in `rustls`;
- the new `mail-send` enabled the **`ring`** feature.

So a single `rustls` ended up with both backends. `rustls` **deliberately refuses** to guess which one you want. Otherwise a service that relies on the FIPS mode of `aws-lc-rs` could silently switch to `ring`. So `rustls` panics and asks you to choose explicitly.

To see who enables which feature:

```bash
cargo tree -e features -i rustls@0.23.46
```

{{< alert "lightbulb" >}}
The `-i` (invert) flag prints the tree bottom-up: from `rustls` to the dependencies that pulled it in. With `-e features` you can see exactly which dependency turned on `ring` and which turned on `aws-lc-rs`.
{{< /alert >}}

---

## Why it didn't crash at startup

The provider is resolved **lazily**, when the first TLS config is created (`ClientConfig::builder()`).

At startup my server connected to Postgres and MinIO inside the Docker network **without TLS**, so `rustls` was never touched. The first TLS usage happened only when `mail-send` started STARTTLS with the recipient's mail server. At that moment the `tokio` worker panicked, the connection dropped, and `nginx` honestly answered `502`.

{{< alert "fire" >}}
A bug like this can sleep in your code for months and wake up on the first HTTPS request to an external API. A healthcheck that only verifies "the process is alive and responding" won't notice it.
{{< /alert >}}

---

## The one-line fix

Install a process-wide provider **before** any TLS connection is created, i.e. as the first line of `main()`:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Pick the rustls crypto backend for the whole process
    let _ = rustls::crypto::ring::default_provider().install_default();

    // ...then logger, DB pool, S3, HTTP server
    Ok(())
}
```

To reach `rustls::crypto::ring`, add `rustls` as a direct dependency:

```toml
[dependencies]
rustls = { version = "0.23", default-features = false, features = ["ring"] }
```

A few details:

1. **Why `let _ =`.** `install_default()` returns `Err` if a provider is already installed (for example, some library got there first). For me that's not an error, so I just ignore the result.
2. **Tests.** `#[tokio::test]` has its own entry point, `main()` is not called there. If your tests use TLS, add the `install_default()` call to a test helper too. Calling it repeatedly is safe, it just returns `Err`:

   ```rust
   fn init_crypto() {
       let _ = rustls::crypto::ring::default_provider().install_default();
   }

   #[tokio::test]
   async fn sends_login_code() {
       init_crypto();
       // ...a test that talks to SMTP or HTTPS
   }
   ```

3. **Alternative.** You can align features so every dependency uses one backend (say, only `aws-lc-rs`). But in a large dependency tree that's fragile: the next `cargo add` can bring the second provider back. An explicit `install_default()` is more reliable.

If you need FIPS or simply want to stay on the `rustls` default backend, the same line works with `aws-lc-rs`: `rustls::crypto::aws_lc_rs::default_provider().install_default()`. The point is to choose explicitly.

---

## Takeaways

1. **Cargo features add up**, they are not chosen: any new dependency can change the behavior of a crate you already use.
2. **`rustls` 0.23+ with two providers panics at runtime**, not at compile time, and only on the first TLS connection.
3. **Pick a `CryptoProvider` explicitly in `main()`** if your project has many networking dependencies (AWS SDK, `reqwest`, `sqlx`, `tonic`, SMTP clients). One line saves you from mysterious `502`s in production.


### 📝 Check yourself: CryptoProvider in rustls


**1. What does Cargo do when two dependencies pull in rustls with different features (ring and aws-lc-rs)?**

- [ ] Builds two separate copies of rustls, each with its own feature
- [ ] Builds one copy of rustls with both features enabled
- [ ] Fails compilation with a feature conflict error
- [ ] Uses the feature of whichever dependency is listed first in Cargo.toml

**2. When does rustls 0.23 panic if both providers are enabled and none is installed explicitly?**

- [ ] At compile time
- [ ] Immediately at program startup
- [ ] When the first TLS config is created, e.g. ClientConfig::builder()
- [ ] Only when the process exits

**3. Why is the result of install_default() ignored with let _ = ?**

- [ ] It returns Err if a provider is already installed, which isn't treated as an error
- [ ] It always returns (), let _ is just cosmetic
- [ ] Otherwise the compiler won't allow calling it in an async context
- [ ] This installs the provider for the current thread only

**4. Why doesn't rustls just pick one of the providers when both are enabled?**

- [ ] Because ring and aws-lc-rs are incompatible and can't live in one binary
- [ ] So that a service relying on, say, the FIPS mode of aws-lc-rs doesn't silently switch to another backend
- [ ] Because it's a bug that will be fixed in the next release
- [ ] Because the provider can only be chosen via an environment variable

**5. Which command shows which dependencies enable features in rustls?**

- [ ] cargo check --features rustls
- [ ] cargo tree -e features -i rustls
- [ ] cargo metadata --rustls
- [ ] cargo clippy -- -W features

**6. Do you need install_default() in #[tokio::test] tests if main() already calls it?**

- [ ] No, main() runs before every test
- [ ] Yes, tests have their own entry point and main() is not called
- [ ] No, rustls always uses ring in tests
- [ ] Yes, but only for non-async tests

🔗 **Docs:** [rustls::crypto::CryptoProvider](https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html)  
📦 **Crates.io:** [crates.io/crates/rustls](https://crates.io/crates/rustls)

