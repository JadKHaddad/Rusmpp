use rusmpp_io::io::{
    read::{AsyncIoRead, AsyncIoReadWithLength},
    write::AsyncIoWrite,
};
use std::{fmt::Debug, io::Cursor};

pub async fn defaut_write_read_compare<T>()
where
    T: Default + Debug + PartialEq + AsyncIoWrite + AsyncIoRead,
{
    let t = T::default();

    let mut curser = Cursor::new(Vec::new());

    t.async_io_write(&mut curser)
        .await
        .expect("Failed to write bytes");

    curser.set_position(0);

    let t_read = T::async_io_read(&mut curser)
        .await
        .expect("Failed to read bytes");

    assert_eq!(t, t_read);
}

pub async fn defaut_write_read_with_length_compare<T>()
where
    T: Default + Debug + PartialEq + AsyncIoWrite + AsyncIoReadWithLength,
{
    let t = T::default();

    let mut curser = Cursor::new(Vec::new());

    t.async_io_write(&mut curser)
        .await
        .expect("Failed to write bytes");

    curser.set_position(0);

    let t_read = T::async_io_read(&mut curser, t.length())
        .await
        .expect("Failed to read bytes");

    assert_eq!(t, t_read);
}
