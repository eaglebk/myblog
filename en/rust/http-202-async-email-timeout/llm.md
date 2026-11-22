# Rake of the Day: the server returned 202 Accepted... after 9 seconds, and the client timed out

> Why an async email endpoint makes a mobile app crash with TimeoutException, and how to offload SMTP properly with tokio::spawn.

- **URL:** http://localhost:1313/en/rust/http-202-async-email-timeout/
- **Date:** 2026-11-19

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
mailer.send_code(&email, &code).await?; // <-- Waiting for the full SMTP dialog right here!
Ok(())
```

While I tested locally with a console mailer (which just printed the code to stdout), it took `0.1 ms`.

Once a real direct MX sender in Rust (`mail-send`) came into play, a single `send_code()` call turned into a chain of network round-trips:

1. DNS lookup for the recipient domain's MX records.
2. TCP connection to port 25 of the external mail server.
3. `EHLO` -> `STARTTLS` -> TLS handshake.
4. Message transfer, DKIM signing, and waiting for `250 OK`.

All of that took **3 to 10 seconds** - while the mobile client had a hard **8-second** timeout.

{{< alert "comment" >}}
HTTP **202 Accepted** means the server has accepted the request for processing, not that the work is already finished when the response is sent. Waiting for an external SMTP server before replying defeats the purpose of `202`.
{{< /alert >}}

---

## The fix: an honest 202 Accepted via tokio::spawn

The code is already persisted in the database, so the client doesn't need to wait until a mail server on the other side of the internet replies `250 OK`. Offloading the send to the background is enough:

```rust
repo::insert_code(&st.db, Uuid::new_v4(), member.id, &hash(&code), expires).await?;

if matches!(mailer, Mailer::Console | Mailer::Capture(_)) {
    // Keep it synchronous in dev and unit tests
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

1. **Don't break unit tests.** When tests use a mock mailer (`Mailer::Capture`), the test immediately checks that the email landed in a vector right after calling the handler. Moving the mock into `tokio::spawn` as well would make tests flaky due to a race condition. Keep mock and console mailers synchronous.
2. **Set timeouts inside the background task.** If the recipient's primary MX server doesn't respond, an untimed `connect()` can hang for a minute on every attempt. I wrap `connect` (10s) and `send` (15s) in `tokio::time::timeout`.
3. **Spam protection is already in place.** Saving the code in the DB (and checking the 60-second cooldown) happens **before** `tokio::spawn`, so a repeated tap from the user won't trigger a second email.

{{< alert "fire" >}}
There is one trade-off with a bare `tokio::spawn`: if the server restarts abruptly or runs out of memory, the in-flight background task may not finish even though the client already got `202`. If guaranteed delivery across restarts matters, store outgoing emails in a database outbox table instead.
{{< /alert >}}


### 📝 Check yourself: HTTP 202 and tokio::spawn


**1. Why did the mobile app fail with TimeoutException even though the server returned 202 Accepted?**

- [ ] Because Flutter does not support status 202
- [ ] Because the server held the HTTP connection open and awaited the full SMTP dialog before replying
- [ ] Because the mail server returned 502 Bad Gateway
- [ ] Because the database locked for 8 seconds on insert

**2. Why are Mailer::Console and Mailer::Capture kept synchronous instead of using tokio::spawn?**

- [ ] Because the console mailer cannot run in async Rust
- [ ] To avoid a race condition in unit tests when asserting immediately after the handler call
- [ ] Because tokio::spawn is disabled in dev builds
- [ ] To reduce DNS traffic in tests

**3. What happens to an error if you don't log it inside tokio::spawn?**

- [ ] Axum automatically sends 500 Internal Server Error to the client
- [ ] The process panics immediately
- [ ] The HTTP handler has already returned 202, so the failure goes unnoticed
- [ ] Tokio automatically retries sending the email

**4. Why wrap connect and send in tokio::time::timeout inside the background task?**

- [ ] So the client receives 202 Accepted faster
- [ ] So a slow or unresponsive MX server doesn't leave the task hanging for minutes
- [ ] Because tokio::spawn won't compile without a timeout
- [ ] To bypass the 60-second cooldown in the database

**5. Why won't a user tapping the button twice trigger two background emails?**

- [ ] Because tokio::spawn deduplicates identical tasks
- [ ] Because saving the code to the DB and checking the 60-second cooldown happen before tokio::spawn
- [ ] Because the mail server drops duplicate emails
- [ ] Because Flutter disables the button for 10 seconds

🔗 **Docs:** [tokio::spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)

