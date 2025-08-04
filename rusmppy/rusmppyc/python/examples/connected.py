from rusmppyc import Events, Client

import asyncio


async def handle_events(events: Events):
    async for event in events:
        print(f"Received event: {event}")

    print("Event handling completed.")


async def main():
    read, write = await asyncio.open_connection("127.0.0.1", 2775)

    client, events = await Client.connected(
        read,
        write,
        enquire_link_interval=5,
        enquire_link_response_timeout=2,
        response_timeout=2,
    )

    asyncio.create_task(handle_events(events))

    response = await client.bind_transceiver(
        system_id="test",
        password="test",
    )

    print(f"Bind response: {response}")

    await client.unbind()
    await client.close()
    await client.closed()

    await asyncio.sleep(3)

    # At this point the tcp connection is not closed
    # Rust does NOT close the StreamWriter
    write.close()
    await write.wait_closed()


if __name__ == "__main__":
    asyncio.run(main())
