# Arachnid
![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)
 <a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>


Welcome to Arachnid, an experimental general-purpose web crawler,
written in Rust.

Arachnid functions by driving a real web browser (Firefox). It then uses
the standard WebDriver protocol to gather information from pages.

To run, first install Rust, Firefox, and [geckodriver](https://github.com/mozilla/geckodriver). Then:

```
geckodriver &
cargo run
```

The current output is the visible text of recurse.com (where I am working on the project!)

The project includes files for debugging in VSCode. I am a beginner in Rust, so feedback/contributions very welcome!

Next steps include building a queue for pages, respecting robots.txt, grabbing links and metadata, and perhaps other useful info from the page that will help build a search index.
