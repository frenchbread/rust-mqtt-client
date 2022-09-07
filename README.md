MQTT Rust app
====

> Client application for publishing and subscribing to messages served by EMQX broker.

[Create](https://cloud-intl.emqx.com/console/) own cloud instance of EMQX Broker for testing or [download](https://www.emqx.io/downloads) and run locally.

### Setup

Rename `.env.template to .env` provide env variables.

- **BROKER_HOST** - EMQX broker host
- **CLIENT_NAME** - client name
- **PUB_TOPIC** - topic to publish to e.g. testtopic/1
- **SUB_TOPICS** - list of topics to subscribe to (separated by comma)
- **QOSS** - QOS value(s) for topics subscribtions (separated by comma)
- **USERNAME** - emqx broker user username
- **PASSWORD** - emqx broker user password

### Build

```
❯ cargo build
```

### Subscribe

Run this command to subscribe to topic(s):
```
❯ ./target/debug/sub
```

### Publish

And this to publish 5 "hello world" messages:

```
❯ ./target/debug/publish
```

### todos

- [x] move sensitive variables to env
- [x] decouple code for connection to separate module and reuse for both pub&sub
- [ ] integrate with tauri application

### LICENSE

[MIT](https://github.com/frenchbread/rust-mqtt-client/blob/main/LICENSE)
