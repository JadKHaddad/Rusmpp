import logging
import asyncio

from rusmppyc import (
    BindTransceiverResp,
    Client,
    CommandId,
    DataCoding,
    Event,
    Events,
    InterfaceVersion,
    Npi,
    SubmitSmResp,
    Ton,
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
    try:
        client, events = await Client.connect(
            host="127.0.0.1:2775",
            enquire_link_interval=5000,
            enquire_link_response_timeout=2000,
            response_timeout=2000,
            max_command_length=4096,
        )

        asyncio.create_task(handle_events(events, client))

        bind_response: BindTransceiverResp = await client.bind_transceiver(
            system_id="test",
            password="test",
            system_type="test",
            interface_version=InterfaceVersion.Smpp5_0(),
            addr_ton=Ton.Unknown(),
            addr_npi=Npi.National(),
        )

        logging.info(f"Bind response: {bind_response}")
        logging.info(f"Bind response system_id: {bind_response.system_id}")
        logging.info(
            f"Bind response sc_interface_version: {bind_response.sc_interface_version}"
        )

        submit_sm_response: SubmitSmResp = await client.submit_sm(
            source_addr_ton=Ton.International(),
            source_addr_npi=Npi.National(),
            source_addr="1234567890",
            dest_addr_ton=Ton.International(),
            dest_addr_npi=Npi.National(),
            destination_addr="0987654321",
            data_coding=DataCoding.Ucs2(),
            short_message=b"Hello, World!",
            message_payload=b"Big Big Message!" * 10,
        )

        logging.info(f"SubmitSm response: {submit_sm_response}")

        await asyncio.sleep(5)

        await client.unbind()
        await client.close()
        await client.closed()

        logging.debug("RUSMPP connection closed")

    except RusmppycException as e:
        logging.error(f"An error occurred: {e}")


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
    logging.getLogger("rusmppc").setLevel(logging.INFO)
    logging.getLogger("rusmppyc").setLevel(logging.DEBUG)

    asyncio.run(main())
