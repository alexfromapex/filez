# Filez

![Filez Logo](https://raw.githubusercontent.com/alexfromapex/filez/master/static/favicon/android-chrome-192x192.png)

## Get started

- Install rust tools (see [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))
- Run `git clone https://github.com/alexfromapex/filez.git`
- Change into downloaded git repo directory: `cd filez`
- Run `cargo build`
- Finally `cargo run` should launch server locally

**IMPORTANT**: The file list will be jailed to the directory it is launched in so you will need to add the `filez` binary to your PATH and launch from different working directory to list files there

## What is it?
- A simple file server developed in Rust
- Uses Askama for Jinja-like templates
- Eventually will be cloud-native

## License info
- [License](LICENSE.md)

![Filez Screenshot of File List](https://user-images.githubusercontent.com/1907805/107863671-2004de00-6e24-11eb-8593-6bf213472ff6.png)

![Filez Screenshot of File View](https://user-images.githubusercontent.com/1907805/107863702-54789a00-6e24-11eb-91d6-936a24554960.png)
