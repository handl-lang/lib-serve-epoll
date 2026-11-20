# lib-serve-epoll

`serve` wired to `concurrent-epoll` — accept loop, TLS, HTTP codec and
WebSocket upgrade per connection task.

## Layout

```text
src/index.hnl            the .hnl sources
src/serve-epoll.test.hnl the .hnl test suite
pkg.hnl                  package manifest — modules, deps, description, license
ci.sh                    bupha entry → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
