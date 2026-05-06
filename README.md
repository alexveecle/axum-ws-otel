# axum-ws-otel

Optional: configure OpenTelemetry traces delivery.

```
$ read OTEL_EXPORTER_OTLP_HEADERS
authorization=...
export OTEL_EXPORTER_OTLP_HEADERS
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318
```

Run the websocket server:

```
cargo run
```

To test, you can use uwsc (in Debian):

```
$ uwsc http://127.0.0.1:3000/ws
Websocket connected, you can send text messages of maximum 256 characters.
To exit uwsc, type !q<enter>
Server message: 'hola'
> foo
Send 'foo'
Server message: 'foo'
> bar
Send 'bar'
Server message: 'bar'
```

, or websocat ([in Brew](https://formulae.brew.sh/formula/websocat)):

```
$ websocat ws://127.0.0.1:3000/ws
hola  # sent by the server
foo   # typed in the terminal
foo   # server response
bar   # ...
bar
```
