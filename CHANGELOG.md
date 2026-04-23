# Changelog

## [1.1.0](https://github.com/davidjpnelson/linear-mcp-rs/compare/v1.0.3...v1.1.0) (2026-04-23)


### Features

* complete Linear API coverage phase 2 — add 142 new tools (253 total) ([f2e53df](https://github.com/davidjpnelson/linear-mcp-rs/commit/f2e53dfe3a9c868b939d97f925077e2ca3c4b41e))
* expand from 68 to 115 tools with full Linear API coverage ([9b5cfaf](https://github.com/davidjpnelson/linear-mcp-rs/commit/9b5cfaf31db541f20a5b0cfa8af1c9cf60a9f1fa))


### Bug Fixes

* correct 20+ GraphQL schema mismatches and add comprehensive test harness ([2df5593](https://github.com/davidjpnelson/linear-mcp-rs/commit/2df559325f308d4adcb4ce92a742f355a0ed7a86))
* **deps:** rustls-webpki 0.103.12 -&gt; 0.103.13 (RUSTSEC-2026-0104) ([7ed5582](https://github.com/davidjpnelson/linear-mcp-rs/commit/7ed5582d2ee55167c2872d3d8bf65fcae3b03be0))
* **deps:** rustls-webpki 0.103.13 (RUSTSEC-2026-0104) + v1.0.2 ([b148233](https://github.com/davidjpnelson/linear-mcp-rs/commit/b1482331660736419ff39c58521c85a3374875cf))
* expand test harness to 254 tools, fix GraphQL schema mismatches ([66d20a0](https://github.com/davidjpnelson/linear-mcp-rs/commit/66d20a0b1489c545d0b1797779810ed8016bd984))
* move team filter to top level in label resolution ([4b74034](https://github.com/davidjpnelson/linear-mcp-rs/commit/4b7403481212d67cc5a93512e3539eb90ed2f072))
* reduce LIST_PROJECTS query complexity ([e687863](https://github.com/davidjpnelson/linear-mcp-rs/commit/e68786318c37363609a5cb463dd94039004bfb16))
* reduce LIST_PROJECTS query complexity to stay under Linear's 10k limit ([7fa7267](https://github.com/davidjpnelson/linear-mcp-rs/commit/7fa72677e327ebf6bd842e7d290e3043de40b0a8))
* remove deprecated roadmap tools, fix 6 bugs found in live testing ([a7bca46](https://github.com/davidjpnelson/linear-mcp-rs/commit/a7bca46127c2432ba95030f5624090f60a6f78e7))
* scope label resolution to issue's team ([e4deb45](https://github.com/davidjpnelson/linear-mcp-rs/commit/e4deb451a6063e24923ef4003cb146c5cb178aee))
* server-side team filtering for list_projects ([8a8e177](https://github.com/davidjpnelson/linear-mcp-rs/commit/8a8e1770d3c99df5555626eb9b44e100de2787e5))
* use server-side team filtering for list_projects ([2c255ab](https://github.com/davidjpnelson/linear-mcp-rs/commit/2c255abed393358022e9f33decc0b09e11432f5c))
