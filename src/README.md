The WebSocketGateway is solely responsible for setting up the WebSocket server, accepting connections, and managing the basic message flows (incoming and outgoing) without processing them.
The main.rs takes full responsibility for what happens with those messages once received or before being sent, which includes calling handle_incoming_messages.

WebSocketGateway struct. This method sets up a WebSocket server, accepts connections, and manages the message flows between the WebSocket clients and the server.

Here is a breakdown of what the run method does:
1.
It binds a TCP listener to the specified address.
2.
It enters a loop to accept incoming connections.
3.
For each incoming connection:
It clones the incoming_tx and outgoing_rx channels for handling messages.
It spawns a new asynchronous task to handle the WebSocket connection.
Within the task:
It accepts the WebSocket connection.
It splits the WebSocket stream into read and write halves.
It reads messages from the WebSocket stream and forwards them to the incoming_tx channel.
It reads messages from the outgoing_rx channel and sends them to the WebSocket client.


Overall, the run method is responsible for managing the bidirectional message flow between the WebSocket clients and the server using the provided channels for communication.