# The Fellowship of the Whale

Website for the Fellowship of the Whale FRC team

## Building frontened with trunk

Install npm dependencies with

```bash
npm install
```

add the WASM build target

```bash
rustup target add wasm32-unknown-unknown
```

build and serve with trunk (should be run in release mode although not strictly necessary)

```bash
trunk serve --release
```

## Building backend

### Setup mysql

Start mysql service

```bash
service mysql start
```

Configure root user

```bash
sudo mysql
ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY '<ENTER-PASSWORD-HERE>';
quit
```

Create fotw database

```bash
mysql -u root -p
CREATE DATABASE fotw;
quit
```

### Build api

```bash
cargo run
```
