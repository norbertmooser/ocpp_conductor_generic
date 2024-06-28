import json

# Base configuration for Redis connection
redis_connection = "redis://127.0.0.1/"

# Starting ports for REST and WebSocket services
rest_start_port = 8081
websocket_start_port = 8080

# Generate the list of charger configurations
chargers = []
for i in range(1, 101):
    charger_name = f"Charger{i:03d}"
    rest_port = rest_start_port + (i - 1) * 2  # Ensure unique ports
    websocket_port = websocket_start_port + (i - 1) * 2 + 2  # Ensure unique ports
    charger_config = {
        "name": charger_name,
        "rest_service_addr": f"127.0.0.1:{rest_port}",
        "websocket_service_addr": f"127.0.0.1:{websocket_port}",
        "redis_connection": redis_connection
    }
    chargers.append(charger_config)

# Create the final configuration structure
config = {
    "chargers": chargers
}

# Write the configuration to a JSON file
with open('charger_configs.json', 'w') as json_file:
    json.dump(config, json_file, indent=4)

print("charger_configs.json has been generated.")
