
from rusmppyc import Events, Client, BindTransceiverResp

import asyncio

async def handle_events(events: Events):
    async for event in events:
        print(f"Received event: {event}")

async def main():
    client, events = await Client.connect(host="127.0.0.1:2775", enquire_link_interval=5, response_timeout=10)

    asyncio.create_task(handle_events(events))

    response: BindTransceiverResp = await client.bind_transceiver(
        system_id="test",
        password="test",
    )

    print(f"Bind response: {response}")
    print(f"Bind response system_id: {response.system_id}")
    print(f"Bind response sc_interface_version: {response.sc_interface_version}")

if __name__ == "__main__":
    asyncio.run(main())