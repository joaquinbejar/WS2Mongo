import asyncio
import websockets
import json
import random

async def websocket_handler(websocket, path):
    print(f"New client connected with IP: {websocket.remote_address}")
    while True:
        try:
            message = json.dumps({'number': random.randint(1, 100)})
            await websocket.send(message)
            await asyncio.sleep(1)  # espera 1 segundo entre mensajes
        except websockets.exceptions.ConnectionClosed:
            print(f"Connection with client {websocket.remote_address} closed")
            break
        else:
            print(f"Sent: {message}")


start_server = websockets.serve(websocket_handler, '0.0.0.0', 5678)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()
