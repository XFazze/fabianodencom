# Fabianoden.com

Frontend in rust yew. Using tailwindcss cli.

## Dev enviroment

### Dependencies

- Yew dependencies[guide](https://yew.rs/docs/getting-started/introductionl)
- Actix dependencies
- Docker-compose
- Tailwindcss cli

### Commands

To generate frontend:
`trunk watch`

To run backend:
`cargo run`

To run backend with watch:
(Install cargo watch `cargo install cargo-watch`)
`cargo watch -x run`

To build prod frontend:
`docker-compose up`

Tailwindcss watch:
`./tailwindcss src/static/style/input.css -o src/static/style/output.css --watch`

Tailwindcss build production:
`./tailwindcss src/static/style/input.css -o src/static/style/output.css --minimize`

### Help/doc Websites

- [Tailwindcss](https://tailwindcss.com/docs)
- [Yew router](https://yew.rs/docs/concepts/router)
- [Yew](https://yew.rs/docs/getting-started/introduction)
- [Yew hooks](https://docs.rs/yew-hooks/latest/yew_hooks/index.html)
- [actix yew example](https://github.com/security-union/yew-actix-template/blob/main/actix-api/src/main.rs)
- [actix yew ssr yt guide](https://www.youtube.com/watch?v=uYhLWN86V48&list=PL2q9pua8FpiUiCv6KmWWhR5Bh8GfElo98&index=8)
- [actix yew ssr yt guide repo](https://github.com/Me163/youtube/tree/main/FullStackRust)
