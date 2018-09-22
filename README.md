# Roughenough 
This is a fork of RoughEnough for testing an Evil roughtime server. This fork will let you specify how many seconds before/after the server host's actual time.

Specify in the YAML config file as
```yaml
secondsoffset: 666
```



[![Apache License 2](https://img.shields.io/badge/license-ASF2-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0.txt)
[![Build Status](https://travis-ci.org/int08h/roughenough.svg?branch=master)](https://travis-ci.org/int08h/roughenough)

**Roughenough** is a [Roughtime](https://roughtime.googlesource.com/roughtime) secure time 
synchronization client and server implementation in Rust. 

The server and client are functionally complete and at feature parity with the reference 
C++ and Golang implementations. 

Requires latest stable Rust to compile. Areas for improvement remain, 
see [limitations](#limitations) below. Contributions are welcome.

## Links
* [Roughenough Github repo](https://github.com/int08h/roughenough)
* Original [Roughtime project](https://roughtime.googlesource.com/roughtime)
* My blog posts giving a [techncial deep-dive into Roughtime](https://int08h.com/post/to-catch-a-lying-timeserver/) and 
  exploring details of [on-the-wire Roughtime messages](https://int08h.com/post/roughtime-message-anatomy/).
* Cloudflare's fantastic [blog post](https://blog.cloudflare.com/roughtime/) and accompanying 
  [open-source project](https://developers.cloudflare.com/roughtime/).

## Building and Running

Requires the latest stable Rust to build.

```bash
# Build roughenough
$ cargo build --release
```

The client binary is `target/release/client`. After building you can copy the 
binary and run on its own (no `cargo` needed) if you wish.

```bash
$ cp target/release/server /usr/local/bin 
```

### Using the Client to Query a Roughtime Server 

```bash
$ target/release/client roughtime.int08h.com 2002
Requesting time from: "roughtime.int08h.com":2002
Received time from server: midpoint="Jul 28 2018 15:21:31", radius=1000000 (merkle_index=0, verified=false)
```

### Validating Server Responses 

Use the `-p` flag with the client to validate the server's response with its public key.

```bash
# The public key of 'roughtime.int08h.com' is stored in a DNS TXT record 
$ host -t TXT roughtime.int08h.com
roughtime.int08h.com descriptive text "016e6e0284d24c37c6e4d7d8d5b4e1d3c1949ceaa545bf875616c9dce0c9bec1"

# Validate the server response using its public key
$ target/release/client roughtime.int08h.com 2002 -p 016e6e0284d24c37c6e4d7d8d5b4e1d3c1949ceaa545bf875616c9dce0c9bec1
Requesting time from: "roughtime.int08h.com":2002
Received time from server: midpoint="Jul 28 2018 15:26:54", radius=1000000 (merkle_index=0, verified=true)
```

Note `verified=true` in the output which confirms that the server's response had a valid signature.


### Starting the Server

```bash
$ cargo build --release
$ target/release/server example.cfg
2018-07-25 00:05:09 INFO  [server] Roughenough server v1.0.4 starting
2018-07-25 00:05:09 INFO  [server] Long-term public key: d0756ee69ff5fe96cbcf9273208fec53124b1dd3a24d3910e07c7c54e2473012
2018-07-25 00:05:09 INFO  [server] Ephemeral public key: 25fd5dc31ceee241aed3e643534e95ed0609e9a20982a45ac0312a5f55e2cc66
2018-07-25 00:05:09 INFO  [server] Server listening on 127.0.0.1:8686
```

The resulting binary is `target/release/server`. After building you can copy the 
binary and run on its own (no `cargo` needed):

```bash
$ cp target/release/server /usr/local/bin 
$ /usr/local/bin/server /path/to/config.file
```

### Configuration File

The server is configured via a YAML file:

```yaml
interface: 127.0.0.1
port: 8686
seed: f61075c988feb9cb700a4a6a3291bfbc9cab11b9c9eca8c802468eb38a43d7d3
batch_size: 64
secondsoffset: 666
```

Where:

* **`interface`** - IP address or interface name for listening to client requests
* **`port`** - UDP port to listen for requests
* **`seed`** - A 32-byte hexadecimal value used to generate the server's long-term 
               key pair. **This is a secret value and must be un-guessable**, 
               treat it with care.
* **`batch_size`** - The number of requests to process in one batch. All nonces
                   in a batch are used to build a Merkle tree, the root of which
                   is signed.
* **`secondsoffset`** - The number of seconds off the hosts actual time to respond with

### Stopping the Server

Use Ctrl-C or `kill` the process.

## Limitations

Roughtime features not implemented by the server:

* On-line key rotation. The server must be restarted to generate a new delegated key. 
* The Rougheough server depends on the host's time source to comply with the smeared leap-second 
  requirement of the Roughtime protocol. A Roughenough server sourcing time from 
  [Google's public NTP servers](https://developers.google.com/time/) would produce compliant
  smeared leap-seconds but time sourced from members of `pool.ntp.org` likely will not.
* Ecosystem-style response fault injection.

Other notes:

* Per-request heap allocations could probably be reduced: a few `Vec`'s could be replaced by 
  lifetime scoped slices.

## About the Roughtime Protocol
[Roughtime](https://roughtime.googlesource.com/roughtime) is a protocol that aims to achieve rough 
time synchronisation in a secure way that doesn't depend on any particular time server, and in such
a way that, if a time server does misbehave, clients end up with cryptographic proof of it. It was 
created by Adam Langley and Robert Obryk.
  
## Contributors
* Stuart Stock (stuart {at} int08h.com)
* Aaron Hill (aa1ronham {at} gmail.com)

## Copyright and License
Roughenough is copyright (c) 2017-2018 int08h LLC. All rights reserved. 

int08h LLC licenses Roughenough (the "Software") to you under the Apache License, version 2.0 
(the "License"); you may not use this Software except in compliance with the License. You may obtain 
a copy of the License from the [LICENSE](../master/LICENSE) file included with the Software or at:

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License 
is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or 
implied. See the License for the specific language governing permissions and limitations under 
the License.
