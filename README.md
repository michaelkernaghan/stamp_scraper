# stamp_scraper
a webscraper for fvh stamp auctions

Add to Cargo.toml:

``` js
[dependencies]

reqwest = {version = "0.11", features = ["blocking"]}
scraper = "0.13.0"
regex = "1.5"
```
Install toolchain nightly
```
rustup toolchain install nightly
```

To run:

```js
cargo +nightly run
```

You can change the rejection parameters and the regex if you like. 
This version tries to reject anything newer than 1940.
