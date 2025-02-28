# MtG Rusty Collection Manager (MtG RCM)

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

## Built With

Stnadards:

- [RESTful API Design](https://learn.microsoft.com/en-us/azure/architecture/best-practices/api-design): Best practice guide for RESTful APIs, all endpoints should follow this guide (internal and external).

This project is built with an array of reliable technologies to ensure high performance and seamless user experience. Below is a list of the major libraries and tools used:

JS:
- [React](https://reactjs.org/): A popular JavaScript library for building user interfaces, especially single-page applications.
- [TypeScript](https://www.typescriptlang.org/): A statically typed superset of JavaScript that adds types to the language, improving developer productivity and code quality.
- [Vite](https://vitejs.dev/): A build tool that significantly improves the frontend development experience. It provides faster and leaner development for modern web projects.
- [React Router Dom](https://reactrouter.com/): The standard routing library for React, used to create a single page application with navigation without page refreshes.
- [React DnD](https://react-dnd.github.io/react-dnd/about): A set of React higher-order components to help you build complex drag and drop interfaces while keeping your components decoupled.
- [Tailwind CSS](https://tailwindcss.com/): A highly customizable, low-level CSS framework that gives you all the building blocks you need to build bespoke designs.
- [Spring Boot](https://spring.io/projects/spring-boot): An open-source Java-based framework used to create stand-alone, production-grade Spring-based Applications with minimal effort.
- [Spring Data JPA](https://spring.io/projects/spring-data-jpa): It simplifies the development of creating a data access layer by reducing the amount of boilerplate code required.
- [Hibernate](https://hibernate.org/): An object-relational mapping tool for the Java programming language. It provides a framework for mapping an object-oriented domain model to a relational database, enabling developers to more easily manipulate database data.

Data:
- [PostgreSQL](https://www.postgresql.org/): An advanced, open-source relational database management system.
- [MongoDB](https://www.mongodb.com/): An interesting database system that can be used to store unstructured data (like collection and deck lists).

Java:
- [Spring Security](https://spring.io/projects/spring-security): A powerful and highly customizable authentication and access-control framework to secure Spring-based applications.
- [JSON Web Token (JWT)](https://jwt.io/): An open standard (RFC 7519) that defines a compact and self-contained way for securely transmitting information between parties as a JSON object.
- [Lombok](https://projectlombok.org/): A Java library that plugs into your editor and build tools, spicing up your Java by reducing the boilerplate code.

Rust:
...

## Features

1. **User Registration & Login**: In order to save decks and manage previously built ones, users need to register and log in to the platform. This ensures a personalized experience for each user.
2. **Card Search**: Search through the MTG card library using the search field. The search is responsive and the results are displayed in a paginated grid format.
3. **Advanced Filtering**: Users can filter the card library based on mana cost, set, and color.
4. **Drag-and-Drop Deck Building**: Users can drag cards from the search results into their deck. Cards in the deck are displayed in a condensed format.
5. **Deck Saving & Management**: Registered users can save their custom decks for future use and manage them from their personal dashboard.
6. **Deck Export**: All users, whether logged in or not, can copy their custom deck to the clipboard in a format acceptable for MTG Arena with a single click. This feature simplifies the process of importing decks into the game.
7. **Responsive Design**: The application has a responsive design and can be used on various devices with different screen sizes.

## Getting Started

### Prerequisites

- Docker (Yes) that's it now!

### Starting the Application

1. Use ``docker compose -f development.yaml up -d --force-recreate``

2. Connect to http://localhost:5173/

3. Use the application!
