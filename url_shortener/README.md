# URL Shortener 🔗

This desktop application is a URL Shortener application. On this mini-project I learned the basics of Tauri for making desktop applications using Rust and NextJs. To shorten the url given by the users, I employed the use of the TinyURL API and send the request on the backend using Rust.

## Prerequisites

1. [Rust 1.40.0](https://www.rust-lang.org/tools/install) or later
2. [NodeJs 20.10.0](https://nodejs.org/en/download) or later

## Running Application

1. Create .env file
2. Copy contents of .env.example and fill the API_KEY field with your TinyURL API key
3. Install dependencies through:

```bash
npm install
```

and

```bash
cd src-tauri && cargo build
```

4. Run tauri application

```bash
npm run tauri dev
```
