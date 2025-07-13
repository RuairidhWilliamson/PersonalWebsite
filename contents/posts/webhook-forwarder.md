# Webhook Forwarder

May 2025

Rust, Web

---

![](/assets/images/fallback.png)

When developing services that rely on external webhooks it is difficult to test and run the service locally, because the external webhook provider cannot reach your local server sitting behind firewalls and NAT (network address translation). The common solution to this is to use a service similar to [smee.io](https://smee.io). Smee is a public server that forwards webhooks to clients using SSE (server side events). This works because the client opens the connection to the server and remains connected in order to receive events, allowing it to work behind NAT and most firewalls.

The problem I ran into when developing with smee is that it parses the webhook request body as JSON and then re encodes it as JSON when it is forwarded to the client. This is fine except that some webhook providers will sign the webhook body so that it can be verified that it originates from a trusted source (even more important when using a webhook forwarder that you don't control). However, because smee has parsed the webhook body and re encoded it there maybe slight differences in the strings. These changes cause the signature verification to fail. This would be solved if smee didn't parse the body and just passed it as a byte array.

In fact smee has added a field to pass the raw body along side the parsed body. [https://github.com/probot/smee.io/pull/55](https://github.com/probot/smee.io/pull/55). This change has been merged but the public smee instance hasn't been updated.

Instead of waiting around for that to be updated and instead of hosting the smee server myself. I decided to make my own webhook forwarder. Naturally I wrote this in Rust. It provides a CLI tool to forward the webhooks to a local address. It also provides a rust crate interface to integrate directly into services. It also uses a similar design to smee making use of SSE (server side events). It provides all the HTTP headers and the HTTP body to the client allowing the signature to be verified.

The implementation is not ideal, for example it encodes the webhook body as an array of bytes which in JSON is represented as an array of numbers, not very efficient. But for a development tool it is okay. If I run into issues I will improve it later.

The code can be found here [https://github.com/RuairidhWilliamson/webhook-forwarder](https://github.com/RuairidhWilliamson/webhook-forwarder)
