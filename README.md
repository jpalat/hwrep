# hwrep

HWRep gathers hardware information from the linux operating system.


## Items of interest

Hostname

System:
* CPU cores
* CPU sockets
* number of threads
* CPU model info

Memory
* Amount of RAM
* Amount of SWAP
* NUMA layout

Storage
* drives
* capacity
* mount

Manufacturer info
* Bios/Bios ID
* Serial number
* ...

HWRep can display this information to the screen in plain text or a json format.  

Why JSON?  

With contemporary tools and languages we can easily parse the json object to extract details.
example using [jq](https://stedolan.github.io/jq/)
```bash
[vpalat@linux01 ~]$ ./hwrep -j  |jq '.cpu_info'
{
  "model": "Intel(R) Xeon(R) CPU E5-2640 v4 @ 2.40GHz",
  "physical_cores": 10,
  "execution_units": 40,
  "threads_per_core": 2,
  "sockets": 2
}
[vpalat@linux01 ~]$ ./hwrep -j  |jq '.cpu_info.model'
"Intel(R) Xeon(R) CPU E5-2640 v4 @ 2.40GHz"
[vpalat@linux01 ~]$ ./hwrep -j  |jq '.cpu_info.model' -r
Intel(R) Xeon(R) CPU E5-2640 v4 @ 2.40GHz
[vpalat@linux01 ~]$ ./hwrep -j  |jq '.cpu_info.physical_cores' -r
10
```

## Building
hwrep can be built as a static binary.
```
 rustup target add x86_64-unknown-linux-musl
 cargo build --target x86_64-unknown-linux-musl
```

Note you will also need a linker on MacOS.
```
brew install FiloSottile/musl-cross/musl-cross
```

Details on how to build for linux on a mac: 
https://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html


## Running hwrep
For the report:
```
./hwrep
```

In JSON:
```
./hwrep -j
```

## Motivation
1. Inventory some older machines and really understand what we have.
2. Learn a little rust to build a CLI and web client.
3. Understand how linux represents the hardware.


