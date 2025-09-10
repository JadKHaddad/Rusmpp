# Rusmppyc

[![PyPI - License](https://img.shields.io/pypi/l/rusmppyc)](https://github.com/JadKHaddad/Rusmpp?tab=readme-ov-file#license)
![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2FJadKHaddad%2FRusmpp%2Frefs%2Fheads%2Fmain%2Frusmppy%2Frusmppyc%2Fpyproject.toml&logo=python)
[![PyPI](https://img.shields.io/pypi/v/rusmppyc?logo=python)](https://pypi.org/project/rusmppyc/)
[![PyPI Downloads](https://static.pepy.tech/badge/rusmppyc)](https://pepy.tech/projects/rusmppyc)

An async [SMPP v5](https://smpp.org/SMPP_v5.pdf) `Python` client powered by `Rust`.

## Example

```python
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
    logging.basicConfig(
        format="%(asctime)-15s %(levelname)s %(name)s %(filename)s:%(lineno)d %(message)s"
    )

    logging.getLogger().setLevel(logging.DEBUG)
    logging.getLogger("rusmpp").setLevel(logging.INFO)
    logging.getLogger("rusmppc").setLevel(logging.INFO)
    logging.getLogger("rusmppyc").setLevel(logging.DEBUG)

    asyncio.run(main())
```

## Develop

- Install [`maturin`](https://www.maturin.rs/installation.html)

- Create a virtual environment:

  ```bash
  python3 -m venv venv
  source venv/bin/activate
  ```

- Generate the `pyi` stubs:

  ```bash
  cargo run --bin stub-gen
  ```

- Generate the bindings:

  ```bash
  maturin develop
  ```

- The bindings are now available in the virtual environment. You can test them by running:

  ```bash
  python3 -c "import rusmppyc; print(rusmppyc.__version__)"
  ```
