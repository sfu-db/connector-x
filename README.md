# connector-agent [![status][ci_badge]][ci_page] [![docs][docs_badge]][docs_page]

[ci_badge]: https://github.com/sfu-db/connector-agent/workflows/ci/badge.svg
[ci_page]: https://github.com/sfu-db/connector-agent/actions

[docs_badge]: https://github.com/sfu-db/connector-agent/workflows/docs/badge.svg
[docs_page]: https://sfu-db.github.io/connector-agent/connector_agent/

Load data from <img src="assets/sources.gif" width="8%"/> to <img src="assets/destinations.gif" width="7%"/>.
## Environment Setup
* Install rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Install nightly: `rustup toolchain install nightly-2021-02-15 && rustup override set nightly-2021-02-15`
* Install Just: `cargo install just`