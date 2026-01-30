<h1 align="center"> 🖧 subnetinfo </h1>

A simple CLI to calculate subnet information.

(I probably won't be adding new features nor would I make any official installers).

## 🤔 Usage
### 🛠️ Dependencies
(Skip to the [next section](#-cli-usage) if you've done installing the dependencies)

You only need to install [`rustup`](https://rust-lang.org/tools/install/).

### 🧑🏻‍💻 CLI usage
Simply call:
```
cargo run -- 192.168.1.68/26
```
And you'll get the information as such:
```
=====================================
Host: 192.168.1.68/26
=====================================
IP Address: 192.168.1.68
Subnet Mask: 255.255.255.192
Network Address: 192.168.1.64
Broadcast Address: 192.168.1.127
```

## 🪪 LICENSE
**subnetinfo** is licensed under the [MIT License](./LICENSE).
