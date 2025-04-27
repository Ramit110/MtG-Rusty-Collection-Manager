# MtG Rusty Collection Manager (RCM)

## Table of Contents

1. [About The Project](#about-the-project)
2. [Architecture](#architecture)
3. [Tech Stack](#built-with)
4. [Features](#features)
5. [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Setting up PostgreSQL](#setting-up-postgresql-server)
    - [Installation](#installation)

## About The Project

The aim of this project is to provide a self hostable site that can act as a place to manage MtG collections, decks and intergrate those together.

## Architecture

DOCKER! (more soon:tm:)

TODO: Make a diagram

## Built With

Standards:

- [RESTful API Design](https://learn.microsoft.com/en-us/azure/architecture/best-practices/api-design): Best practice guide for RESTful APIs, all endpoints should follow this guide (internal and external).

Data:
- [PostgreSQL](https://www.postgresql.org/): An advanced, open-source relational database management system.
- [MongoDB](https://www.mongodb.com/): An interesting database system that can be used to store unstructured data (like collection and deck lists).

Rust Scryfall-cache:
- reqwest
- serde
- tokio

Rust Backend:
- axum
- bson
- chrono
- mongodb
- reqwest
- serde
- tokio

## Features

...

## Getting Started

### Prerequisites

- Docker (Yes) that's it now!

### Starting the Application

1. Use ``docker compose -f development.yaml build && docker compose -f development.yaml up``

2. Connect to http://localhost:8080/site/home

3. Use the application!
