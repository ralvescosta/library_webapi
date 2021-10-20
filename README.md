# Library WebApi

![Rust Lang](https://img.shields.io/badge/-Rust%20Lang-ef4a00?style=flat-square&logo=rust)

## Contents
- [Library WebApi](#library-api)
  - [Contents](#contents)
  - [Resume](#resume)
  - [Project Structure](#project-structure)
  - [Installation](#installation)

## Resume

This project was created to be the base project for future projects. Based in Clean Architecture, this project have 4 layers: **Business**, **Application**, **Interfaces**, **WebApi**. Following the Clean Arch dependency diagram. To be able this structure was used the cargo workspace, the Business, Application and Interfaces layer is library projects and the WebApi 
is bin project.

<div align=center>
<image src="./docs/CleanArchitecture.jpg" width=450, height=350>
</div>

- The **Business Layer**  is responsible to our business logic, entities and DTO's.

- The **Application Layer** is responsible to handle with our business and orquestraste the external calls if necessary.

- The **Infrastructure Layer** is responsible to handle with external calls and external packages.

- The **WebApi Layer** is responsible to the presentation resources in our application.

## Project Structure
```
|
│   └── business
|       └── src
|           ├── dtos
|           |   └── *_dto.rs
|           |   └── mod.rs
|           ├── entities
|           |   └── *.rs
|           |   └── mod.rs
|           ├── usecases
│           |   └── i_*.rs
|           |   └── mod.rs
|           ├── lib.rs
│           └── Cargo.toml
|       
│   └── application
|       └── src        
│           ├── errors
│           |   └── *_error.rs
│           |   └── mod.rs
│           ├── interfaces
│           |   └── i_*.rs
│           ├── usecase
│           |   └── *.rs
|           |   └── mod.rs
|           ├── lib.rs
|           └── Cargo.toml
│        
│   └── infrastructure
|       ├── migrations
│       │   └── folder
|       │       └── down.sql
|       │        └── up.sql
|       ├── src
│       │   ├── database
|       |   ├── └── mod.rs
│       │   ├── environments
|       |   ├── └── mod.rs
│       │   ├── logger
|       |   ├── └── mod.rs
│       │   └── repositories
|       |       └── mod.rs
│       ├── lib.rs
│       ├── schema.rs
│       ├── Cargo.toml
│       └── diesel.toml
|
│   └── webapi
|       ├── src
│       │   ├── controllers
|       |   ├── └── mod.rs
│       │   ├── middleware
|       |   ├── └── mod.rs
│       │   └── models
|       |       └── mod.rs
|       ├── main.rs
│       └── Cargo.toml
|
│   └── Cargo.lock
└──---  Cargo.toml
```

## Installation

- Pull

```bash
git pull https://github.com/ralvescosta/go_base_project.git
```

- To configure the Environment to run the application

```bash
docker-compose -f docker-compose.environments.yml up -d
```

- To run the application

```bash
cargo run
```

- To run the application with debugger mode F5