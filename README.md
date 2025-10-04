+-------------------------+
|    MCP Server (Rust)    |
+-------------------------+

        MCP Server implements the Model Context Protocol (MCP)
        in Rust, simplifying user access to drivers by providing
        a central server interface for interacting with different
        hardware or software drivers safely and efficiently.

        Current Version: Linux-only

Features:

    - Fast & Safe: Written in Rust, leveraging memory safety
    - Unified Driver Access: Single interface for multiple drivers
    - Linux-Only: Currently works only on Linux systems

Setup & Usage:

    1. Build MCP Server:
        cargo build -p drivers-mcp --release

    2. Run MCP Inspector:
        npx -y @modelcontextprotocol/inspector ./target/release/drivers-mcp

Contact:

    Aaryan Dogra
    aaryan.dogra@hotmail.com

