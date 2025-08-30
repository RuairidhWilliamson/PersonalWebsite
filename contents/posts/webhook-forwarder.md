# Webhook Forwarder

May 2025

Rust, Web

---

![](/assets/images/fallback.png)

When developing services that rely on external webhooks it is difficult to test and run the service locally, because the external webhook provider cannot reach your local server sitting behind firewalls and NAT (network address translation). The common solution to this is to use a service similar to [smee.io](https://smee.io). Smee is a public server that forwards webhooks to clients using SSE (server side events). This works because the client opens the connection to the server and remains connected in order to receive events, allowing it to work behind NAT and most firewalls.

The problem I ran into when developing with smee is that it parses the webhook request body as JSON and then re encodes it as JSON when it is forwarded to the client. This is fine except that some webhook providers will sign the webhook body so that it can be verified that it originates from a trusted source (this is even more important when using a webhook forwarder which you don't control). However, because smee has parsed the webhook body and re encoded it there maybe slight differences in the strings. These changes cause the signature verification to fail. This would be solved if smee didn't parse the body and just passed it as a byte array.

In fact smee has added a field to pass the raw body along side the parsed body. [https://github.com/probot/smee.io/pull/55](https://github.com/probot/smee.io/pull/55). This change has been merged but the public smee instance hasn't been updated.

Instead of waiting around for that to be updated and instead of hosting the smee server myself. I decided to make my own webhook forwarder. Naturally I wrote this in Rust. It provides a CLI tool to forward the webhooks to a local address. It also provides a rust crate interface to integrate directly into services. It also uses a similar design to smee making use of SSE (server side events). It provides all the HTTP headers and the HTTP body to the client allowing the signature to be verified.

I had to put some work into making the client library work with async callbacks. The interface that client consumers implement is

```rust
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, headers: HeaderMap, body: Vec<u8>) -> Result<()>;
}
```

The client is generic over this trait but still uses `Box::pin` to call the async function. This callback didn't need to be async but seeing as most rust web servers make use of async it is beneficial to allow webhook forwarder to take part in that runtime and pass it on to the callback.

The server uses a concurrent hash map using dashmap to maintain a mapping of channel id to the client. By not using a mutex in theory there should be less resource contention. We don't want to clean up channels immediately if they exit because the client may reconnect. Instead at regular intervals all the closed channels are cleaned up.

I created a docker file to make it easy to host the server. I tried hosting this on Google Cloud's serverless cloud run platform, but the connections were ended after a timeout making it impractical to use for long periods of time.

The implementation is not ideal, for example it encodes the webhook body as an array of bytes which in JSON is represented as an array of numbers, not very efficient. But for a development tool it is okay. If I run into issues I will improve it later.

The code can be found here [https://github.com/RuairidhWilliamson/webhook-forwarder](https://github.com/RuairidhWilliamson/webhook-forwarder)
