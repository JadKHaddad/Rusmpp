
from rusmppyc import Events, Client, BindTransceiverResp, Event
from rusmppyc.exceptions import RusmppycException

import asyncio

async def handle_events(events: Events):
    async for event in events:
        match event:
            case Event.Incoming(cmd):
                print(f"Received Command: {cmd}")
            case Event.Error(err):
                print(f"Error occurred: {err}")
            case _:
                print(f"Unknown event: {event}")
    
    print("Event handling completed.")

async def main():
    try:
        client, events = await Client.connect(host="127.0.0.1:2775", enquire_link_interval=5, enquire_link_response_timeout=2, response_timeout=2)

        asyncio.create_task(handle_events(events))

        response: BindTransceiverResp = await client.bind_transceiver(
            system_id="test",
            password="test",
        )

        print(f"Bind response: {response}")
        print(f"Bind response system_id: {response.system_id}")
        print(f"Bind response sc_interface_version: {response.sc_interface_version}")

        await asyncio.sleep(10)  # Keep the connection alive for a while

        await client.unbind()
        await client.close()
        await client.closed()
    
    except RusmppycException as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    asyncio.run(main())