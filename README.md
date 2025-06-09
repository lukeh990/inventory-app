<div align="center">
<h1>Inventory Tracker</h1>
<img src="assets/logo.png" alt="Project Logo" width="100">
</div>

A quick terminal based inventory tracker.

**Not Ready For Production**

## Goal

The goal of this project is to provide a self-hostable flexible, unique, and efficent interface for managing inventory.

## Design

The following is a rough outline of how this project will work.┊

```txt
┏━━━━━━━━━━━━┓   ┏━━━━━━━━━━━━━━━┓   ┏━━━━━━━━━━━━┓   ┏━━━━━━━━━━┓
┃ Web Client ┃┈┈┈┃ Reverse Proxy ┃┈┈┈┃ API Server ┃┈┈┈┃ Database ┃
┗━━━━━━━━━━━━┛   ┗━━━━━━━━━━━━━━━┛   ┗━━━━━━━━━━━━┛   ┗━━━━━━━━━━┛
                                            ┊
                                     ┏━━━━━━━━━━━━┓
                                     ┃ CLI Client ┃
                                     ┗━━━━━━━━━━━━┛
```

A connection would look roughly like this:

1. The web client would request a web socket with the API server.
2. The web server spawns an instance of the CLI Client and presents the terminal output to the websocket.
3. The web client would use a web terminal to display the content.
4. The web client would send any interactions to the server

### Reasoning

Why use a web terminal and why not program the client logic into the web client? I want the CLI client to be usable in a standalone environment and I want the web client to have the same experience.

### Tech Stack

**Reverse Proxy** - NGINX

**Databse** - PostgreSQL

#### Web Client

- [xterm.js](https://github.com/xtermjs/xterm.js) - Terminal
- [Vite](https://vite.dev/) - Build Engine
- [Typescript](https://www.typescriptlang.org/) - Language

#### API Server

- [actix_web](https://actix.rs/) - Web Framework
- [sqlx](https://github.com/launchbadge/sqlx) - SQL Toolkit
- [tokio](https://tokio.rs/) - Asynchronous Runtime
- [Rust](https://www.rust-lang.org/) - Language

#### CLI Client

- [ratatui](https://ratatui.rs/) - CLI Framework
- [reqwest](https://crates.io/crates/reqwest) - HTTP Client
- [tokio](https://tokio.rs/) - Asynchronous Runtime
- [Rust](https://www.rust-lang.org/) - Language

## To Do

- [ ] Web Client App
  - [x] Terminal Shell
  - [ ] Interactive Terminal
  - [ ] Attach to Server
- [ ] API Server
  - [ ] Server Shell
    - [ ] Logging
    - [ ] Config
    - [ ] Multi-tasking
  - [ ] Database Connection
    - [ ] Schema
    - [ ] API
  - [ ] Web Sockets
    - [ ] Authentication Challenge
    - [ ] Span Sub-Processes
  - [ ] REST API
    - [ ] Create
    - [ ] Read
    - [ ] Update
    - [ ] Delete
- [ ] CLI Client
  - [ ] Item Management
  - [ ] Location Management
  - [ ] Quanity Management

## API Server

### Environment Variable

- `CONF_FILE` | The location of the config file. | Default: `config.toml`
