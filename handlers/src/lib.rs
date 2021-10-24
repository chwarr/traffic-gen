/*
Copyright 2021, G. Christopher Warrington <code@cw.codes>

This set of tools is free software: you can redistribute it and/or modify it
under the terms of the GNU Affero General Public License Version 3 as
published by the Free Software Foundation.

This set of tools is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
License for more details.

A copy of the GNU Affero General Public License Version 3 is included in the
file LICENSE in the root of the repository.

SPDX-License-Identifier: AGPL-3.0-only
*/

use rand::prelude::*;
use std::io::Result;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};

pub async fn read(mut read_socket: ReadHalf<TcpStream>) -> Result<()> {
    let mut buf = vec![0; 4096];

    const PROGRESS_BYTES_COUNT: usize = 1 * 1024 * 1024 * 1024;
    let mut bytes_read_acc: usize = 0;

    loop {
        let bytes_read = read_socket.read(&mut buf).await?;
        bytes_read_acc += bytes_read;

        if bytes_read_acc >= PROGRESS_BYTES_COUNT {
            eprint!("r");
            bytes_read_acc = 0;
        }

        if bytes_read == 0 {
            return Ok(());
        }
    }
}

pub async fn write(mut write_socket: WriteHalf<TcpStream>) -> Result<()> {
    let mut buf: Vec<u8> = vec![0; 4096];

    const PROGRESS_BYTES_COUNT: usize = 1 * 1024 * 1024 * 1024;
    let mut bytes_written_acc: usize = 0;

    loop {
        thread_rng().fill(&mut buf[..]);
        write_socket.write_all(&buf).await?;

        bytes_written_acc += buf.len();
        if bytes_written_acc >= PROGRESS_BYTES_COUNT {
            eprint!("w");
            bytes_written_acc = 0;
        }
    }
}
