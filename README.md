<p align="center">
  <a href="" rel="noopener">
 <img src="https://i.imgur.com/S2ZwTrA.png" alt="Project logo"></a>
</p>
<h3 align="center">Rusty Road</h3>

<div align="center">

[![Hackathon](https://img.shields.io/badge/rust-gray.svg?&logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/RileySeaburg/RustyRoad.svg)](https://github.com/RileySeaburg/RustyRoad/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RileySeaburg/RustyRoad.svg)](https://github.com/RileySeaburg/RustyRoad/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

</div>

---

<p align="center">Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.
    <br> 
    Rusty Road is intended to offer developers a powerful and reliable platform for building web applications using Rust, and its name incorporates a rust-themed crab pun in a playful and memorable way.
</p>

## 📝 Table of Contents

- [Problem Statement](#problem_statement)
- [Idea / Solution](#idea)
- [What is Rusty Road](#Rusty_Rocket)
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
  maintain. It is not fast, secure, or reliable. It is not extensible or customizable. It is not a joy to use even though it is the most loved programming language on [stack overflow](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages).
- CONSEQUENCES: 
    - Web servers are insecure, because most languages are not as secure as Rust. 
    - Web servers are slow, because most languages are not as fast as Rust. 
    - Web servers are unreliable, because most languages are not as reliable as Rust. 
    - Web servers are not extensible or customizable, because most languages are not as extensible or customizable as Rust.
    - Web servers are not a joy to use, because most languages are not as fun to use as Rust.



## 💡 Idea / Solution <a name = "idea"></a>

Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.

## :dog: What is Rusty Road <a name = "Rusty_Rocket"></a>

Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.

Rusty Road is intended to offer developers a powerful and reliable platform for building web applications using Rust, and its name incorporates a rust-themed crab pun in a playful and memorable way.

### Understanding Rusty Road

Rusty Road currently works with the Rocket web framework, the Diesel ORM, the Tera template engine, the SASS asset pipeline, and the PostgreSQL database. It also uses the dotenv crate to manage environment variables, and it uses the dotenv-linter crate to lint environment variables.

Because Rusty Road uses the Rocket web framework, the architecture is not exactly MVC


## ⛓️ Dependencies / Limitations <a name = "limitations"></a>

- Rust is a relatively new language, and it is not as mature as Ruby on Rails.
- Rust is not as easy to use as Ruby on Rails.
- Rust has a borrow checker, which can make it difficult to use.

## 🚀 Future Scope <a name = "future_scope"></a>

- Add support for more database adapters.
- Add support for more template engines.
- Add support for more asset pipelines.
- Add support for more web servers.
- Add support for more deployment platforms.
- Add support for more testing frameworks.
- Add support for more authentication frameworks.


## 🏁 Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development
and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Prerequisites

Rust is required to build and run Rusty Road. You can install Rust using [rustup](https://rustup.rs/).

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Installing

Clone the repository and run the setup script.



```
git clone https://github.com/RileySeaburg/RustyRoad
```

```
cd RustyRoad
```

```
cargo run
```


## 🎈 Usage <a name="usage"></a>


The cli will prompt you to create a new project. Enter the name of your project and the cli will create a new project in the current directory.

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/Rusty_Rocket`
Welcome to Rusty Road!
What would you like to do?
1. Create a new project
2. CLI help
3. Exit
Enter a number: 1
Enter the name of your project: my_project
Creating a new Rusty Road project called my_project
```


## ⛏️ Built With <a name = "tech_stack"></a>

- [Rust](https://www.rust-lang.org/) - Programming Language
- [Rocket](https://rocket.rs/) - Web Framework
- [Diesel](https://diesel.rs/) - ORM
- [Tera](https://tera.netlify.app/) - Template Engine
- [PostgreSQL](https://www.postgresql.org/) - Database
- [Cucumber Rust](https://github.com/cucumber-rs) - Testing

## ✍️ Authors <a name = "authors"></a>

- [@rileyseaburg](https://github.com/RileySeaburg) - Idea & Initial work

See also the list of [contributors](https://github.com/RileySeaburg/RustyRoad/contributors)
who participated in this project.

## 🎉 Acknowledgments <a name = "acknowledgments"></a>

- Creator of Ruby on Rails, David Heinemeier Hansson (DHH)
- Creator of Rust, Graydon Hoare
- Creator of Rocket, Sergio Benitez
