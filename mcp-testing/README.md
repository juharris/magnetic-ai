To use in Cursor:
```shell
npm run start-http
```

Then in Cursor, add a new MCP server in `~/.cursor/mcp.json`:
```JSON
{
  "mcpServers": {
    "example": {
      "url": "http://localhost:3000/sse"
    }
  }
}
```
