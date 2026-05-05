# axum-ws-otel

```
$ read OTEL_EXPORTER_OTLP_HEADERS
authorization=...
export OTEL_EXPORTER_OTLP_HEADERS
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318
cargo run
```

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
