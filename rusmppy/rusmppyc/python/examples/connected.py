import logging
import asyncio

from rusmppyc import (
    BindTransceiverResp,
    Client,
    CommandId,
    Event,
    Events,
)
from rusmppyc.exceptions import RusmppycException


async def handle_events(events: Events, client: Client):
    async for event in events:
        match event:
            case Event.Incoming(cmd):
                logging.debug(f"Received Command: {cmd.id}")

                match cmd.id:
                    case CommandId.DeliverSm():
                        try:
                            await client.deliver_sm_resp(
                                cmd.sequence_number, "the message id"
                            )
                        except RusmppycException as e:
                            logging.error(f"Failed to send DeliverSm response: {e}")

            case Event.Error(err):
                logging.error(f"Error occurred: {err}")
            case _:
                logging.warning(f"Unknown event: {event}")

    logging.debug("Event handling completed")


async def main():
    read, write = await asyncio.open_connection("127.0.0.1", 2775)

    try:
        # Use Client.connected to create a client with an existing StreamReader and StreamWriter
        client, events = await Client.connected(
            read,
            write,
            enquire_link_interval=5000,
            enquire_link_response_timeout=2000,
            response_timeout=2000,
        )

        asyncio.create_task(handle_events(events, client))

        response: BindTransceiverResp = await client.bind_transceiver(
            system_id="test",
            password="test",
        )

        logging.info(f"Bind response: {response}")
        logging.info(f"Bind response system_id: {response.system_id}")
        logging.info(
            f"Bind response sc_interface_version: {response.sc_interface_version}"
        )

        await asyncio.sleep(2)

        await client.unbind()
        await client.close()
        await client.closed()

        logging.debug("RUSMPP connection closed")

    except RusmppycException as e:
        logging.error(f"An error occurred: {e}")

    finally:
        # At this point the tcp connection is not closed
        # Rust does NOT close the StreamWriter

        write.close()

        await write.wait_closed()

        logging.debug("TCP connection fully closed")


if __name__ == "__main__":
    # Blue
    logging.addLevelName(
        logging.DEBUG, "\033[1;34m%s\033[1;0m" % logging.getLevelName(logging.DEBUG)
    )
    # Green
    logging.addLevelName(
        logging.INFO, "\033[1;32m%s\033[1;0m" % logging.getLevelName(logging.INFO)
    )
    # Yellow
    logging.addLevelName(
        logging.WARNING, "\033[1;33m%s\033[1;0m" % logging.getLevelName(logging.WARNING)
    )
    # Red
    logging.addLevelName(
        logging.ERROR, "\033[1;31m%s\033[1;0m" % logging.getLevelName(logging.ERROR)
    )
    # White on Red Background
    logging.addLevelName(
        logging.CRITICAL,
        "\033[1;37;41m%s\033[1;0m" % logging.getLevelName(logging.CRITICAL),
    )

    logging.basicConfig(
        format="%(asctime)-15s %(levelname)s %(name)s %(filename)s:%(lineno)d %(message)s"
    )

    logging.getLogger().setLevel(logging.DEBUG)
    logging.getLogger("rusmpp").setLevel(logging.INFO)

    # Avoid windows_events.py:859 <IocpProactor overlapped#=1 result#=0> is running after closing for xx.x seconds
    # asyncio.set_event_loop_policy(asyncio.WindowsSelectorEventLoopPolicy())

    asyncio.run(main())
