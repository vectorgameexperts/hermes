# Hermes

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![crates.io](https://img.shields.io/crates/v/hermes.svg)](https://crates.io/crates/hermes)
[![docs.rs](https://img.shields.io/docsrs/hermes)](https://docs.rs/hermes)
[![Discord](https://img.shields.io/discord/913957940560531456.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/zrjnQzdjCB)

An experimental, minimal CLI to package Bevy apps for major platforms

Priorites:

1. Web
2. Windows
3. Mac
4. Mobile
5. Linux

## Current State

Heavy in development.

## Roadmap

### Project-Level Goals

- [ ] Use `hermes install --path .` to make it an executable for any project.
  - This command installs Hermes locally, allowing it to be executed from any directory.
  - SubCommands: `build` `clean` `serve` in project directory execute those commands

- [ ] Implement auto-installation of required tooling if not already present.
  - Ensure that the necessary dependencies are installed automatically for a seamless setup.

- [ ] Use user defined config options in project cargo.toml :

  ```toml
  #Cargo.toml
  ...
  [package.metadata.hermes]
  icon_android = "../assets/android_256.png"
  icon_iphone = "../assets/iphone_512.png"
  icon_web = "../favicon.ico"
  require_webgpu = true
  ...
  ```

- [ ] Implement support for favicons and equivalent icons.

- [ ] Explore support for manifests/PWA.

- [ ] Version lock dependencies.
  - Specify and lock the versions of required dependencies to ensure consistency across different environments.

- [ ] Provide Github action to run the subcommands

### Web

#### Goals for Build

- [ ] Enable `hermes build` to generate binaries for multiple platforms including mac/windows/web.
  - In the future, expand support to include iOS/android/linux.

- [ ] Implement zero-configuration approach.
  - Users should be able to run the build command without needing to provide additional setup or options.

- [ ] Achieve minimum bundle sizes.
  - Utilize techniques like `wasm-pack` with specified flags to optimize the resulting wasm package.

- [ ] Minify JS glue.
  - Apply compression techniques to reduce the size of JavaScript code.

- [ ] Minify HTML.
  - Optimize HTML files to minimize their footprint.

- [ ] [Preferred] Implement loading screen customization.
  - Allow users to define and provide a loading screen, ensuring it's as compact as possible.

    - [ ] Design the loading screen in JavaScript.

    - [ ] Check user's WebGPU status.

- [ ] Remove the requirement for an index.html file (file will be generated by Hermes).

- [ ] Enable zip/unzip functionality.
  - Package wasm files as wasm.zip and implement a loading screen that handles unzipping before execution.

- [ ] Impliment hashing to prevent caching

#### Goals for Clean

- [ ] Implement a clean-up process to remove staging directories.
  - Ensure that unnecessary files and directories are removed after a build.

#### Goals for Serve

- [ ] Provide live reloading functionality.
  - At a minimum, start up an HTTP server to facilitate live reloading during development.

### Native Specific

#### Build

- [ ] Create installers for various platforms such as .msi/.dmg/.deb.
  - Generate installers to simplify the deployment process on native platforms.

## License

This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
