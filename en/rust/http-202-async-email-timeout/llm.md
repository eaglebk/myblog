# Rake of the Day: the server returned 202 Accepted... after 9 seconds, and the client timed out

> Why an async email endpoint makes a mobile app crash with TimeoutException, and how to offload SMTP properly with tokio::spawn.

- **URL:** https://eagle2000.space/en/rust/http-202-async-email-timeout/
- **Date:** 2026-11-25

- **Tags:** rust, tokio, axum, flutter, debugging

---{{< lead >}}
"Rake of the Day" - short breakdowns of non-obvious bugs I stepped on myself, so you don't have to.
{{< /lead >}}

Today's topic is the deceptive `202 Accepted` status code and why sending an email over SMTP must never block an HTTP response.

---

## What it looked like

A user in the Flutter app enters their email and taps "Get code". Eight seconds later the screen shows an error:

```text
TimeoutException after 0:00:08: Future not completed
```

A couple of seconds later, the email with the code **arrives in their inbox**. So the server did everything right, yet the client assumed the server was dead and never navigated to the code-entry screen.

---

## What was happening on the server

My `POST /v1/members/magic` endpoint honestly returned `202 Accepted`, but inside the handler there was a plain `await`:

```rust
repo::insert_code(&st.db, Uuid::new_v4(), member.id, &hash(&code), expires).await?;
mailer.send_code(&email, &code).await?; // <-- Waiting for the full SMTP send right here!
Ok(())
```

While I tested locally with a console mailer (which just printed the code to stdout), it took `0.1 ms`.

Once a real direct MX sender in Rust (`mail-send`) came into play, a single `send_code()` call turned into a chain of network round-trips: an MX DNS lookup, TCP connection on port 25, TLS handshake, and message transfer until the remote mail server replies.

All of that took **3 to 10 seconds**, and with recent ISP throttling and network slowdowns, external TCP/TLS handshakes to mail servers stall even longer. Meanwhile, the mobile client had a hard **8-second** timeout.

{{< alert "comment" >}}
HTTP **202 Accepted** means the server has accepted the request for processing, not that the work is already finished when the response is sent. Waiting for an external SMTP server before replying defeats the purpose of `202`.
{{< /alert >}}

---

## The fix: an honest 202 Accepted via tokio::spawn

The code is already persisted in the database, so the client doesn't need to wait until a mail server on the other side of the internet replies `250 OK`. Offloading the send to the background is enough:

```rust
repo::insert_code(&st.db, Uuid::new_v4(), member.id, &hash(&code), expires).await?;

if matches!(mailer, Mailer::Console | Mailer::Capture(_)) {
    // In dev and unit tests, await directly here without tokio::spawn
    mailer.send_code(&email, &code).await?;
} else {
    let mailer = mailer.clone();
    let email_bg = email.clone();
    tokio::spawn(async move {
        if let Err(e) = mailer.send_code(&email_bg, &code).await {
            tracing::error!(error = %e, to = %email_bg, "failed to send magic code");
        }
    });
}
```

Now the response goes back to the client in **~15 ms** instead of 8+ seconds.

{{< alert "lightbulb" >}}
Errors from a spawned `tokio::spawn` task never reach the HTTP handler, so log them explicitly inside the `async move` block (for example with `tracing::error!`). Otherwise SMTP failures will happen silently.
{{< /alert >}}

---

## Three important details to avoid stepping on a new rake

1. **Don't break unit tests.** When tests use a mock mailer (`Mailer::Capture`), the test immediately checks that the email was saved in the mock's in-memory buffer (e.g., an internal `Vec`) right after calling the handler. Moving the mock into `tokio::spawn` as well would make tests flaky due to a race condition, so keep a direct `.await` for mock and console mailers.
2. **Set timeouts inside the background task.** If the recipient's primary MX server doesn't respond, an untimed `connect()` can hang for a minute on every attempt. I wrap `connect` (10s) and `send` (15s) in `tokio::time::timeout`.
3. **Spam protection is already in place.** Saving the code in the DB (and checking the 60-second cooldown) happens **before** `tokio::spawn`, so a repeated tap from the user won't trigger a second email.

{{< alert "fire" >}}
There is one trade-off with a bare `tokio::spawn`: if the server restarts abruptly or runs out of memory, the in-flight background task may not finish even though the client already got `202`. If guaranteed delivery across restarts matters, store outgoing emails in a database outbox table instead.
{{< /alert >}}

🔗 **Docs:** [tokio::spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)

