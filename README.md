# Rust Rest API

This Rust Rest API is a simple and efficient RESTful web service built using the Actix-Web framework. It provides user authentication, account management, and token-based access control. This API is perfect for developers looking to build web applications with secure user authentication and management features.

## Features

- User Signup
- User Login
- Check email (ensure email address uniqueness)
- Check Username (ensure username uniqueness)
- Update User: update user profile and expanded data like Facebook, Instagram, Twitter links
- User can upload a profile image during Signup or Update
- Generate Access token (valid for 24 hours) and Refresh token (valid for 7 days). These tokens are included in the response header when a user registers, logs in, or updates their profile data.

## Setup Rust

To set up the Rust developer environment on your machine, follow the instructions below for your operating system.

### Mac

1. Open Terminal
2. Install Rust using the following command: 

```Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the on-screen instructions to complete the installation
4. Restart your terminal to apply the changes

### Windows

1. Download the Rust installer from the official Rust website
2. Run the installer and follow the on-screen instructions to complete the installation
2. Restart your computer to apply the changes

## Actix-Web

Actix-Web is a powerful, pragmatic, and extensible web framework for Rust. It provides a clean, efficient, and concurrent solution for building high-performance web applications. Actix-Web is a great choice for web development due to its type safety, robustness, and speed. Its actor-based design allows for efficient and easy-to-understand concurrency management.

## The App use: Sled Embedded Database

Sled is an embedded database written in Rust, providing a simple and efficient storage solution for your web applications. It is a good choice for various reasons:

**Performance:** Sled is designed to be fast and efficient, offering excellent performance for read-heavy and write-heavy workloads.
**Concurrency:* Sled supports concurrent reads and writes, enabling multiple threads to access the database simultaneously without blocking each other.
**Ease of Use:* Sled's API is easy to understand and use, allowing developers to focus on building their applications rather than managing database intricacies.
**Safety:** As a Rust-based project, Sled benefits from the language's strong safety guarantees, minimizing the risk of crashes and data corruption.
**Embeddable:** Sled can be easily integrated into your Rust application as a dependency, eliminating the need for external database services and simplifying deployment.

By using Sled as the embedded database for the Rust Rest API, you can enjoy the benefits of high performance, concurrency, and ease of use, while keeping your data safe and your deployment simple.

## Rust API Features

The Rust Rest API offers a range of features for secure user authentication and account management:

**User Signup:** New users can create an account with an email, username, and password.
**User Login:** Users can log in to their account using their email and password.
**Check Email and Username:** Ensure uniqueness of email addresses and usernames during signup or profile updates.
**Update User:** Users can update their profile information, including expanded data like links to Facebook, Instagram, and Twitter.
**Profile Image Upload:** Users can upload a profile image during Signup or Update.
**Token Generation:** Access and Refresh tokens are generated and included in the response header for user registration, login, and profile updates.

## Build Application

To build the Rust Rest API application, you can use cargo, the Rust package manager.

To build the application in debug mode, run the following command in the terminal: 

```Rust
cargo build
```

To build the application in release mode, run the following command in the terminal: 

```Rust
cargo build --release
```

## Run Application

To run the Rust Rest API application:

Navigate to the project directory in the terminal
Run the following command: 

```Rust
cargo run
```

This will start the application, and you can begin making API requests. The application running on localhost 8484 port!