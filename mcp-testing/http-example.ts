import express from "express";
import { randomUUID } from "node:crypto";
import { StreamableHTTPServerTransport } from "@modelcontextprotocol/sdk/server/streamableHttp.js";
import { isInitializeRequest } from "@modelcontextprotocol/sdk/types.js"
import { SSEServerTransport } from "@modelcontextprotocol/sdk/server/sse.js";


import { McpServer, ResourceTemplate } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";

const server = new McpServer({
  name: "Example",
  version: "1.0.0"
});

server.resource(
  "echo",
  new ResourceTemplate("echo://{message}", { list: undefined }),
  async (uri, { message }) => ({
    contents: [{
      uri: uri.href,
      text: `Resource echo: ${message}`
    }]
  })
);

server.tool("add",
    { a: z.number(), b: z.number() },
    async ({ a, b }) => ({
      content: [{ type: "text", text: String(a + b) }]
    })
  );

server.tool(
  "echo",
  { message: z.string() },
  async ({ message }) => ({
    content: [{ type: "text", text: `Tool echo: ${message}` }]
  })
);

server.prompt(
  "echo",
  { message: z.string() },
  ({ message }) => ({
    messages: [{
      role: "user",
      content: {
        type: "text",
        text: `Please process this message: ${message}`
      }
    }]
  })
);

const app = express();
app.use(express.json());

const transports = {
  streamable: {} as Record<string, StreamableHTTPServerTransport>,
  sse: {} as Record<string, SSEServerTransport>
};


app.post('/mcp', async (req: express.Request, res: express.Response) => {
  // In stateless mode, create a new instance of transport and server for each request
  // to ensure complete isolation. A single instance would cause request ID collisions
  // when multiple clients connect concurrently.
  console.log('Received POST MCP request');
   // Check for existing session ID
   const sessionId = req.headers['mcp-session-id'] as string | undefined;
   let transport: StreamableHTTPServerTransport;
 
   if (sessionId && transports.streamable[sessionId]) {
     // Reuse existing transport
     transport = transports.streamable[sessionId];
   } else if (!sessionId && isInitializeRequest(req.body)) {
     // New initialization request
     transport = new StreamableHTTPServerTransport({
       sessionIdGenerator: () => randomUUID(),
       onsessioninitialized: (sessionId) => {
         // Store the transport by session ID
         transports.streamable[sessionId] = transport;
       }
     });
 
     // Clean up transport when closed
     transport.onclose = () => {
       if (transport.sessionId) {
         delete transports.streamable[transport.sessionId];
       }
     };

     // Connect to the MCP server
     await server.connect(transport);
   } else {
     // Invalid request
     res.status(400).json({
       jsonrpc: '2.0',
       error: {
         code: -32000,
         message: 'Bad Request: No valid session ID provided',
       },
       id: null,
     });
     return;
   }
 
   // Handle the request
   await transport.handleRequest(req, res, req.body);
});

// Reusable handler for GET and DELETE requests
const handleSessionRequest = async (req: express.Request, res: express.Response) => {
  const sessionId = req.headers['mcp-session-id'] as string | undefined;
  if (!sessionId || !transports.streamable[sessionId]) {
    res.status(400).send('Invalid or missing session ID');
    return;
  }
  
  const transport = transports.streamable[sessionId];
  await transport.handleRequest(req, res);
};

// Handle GET requests for server-to-client notifications via SSE
app.get('/mcp', handleSessionRequest);

// Handle DELETE requests for session termination
app.delete('/mcp', handleSessionRequest);


// Legacy SSE endpoint for older clients (needed for Cursor)
app.get('/sse', async (req, res) => {
  // Create SSE transport for legacy clients
  const transport = new SSEServerTransport('/messages', res);
  transports.sse[transport.sessionId] = transport;
  
  res.on("close", () => {
    delete transports.sse[transport.sessionId];
  });
  
  await server.connect(transport);
});

// Legacy message endpoint for older clients (needed for Cursor)
app.post('/messages', async (req, res) => {
  const sessionId = req.query.sessionId as string;
  const transport = transports.sse[sessionId];
  if (transport) {
    await transport.handlePostMessage(req, res, req.body);
  } else {
    res.status(400).send('No transport found for sessionId');
  }
});


// Start the server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`MCP Stateless Streamable HTTP Server listening on port ${PORT}`);
});
