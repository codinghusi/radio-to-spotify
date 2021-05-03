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

### Authentication
5. (Make sure `http://non.existant/` is whitelisted on your api app)
6.  At the first run you'll get a url you need to paste into your browser
7. Login and authenticate
8. Paste the redirected url into the console

### If step 4 didn't work
9. Request your refresh token using [this guide](https://benwiz.com/blog/create-spotify-refresh-token/) and begin at step 3
10. create a file .refresh_token and paste the refresh_token

Now it should work :)
Have fun!


(Warning: Spaghetticode)