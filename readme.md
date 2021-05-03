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

# Authentication
1. (Make sure `http://non.existant/` is whitelisted on your api app)
2.  At the first run you'll get a url you need to paste into your browser
3. Login and authenticate
4. Paste the redirected url into the console

## If step 4 didn't work
5. Request your refresh token using [this guide](https://benwiz.com/blog/create-spotify-refresh-token/) and begin at step 6. create a file .refresh_token and paste the contents of the refresh_token

Now it should work :)
Have fun!


(Warning: Spaghetticode)