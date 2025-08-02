from rusmppyc_sys import Client, Events
import asyncio

async def handle_events(events: Events):
    async for event in events:
        print(f"Received event: {event}")

async def main():
    client, events = await Client.connect(host="127.0.0.1:2775")

    asyncio.create_task(handle_events(events))

    response = await client.bind_transceiver(
        system_id="test",
        password="test",
    )

    print(f"Bind response: {response}")

if __name__ == "__main__":
    asyncio.run(main())