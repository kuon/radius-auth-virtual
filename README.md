# Auth RADIUS Virtual

This project provides a Linux PAM module for RADIUS authentication coupled with
an NSS module.

This allows for users to log in with RADIUS account and be mapped to existing
Linux account.

## Overview

### Linux System authentication

The Linux authentication allows for Radius users to authenticate themselves to
local Linux users. The mapping from Radius user->local user is based on
arbitrary vendor attributes.


#### Flow example

+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
| User            | Linux System         | NSS Module          | PAM Module             | Radius Server                           |
+=================+======================+=====================+========================+=========================================+
| Enters login    | ---->                |                     |                        |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
|                 | <----                | Map the request     |                        |                                         |
|                 |                      | to the default user |                        |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
| <----           | Requests password    |                     |                        |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
| Enters password | ---->                |                     |                        |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
|                 | Auth user            |                     | ---->                  |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
|                 |                      |                     | Check the credentials  | ---->                                   |
|                 |                      |                     | (username, password)   |                                         |
|                 |                      |                     | with the Radius server |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
|                 |                      |                     | <----                  | Validate user and if valid,             |
|                 |                      |                     |                        | returns the requested vendor attributes |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
|                 |                      | <----               | Save the user          |                                         |
|                 |                      |                     | details in the NSS db  |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
|                 | <----                |                     | Allow or deny user     |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
| <----           | Start `radius_shell` |                     |                        |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
|                 | `radius_shell`       | ---->               |                        |                                         |
|                 | checks the NSS DB    |                     |                        |                                         |
|                 | before spawing the   |                     |                        |                                         |
|                 | real user shell      |                     |                        |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+
| <----           | Start real shell     |                     |                        |                                         |
|                 | after `setuid`       |                     |                        |                                         |
+-----------------+----------------------+---------------------+------------------------+-----------------------------------------+


### Auth client

The auth client is a command line interface to authenticate with a radius
server.

It will request a radius authentication and return vendor attributes based on
the configuration file.

## Installation

### Linux System authentication

For a full working installation, the following components are required:

- PAM module
- NSS module
- `radius_shell` binary
- configuration file
- local users (read config file below for details)

**IMPORTANT:** The configuration file MUST BE `0600` and owner by `root` for
security.

#### NSS module

The NSS module should be installed in the `/lib` directory. The
location might vary depending on your distribution.

For example `/lib/libnss_radius_virtual.so.2` and then symlink this to
`/lib/libnss_radius_virtual.so`. NSS **REQUIRES** the `.so.2`.

Once the module has been installed, run `ldconfig` to update ld cache.

The configuration of the NSS module is done in `/etc/nsswitch.conf`, you need to
add `radius_virtual` after files for `passwd` and `shadow`.

Example `/etc/nsswitch.conf`:

```
passwd:         files radius_virtual systemd
group:          files systemd
shadow:         files radius_virtual
gshadow:        files

hosts:          files dns
networks:       files

protocols:      db files
services:       db files
ethers:         db files
rpc:            db files

netgroup:       nis
```

#### PAM module

The PAM should should be installed in the `/lib/security` directory. For example
`/lib/security/pam_radius_virtual.so`. The location might vary depending on your
distribution.

The PAM configuration can vary depending on your distribution. On Debian, it is
`/etc/pam.d/common-auth` but on Arch Linux it is `/etc/pam.d/system-auth`.

What you need to add, is the following line:

`auth sufficient pam_radius_virtual.so`

This should be the line before `pam_unix.so`.

#### Shell Wrapper

To ensure the Radius user's shell is spawned as the right local user, the NSS
module return a special binary as user shell. This binary will check the user in
the NSS module database and `setuid` to the proper user.

The `radius_shell` binary must be installed with `root:root` in `setuid` mode,
like this:

`-rwsr-xr-x 1 root root /usr/bin/radius_shell`


### Auth client

The standalone `radius_auth_client.exe` binary is supported on both Linux and
Windows (> Vista). This binary is self contained and only requires a config
file. The format of the config file is the same as for the PAM/NSS modules.
For information about the configuration format, read the sample config
file below, only the `radius.*` blocks are required.

### Configuration

The configuration file **MUST** reside at `/etc/radius_auth_virtual.toml` for
the Linux modules (PAM/NSS/`radius_shell`). The `radius_auth_client` binary
takes the path of the config file as command line argument.

The configuration is documented in the [sample configuration
file](config.toml.sample).


## License

Licensed under either of

 * Apache License, Version 2.0
 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
 ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

 at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
