import asyncio
import websockets
import json
import random
from faker import Faker

def generate_random_user():
    fake = Faker()
    return {
        "name": fake.name(),
        "address": fake.address(),
        "email": fake.email()
    }

def basic_type():
    switcher = random.choice([
        lambda: random.randint(1, 100),
        lambda: random.random(),
        lambda: random.choice(["apple", "banana", "cherry"]),
        lambda: random.choice([True, False])
    ])
    return switcher()

def random_value():
    switcher = random.choice([
        lambda: random.randint(1, 100),
        lambda: random.random(),
        lambda: random.choice(["apple", "banana", "cherry"]),
        lambda: random.choice([True, False]),
        lambda: list(set([basic_type() for _ in range(random.randint(1, 10))])),
        lambda: None,
        lambda: [generate_random_user() for _ in range(random.randint(1, 10))]
    ])
    return switcher()

def random_json(depth=0):
    if depth > 5:
        return random_value()

    if random.choice([True, True,False]):
        # Crea un diccionario con claves y valores aleatorios
        return {f'key_{i}': random_json(depth + 1) for i in range(random.randint(1, 5))}
    else:
        return {f'key': random_json(depth + 1)}


async def send_random_json(websocket):
    """ Helper function to send random JSON data to the client """
    message = json.dumps(random_json(4))
    await websocket.send(message)
    await asyncio.sleep(1)
    return message

async def websocket_handler(websocket, path):
    print(f"New client connected with IP: {websocket.remote_address}")
    try:
        while True:
            # Espera por un mensaje del cliente
            receive_task = asyncio.ensure_future(websocket.recv())
            send_task = asyncio.ensure_future(send_random_json(websocket))

            done, pending = await asyncio.wait(
                [receive_task, send_task],
                return_when=asyncio.FIRST_COMPLETED
            )

            if receive_task in done:
                message = receive_task.result()
                print(f"Received from {websocket.remote_address}: {message}")
            else:
                receive_task.cancel()

            if send_task in done:
                print(f"Sent to {websocket.remote_address}: {send_task.result()}")
            else:
                send_task.cancel()

    except websockets.exceptions.ConnectionClosed:
        print(f"Connection with client {websocket.remote_address} closed")

start_server = websockets.serve(websocket_handler, '0.0.0.0', 5678)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()
