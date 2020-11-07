// Copyright 2020 tearust
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;
use std::io::Cursor;
use futures::TryStreamExt;

// Creates an Ipfs client, and adds this source file to Ipfs.
//
#[cfg_attr(feature = "actix", actix_rt::main)]
#[cfg_attr(feature = "hyper", tokio::main)]
async fn main() {
    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let data = Cursor::new("Hello world!".as_bytes().to_vec());
    match client.block_put(data, true).await {
        Ok(res) => {
            eprintln!("block put hash: {}", &res.key);
            match client
                .block_get(&res.key)
                .map_ok(|chunk| chunk.to_vec())
                .try_concat()
                .await
            {
                Ok(r) => {
                    eprintln!("get result: {}", String::from_utf8(r.to_vec()).unwrap());
                }
                Err(e) => eprintln!("block get error: {:?}", e)
            }

            match client.pin_ls(Some(&res.key), None).await {
                Ok(r) => {
                    eprintln!("pinned keys: {:?}", r.keys)
                }
                Err(e) => eprintln!("pin ls error: {:?}", e)
            }
        }
        Err(e) => eprintln!("block put error: {:?}", e)
    }
}
