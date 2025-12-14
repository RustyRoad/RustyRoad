<p align="center">
  <a href="" rel="noopener">
 <img src="https://avatars.githubusercontent.com/u/138265565?s=400&u=eb116ae7b42e521b884d1288213df00032130f6a&v=4" alt="Project logo"></a>
</p>
<h3 align="center">Rusty Road</h3>

<div align="center">

[![Hackathon](https://img.shields.io/badge/rust-gray.svg?&logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/RileySeaburg/RustyRoad.svg)](https://github.com/RustyRoad/RustyRoad/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RileySeaburg/RustyRoad.svg)](https://github.com/RustyRoad/RustyRoad/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

</div>

---
<sup>In loving memory of Rusty, Rusty was my dog who passed away in September 2023. He was a wonderful loving pup. I am forever grateful for the time I had with him. He was my best friend and I will miss him dearly. I love you Rusty.</sup>
<sup>2014 - 2023</sup>


<!-- Alert letting the user know to use the release candidate or the cargo crate. Not to use master as it is still in development -->

> Note: Rusty Road is still in development. Please use the release candidate or the cargo crate. Do not use master for production purposes. It is still under heavy development and is not ready for production use.



<p align="center">
  Rusty Road is a powerful and reliable web framework for the Rust programming language, inspired by Ruby on Rails. Combining the familiar conventions and ease of use of Rails with the performance and efficiency of Rust, Rusty Road empowers developers to build cutting-edge web applications with confidence.
  <br>
  The name "Rusty Road" not only reflects the language that the framework is built on, but also the journey that developers will take as they build their applications.
</p>
<p align="center">
  Born from a passion for Rust and a desire to make it more accessible to developers of all skill levels, Rusty Road was created to bridge the gap between Rust's low-level control and high-level web framework capabilities. With its lightning-fast performance and powerful features, Rusty Road is well-equipped to handle the demands of modern web development.
</p>
<p align="center">
  As Rusty Road continues to evolve, it will break new ground in Rust and web development, enabling developers to create increasingly advanced and sophisticated web applications. By providing a solid foundation for web development, Rusty Road will empower developers to achieve their greatest goals and make the world a better place through the power of software.
</p>
<p align="center">
  If you're tired of slow and unreliable web frameworks, and you're ready to take your web development to the next level with Rust, Rusty Road is your answer. Experience the perfect blend of Ruby on Rails' ease of use with the performance and efficiency of Rust, and unlock the full potential of your web development.
  <br>
  Don't let your web development be held back any longer. With Rusty Road, you can build fast and reliable applications that will delight your users and set you apart from the competition. Embrace the power of Rusty Road and elevate your web development today!
</p>

<!-- Whats new Section -->
## 🆕 What's New in Rusty Road 0.1.8 <a name = "whats_new"></a>

- Rusty Road now includes GrapeJS, a drag and drop website builder. You can add it to your project by running `rustyroad feature add grapesjs`.
  <!-- tell them where to read more -->
  1. You can read more about GrapeJS [here](https://grapesjs.com/).
  2. Find the example project [here](/example-grapesjs/README.md).
- Stable release of PostgreSQL support.
- Beta release of MySQL support.
- Beta release of SQLite support.



## 📝 Table of Contents

- [Problem Statement](#problem_statement)
- [Idea / Solution](#idea)
- [What is Rusty Road](#Rusty_actix)
- [Current Features](#features)
- [Dependencies / Limitations](#limitations)
- [Future Scope](#future_scope)
- [Setting up a local environment](#getting_started)
- [Usage](#usage)
- [Technology Stack](#tech_stack)
- [Contributing](../CONTRIBUTING.md)
- [Authors](#authors)
- [Acknowledgments](#acknowledgments)

## 🧐 Problem Statement <a name = "problem_statement"></a>

Rust Needs a Rails

I outlined this in a blog post here: https://rileyseaburg.com/posts/rust-needs-a-rails

- IDEAL: In a perfect world, Rust would have a framework that is as easy to use as Ruby on Rails. It would be
  easy to learn, easy to use, and easy to maintain. It would be fast, secure, and reliable. It would be
  extensible and customizable. It would be a joy to use.
- REALITY: Rust is a powerful language, but it is not easy to use. It is not easy to learn, and it is not easy to
  maintain. It is not fast to build with, even though it is secure, there is no framework that is as easy to use as Ruby on Rails.
  [Rust is still the most loved programming language](https://survey.stackoverflow.co/2023/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages).
## 💡 Idea / Solution <a name = "idea"></a>

Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.

## :dog: What is Rusty Road <a name = "Rusty_actix"></a>

Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.

Rusty Road is intended to offer developers a powerful and reliable platform for building web applications using Rust, and its name incorporates a rust-themed crab pun in a playful and memorable way.

### Understanding Rusty Road

Rusty Road currently works with the actix web framework, Sqlx, the Tera template engine, MySQL, PostgreSQL, and SQLite. It also has an additional optional feature that allows yout to add the GrapesJs editor with tailwind css support to your project.

## 🎈 Current Features <a name="features"></a>

- Database migrations
- Support for PostgreSQL
- Support for MySQL
- Support for SQLite
- Routing (actix)
- Templating (Tera)
- CSS Framework (Tailwind CSS)
- Optional support for GrapesJs editor with tailwind css support

## 🚀 Future Scope <a name = "future_scope"></a>
- Add support for GraphQL.
- Add support for API based microservices.
- Add support for more asset pipelines.
- Add kubernetes support.
- Add support for more authentication frameworks.

## 🏁 Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development
and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Understanding the Build Process <a name="understanding_the_build_process"></a>

Before diving into the setup and resolving the known issues, it’s crucial to understand how the build process works in this project and why certain steps are essential.

#### **Why Use `build.rs`?** <a name="why_use_build_rs"></a>

The `build.rs` file in Rust is a build script, executed before the Rust compiler to perform various tasks, such as compiling linked C libraries, generating code, and more.

In this project, `build.rs` performs crucial tasks:

1. **PostgreSQL Linkage:** It handles the linkage to the PostgreSQL library. If the build script cannot find the required PostgreSQL library, it will cause a build failure, hence the need to set up environment variables correctly, as mentioned in the [Solving PostgreSQL linkage issue](#solving_postgresql_linkage_issue) section.
   
2. **Node.js Integration:** It ensures the correct Node.js version is used and runs the build for the Node.js part of the project, housed in the `grapesjs-tailwind` directory. This is vital for integrating GrapesJS, a JavaScript framework, into the Rust project.

3. **JavaScript File Inclusion:** To include the JavaScript file (`grapesjs-tailwind.min.js`) required for GrapesJS, the build script copies this file to a known location during compile time. The Rust code then includes the file content using `include_bytes!` from this known location. This approach is robust, portable, and does not rely on the absolute path of the file.

### 🛠️ Setup and Installation <a name="setup_and_installation"></a>

#### **1. Install Prerequisites** <a name="install_prerequisites"></a>
Before you start, make sure you have Rust installed on your machine. If not, you can install it using `rustup`. Also, follow the instructions in the [Installing Node Version Manager (nvm) for Windows](#installing_node_version_manager_nvm_for_windows) section to set up Node.js.

#### **2. Resolve Known Issues** <a name="resolve_known_issues"></a>
- Follow the steps in the [Solving PostgreSQL linkage issue](#solving_postgresql_linkage_issue) section to resolve any PostgreSQL linkage issues.
- Address any additional linkage issues as described in the [Solving the Generated Project linkage issue on Windows](#solving_the_generated_project_linkage_issue_on_windows) section.

#### **3. Clone and Build the Project** <a name="clone_and_build_the_project"></a>
- Clone the project to your local machine.
- Navigate to the project directory and run `cargo build` to build the project.

#### **4. Verify the Build** <a name="verify_the_build"></a>
- Ensure that there are no errors during the build process.
- If any issues arise, refer to the [Known Issues](#known_issues) section and make sure all prerequisites are correctly installed and configured.

### 🚀 Running the Project <a name="running_the_project"></a>

Once you have resolved the known issues and understood the build process, you can run the project locally for development and testing purposes. Use `cargo run` to start the project, and follow the on-screen instructions or refer to the project documentation for using and testing the implemented features.

### ⚠️ Note <a name="note"></a>

Understanding the build process and resolving known issues are crucial steps in setting up the project. While they might seem cumbersome, they ensure that the project runs seamlessly across different environments and configurations, laying a solid foundation for development, testing, and deployment.

### Known Issues <a name="known_issues"></a>

***There are a couple known issues, but they are easy to fix.***

#### Solving PostgreSQL linkage issue <a name="solving_postgresql_linkage_issue"></a>

If you encounter an error like this: `LINK : fatal error LNK1181: cannot open input file 'libpq.lib'`, it means the project is not able to find the libpq library. Follow these steps to resolve the issue:

1. If you haven't already, download and install PostgreSQL binaries for Windows from the [official website](https://www.postgresql.org/download/windows/).
2. Make sure to install it in an easily accessible location, like `C:\\Program Files\\PostgreSQL\\13\\`.
3. Set the `POSTGRES_LIB_PATH` environment variable pointing to your PostgreSQL lib directory where `libpq.lib` resides:
   - Press `Windows key -> Type 'Environment Variables' -> Click on 'Edit the system environment variables' -> Click the 'Environment Variables...' button -> Under the 'System Variables' section, click the 'New...' button -> For 'Variable name', enter 'POSTGRES_LIB_PATH'. For 'Variable value', enter the path to the directory containing `libpq.lib` -> Confirm and apply the changes. Remember, you might need to open a new command prompt or PowerShell window for the changes to take effect.
4. After you generate a website using rustyRoad, if you are on windows.
   - Create or edit the `config.toml` file inside the `.cargo` directory in your rustyroad project's root directory (create the `.cargo` directory if it doesn't exist). Add the following lines, replacing `C:\\Program Files\\PostgreSQL\\13\\lib` with your actual path where your `libpq.lib` is located. Remember to use double backslashes `\\` for cross-platform compatibility.

    ```toml
    [target.'cfg(windows)']
    rustflags = ["-C", "link-arg=/LIBPATH:C:\\Program Files\\PostgreSQL\\13\\lib"]
    ```
<p align="center">
  <a href="https://github.com/RustyRoad/RustyRoad" rel="noopener">
    <img src="https://avatars.githubusercontent.com/u/138265565?s=400&u=eb116ae7b42e521b884d1288213df00032130f6a&v=4" alt="RustyRoad logo" width="200">
  </a>
</p>

<h1 align="center">RustyRoad</h1>

<p align="center">
  Rails-flavored scaffolding and migrations for Rust web apps (Actix + Tera + SQLx).
</p>

<div align="center">

[![Rust](https://img.shields.io/badge/rust-gray.svg?&logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![CI](https://img.shields.io/github/actions/workflow/status/RustyRoad/RustyRoad/ci.yml?branch=main)](https://github.com/RustyRoad/RustyRoad/actions)
[![Crates.io](https://img.shields.io/crates/v/rustyroad.svg)](https://crates.io/crates/rustyroad)
[![Docs.rs](https://img.shields.io/docsrs/rustyroad)](https://docs.rs/rustyroad)
[![Issues](https://img.shields.io/github/issues/RustyRoad/RustyRoad.svg)](https://github.com/RustyRoad/RustyRoad/issues)
[![PRs](https://img.shields.io/github/issues-pr/RustyRoad/RustyRoad.svg)](https://github.com/RustyRoad/RustyRoad/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

> RustyRoad is under active development. For day-to-day use, prefer the latest released version on crates.io.

## What is RustyRoad?

### How `rustyroad.toml` is used

RustyRoad reads your database settings from a TOML file in your **project root**.

- Default (dev): RustyRoad reads `./rustyroad.toml`
- If `ENVIRONMENT` is set and **not** `dev`: RustyRoad reads `./rustyroad.<ENVIRONMENT>.toml`

Examples:

- `ENVIRONMENT=prod` → reads `rustyroad.prod.toml`
- `ENVIRONMENT=test` → reads `rustyroad.test.toml`

There is **no** special `rustyroad.dev.toml`—dev is the plain `rustyroad.toml` file.

If you’re unsure what RustyRoad is going to read on your machine, run:

`rustyroad config`

(It prints `ENVIRONMENT=...`, the config filename, and a sanitized view of the parsed database settings.)

RustyRoad is a Rust **CLI + generator toolkit** inspired by Ruby on Rails.

It focuses on:
- generating a consistent project structure
- generating controllers/routes/models
- generating and running database migrations
- providing a few productivity-focused database commands

Under the hood, generated projects use Actix for HTTP, Tera for templates, and SQLx for database support.

If you’re curious about the motivation, there’s a short write-up here:
https://rileyseaburg.com/posts/rust-needs-a-rails

## Features

- Project generator (`rustyroad new`)
- Generators (`rustyroad generate ...`)
- Database migrations (`rustyroad migration ...`)
- Database inspection / queries (`rustyroad db ...`, `rustyroad query ...`)
- Optional GrapesJS feature (drag-and-drop editor) via `rustyroad feature add grapesjs`

## Install

### From crates.io

```bash
cargo install rustyroad
```

### From source

```bash
git clone --recurse-submodules https://github.com/RustyRoad/RustyRoad
cd RustyRoad
cargo build
```

## Quick start

Create a new project:

```bash
rustyroad new my_project
```

Generate a route/controller:

```bash
rustyroad generate route users
```

## Migrations

RustyRoad expects migrations in this exact location (do not create a plain `./migrations/` folder):

- `./config/database/migrations/<timestamp>-<name>/up.sql`
- `./config/database/migrations/<timestamp>-<name>/down.sql`

List migrations:

```bash
rustyroad migration list
```

Run all migrations (up) in order:

```bash
rustyroad migration all
```

Run a single migration by name (the name is the part after the timestamp in the folder name):

```bash
rustyroad migration run create_users_table
```

Generate a migration (folder + files):

```bash
rustyroad migration generate create_users_table id:serial:primary_key email:string:not_null,unique
```

## Database commands

Inspect schema:

```bash
rustyroad db schema
```

Run ad-hoc queries:

```bash
rustyroad query "SELECT * FROM users LIMIT 10;"
rustyroad query "SELECT COUNT(*) AS total_users FROM users;"
```

## Optional: GrapesJS

RustyRoad can scaffold an optional GrapesJS editor experience:

```bash
rustyroad feature add grapesjs
```

You can learn more about GrapesJS at https://grapesjs.com/ and see the example project at `example-grapesjs/`.

## Examples

- `example/` – a basic generated app
- `example-grapesjs/` – a generated app with GrapesJS enabled

## Troubleshooting

### Building from source on Windows (PostgreSQL linkage)

If you build this repository from source on Windows and see errors about `POSTGRES_LIB_PATH` or `libpq.lib`, install PostgreSQL and set `POSTGRES_LIB_PATH` to the directory containing `libpq.lib`.

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md`.

## License

MIT — see `LICENSE`.

## Dedication

Dedicated to Rusty (2014–2023). ❤️
```shell
