use std::future::Future;

use bytes::Bytes;
use futures::{Sink, Stream};
use pyo3::{types::PyBytes, PyErr, PyObject, PyResult, Python};
use tokio::io::{AsyncRead, AsyncWrite};

pub trait IO {
    fn into_tokio_async_read_and_write(self) -> impl AsyncRead + AsyncWrite;
}

impl IO for (PyObject, PyObject) {
    // Converts a tuple of Python asyncio StreamReader and StreamWriter to a Tokio AsyncRead and AsyncWrite.
    fn into_tokio_async_read_and_write(self) -> impl AsyncRead + AsyncWrite {
        tokio::io::join(
            async_io_stream_reader_to_async_read(self.0),
            async_io_stream_writer_to_async_write(self.1),
        )
    }
}

/// Converts a Python [asyncio.StreamReader](https://docs.python.org/3/library/asyncio-stream.html#streamreader) to a [`Stream`].
fn async_io_stream_reader_to_stream(reader: PyObject) -> impl Stream<Item = PyResult<Bytes>> {
    futures::stream::unfold(reader, |reader| async move {
        let extract = async {
            let fut = Python::with_gil(|py| {
                let awaitable = reader.call_method1(py, "read", (1024,))?;

                pyo3_async_runtimes::tokio::into_future(awaitable.into_bound(py))
            })?;

            let obj = fut.await?;

            let extracted = Python::with_gil(|py| obj.extract::<Vec<u8>>(py))?;

            PyResult::Ok(extracted)
        };

        match extract.await {
            Ok(bytes) if !bytes.is_empty() => Some((Ok(Bytes::from(bytes)), reader)),
            Ok(_) => {
                // EOF, terminate stream
                None
            }
            Err(err) => Some((Err(err), reader)),
        }
    })
}

/// Converts a Python [asyncio.StreamReader](https://docs.python.org/3/library/asyncio-stream.html#streamreader) to an [`AsyncRead`].
fn async_io_stream_reader_to_async_read(stream_reader: PyObject) -> impl AsyncRead {
    tokio_util::io::StreamReader::new(async_io_stream_reader_to_stream(stream_reader))
}

/// Converts a Python [asyncio.StreamWriter](https://docs.python.org/3/library/asyncio-stream.html#streamwriter) to a [`Sink`].
fn async_io_stream_writer_to_sink(writer: PyObject) -> impl for<'a> Sink<&'a [u8], Error = PyErr> {
    fn f(writer: PyObject, data: &[u8]) -> impl Future<Output = Result<PyObject, PyErr>> + 'static {
        let write_result =
            Python::with_gil(|py| writer.call_method1(py, "write", (PyBytes::new(py, data),)));

        async {
            write_result?;

            let fut = Python::with_gil(|py| {
                let awaitable = writer.call_method0(py, "drain")?;

                pyo3_async_runtimes::tokio::into_future(awaitable.into_bound(py))
            })?;

            fut.await?;

            Ok(writer)
        }
    }

    futures::sink::unfold(writer, f)
}

/// Converts a Python [asyncio.StreamWriter](https://docs.python.org/3/library/asyncio-stream.html#streamwriter) to an [`AsyncWrite`].
fn async_io_stream_writer_to_async_write(stream_writer: PyObject) -> impl AsyncWrite {
    tokio_util::io::SinkWriter::new(async_io_stream_writer_to_sink(stream_writer))
}
