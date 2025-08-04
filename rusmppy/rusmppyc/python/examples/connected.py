from rusmppyc import Events, Client, BindTransceiverResp, Event, CommandId
from rusmppyc.exceptions import RusmppycException


import asyncio


async def handle_events(events: Events, client: Client):
    async for event in events:
        match event:
            case Event.Incoming(cmd):
                print(f"Received Command: {cmd.id}")

                match cmd.id:
                    case CommandId.DeliverSm():
                        # TODO: main does not want to terminate when using this line. Maybe because of the exception or wait_close
                        try:
                            await client.deliver_sm_resp(
                                cmd.sequence_number, "the message id"
                            )
                        except:
                            pass

            case Event.Error(err):
                print(f"Error occurred: {err}")
            case _:
                print(f"Unknown event: {event}")

    print("Event handling completed.")


async def main():
    try:
        read, write = await asyncio.open_connection("127.0.0.1", 2775)

        client, events = await Client.connected(
            read,
            write,
            enquire_link_interval=5,
            enquire_link_response_timeout=2,
            response_timeout=2,
        )

        asyncio.create_task(handle_events(events, client))

        response: BindTransceiverResp = await client.bind_transceiver(
            system_id="test",
            password="test",
        )

        print(f"Bind response: {response}")
        print(f"Bind response system_id: {response.system_id}")
        print(f"Bind response sc_interface_version: {response.sc_interface_version}")

        await asyncio.sleep(10)

        await client.unbind()
        await client.close()
        await client.closed()

        print("RUSMPP connection closed")

    except RusmppycException as e:
        print(f"An error occurred: {e}")

    finally:
        # At this point the tcp connection is not closed
        # Rust does NOT close the StreamWriter
        write.close()
        await write.wait_closed()

        print("TCP connection fully closed")


if __name__ == "__main__":
    asyncio.run(main())
