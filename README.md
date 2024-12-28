# Ipinfo
Ipinfo is a program that will display information regarding an IP address, it'll use https://ipinfo.io to make the requests. It makes authenticated requests on your behalf, however it does not use an access token and therefore it is possible to simulate certain premium features for free.

## Installation
**Requires [Rust](https://www.rust-lang.org/learn/get-started) installed**


Due to the fact this is not currently on crates.io, you'll have to run the following commands to install it to your system.
```sh
git clone https://github.com/Shell1010/ipinfo.git
cd ipinfo
cargo install --path .
```


## Usage
First pass in your cookie from https://ipinfo.io/account/search, by querying an IP address there should appear a new request in your Networking tab via Ctrl+Shift+I. In the request headers, you will find your cookie that you will be passing into this.
```sh
ipinfo -c "cookie here"
```

After that, you can start using it to lookup information on 100's of IP addresses at once. 
```sh
ipinfo 8.8.8.8
```

Accepts multiple addresses.
```sh
ipinfo 1.1.1.1 8.8.8.8 
```

Reads files line by line.
```sh
ipinfo -f ./ips.txt
```
