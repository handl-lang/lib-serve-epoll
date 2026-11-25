# lib-serve-epoll

`serve` wired to `concurrent-epoll` — accept loop, TLS, HTTP codec and
WebSocket upgrade per connection task. Pure `.hnl`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the implementation |
| `src/serve-epoll.test.hnl` | the .hnl test suite |
| `pkg.hnl` | manifest — `modules: ["serve-epoll"]`, deps `serve`, `concurrent-epoll`, `http_rfc`, `tls-libssl`, `ws_frame`, `ws_frame-handl`, `ws-whatwg-c` |
| `scripts/hnl-gate.sh`, `workflows/gate.yaml` | the gate / bupha entry |

## Contract

- Per-connection task model on the epoll scheduler; TLS via
  `tls-libssl`, HTTP codec via `http_rfc`, WS upgrade via the
  `ws_*` packages.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
