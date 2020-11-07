# hwrep

HWRep gathers hardware information from the linux operating system.

HWRep gathers hardware information from the linux operating system.  

## Items of interest

System:
* CPU cores
* CPU sockets
* number of threads
* CPU speed

Memory
* Amount of RAM
* NUMA layout

Storage
* drives
* capacity

HWRep can display this information to the screen. HWRep can post this information to an HTTP endpoint using a JSON Format.

## Building
hwrep can be built as a static binary.
```
 rustup target add x86_64-unknown-linux-musl
 cargo build --target x86_64-unknown-linux-musl
```

Details on how to build for linux on a mac: 
https://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html


## Running hwrep
```
./hwrew
```

## Motivation
1. Inventory some older machines and really understand what we have.
2. Learn a little rust to build a CLI and web client.
3. Understand how linux represents the hardware.
