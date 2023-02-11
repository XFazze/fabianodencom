# Fabianoden.com

Website in rust yew. Using tailwindcss cli and docker.

## Dev enviroment

### Dependencies

- Yew dependencies[guide](https://yew.rs/docs/getting-started/introductionl)
- Docker-compose

### Commands

To run dev server:
`cargo serve`

To build prod:
`trunk build --release`

Tailwindcss watch:
`./tailwindcss src/static/style/input.css -o src/static/style/output.css --watch`

Tailwindcss build production:
`./tailwindcss src/static/style/input.css -o src/static/style/output.css --minimize`

### Help/doc Websites

- [Tailwindcss](https://tailwindcss.com/docs)
- [Yew router](https://yew.rs/docs/concepts/router)
- [Yew](https://yew.rs/docs/getting-started/introduction)
- [Yew hooks](https://docs.rs/yew-hooks/latest/yew_hooks/index.html)
