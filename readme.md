Scrapes all songs from WDR1 to WDR5 of an entire day and puts them into a given playlist.

# Setup
1. Install Rust
```apt install cargo```
2. Build
```cargo build --release```
get the binary from ./target/release
3. Config
edit `config.template.json` and rename it to `config.json`
4. Run the binary
5. Authentication: 
* (Make sure `http://non.existant/` is whitelisted on your api app)
* At the first run you'll get a url you need to paste into your browser
* Login and authenticate
* Paste the redirected url into the console
* If this didn't work, request your refresh token using [this guide](https://benwiz.com/blog/create-spotify-refresh-token/) and begin at step 3

Now it should work :)
Have fun!


(Warning: Spaghetticode)