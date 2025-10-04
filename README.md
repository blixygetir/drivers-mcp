# MCP Server in Rust

+-------------------------+
|       Project Overview  |
+-------------------------+

This project implements an **MCP (Model Context Protocol) server** in **Rust**.  
It aims to **simplify user access to drivers** by providing a central server interface for interacting with different hardware or software drivers safely and efficiently.

+-------------------------+
|      Features           |
+-------------------------+

| Feature                         | Description                                      |
|---------------------------------|-------------------------------------------------|
| Fast & Safe                     | Written in Rust, leveraging memory safety       |
| Unified Driver Access           | Provides a single interface to multiple drivers |
| Linux-Only                      | Currently works only on Linux systems           |

+-------------------------+
|      Setup & Usage       |
+-------------------------+



+-------------------------+
|    Build MCP Server     |
+-------------------------+
Build MCP Server:

```bash
cargo build -p drivers-mcp --release
```

```bash
npx -y @modelcontextprotocol/inspector ./target/release/drivers-mcp
```



+-------------------------+
|      Contact             |
+-------------------------+

- Author: Aaryan
- GitHub: [https://github.com/blixygetir/drivers-mcp](https://github.com/blixygetir/drivers-mcp)
