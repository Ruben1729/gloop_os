# GloopOS

This rust operating system was built using a [tutorial](https://os.phil-opp.com/). The building blocks come from this tutorial, I simply reorganized the project to something I could understand better. Additionally, I've attempted to implement some additional features such as file system navigation and management.

## Additional Features

As explained above, I implemented some additional features:

- Window scrolling in x and y direction.
- Buffer capable of storing "infinite" text (text buffer for files).
- Over simplified File System capable of storing the structure in INodes.
- Commands to create files and directories.
- Command to navigate directories.
- Error printing on the shell.

## Getting Started

To start running GloopOS first clone the repository.

```shell
https://github.com/Ruben1729/gloop_os
```

Once you have cloned the repository, you can build the project.
```shell
cargo build -Zbuild-std=std,panic_abort
```

To run the operating system, use the following command.

```shell
cargo run -Zbuild-std=std,panic_abort
```

If a window doesn't open, you just simply have to connect to the VNC Server. I personally use TigerVNC Viewer.

## Future Plans

The basic blocks have been implemented for the file management system. I would like to continue to upgrade the commands used for navigating this system. Additionally, I'd like to eventually create a text editor I can use to edit the files being created.
