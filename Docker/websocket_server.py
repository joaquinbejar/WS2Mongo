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

async def websocket_handler(websocket, path):
    print(f"New client connected with IP: {websocket.remote_address}")
    while True:
        try:
            message = json.dumps(random_json(4))
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
