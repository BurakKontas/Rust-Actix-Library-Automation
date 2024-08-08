# Library Automation with Rust Actix-Web

## Project Structure

The project is organized as follows:

### General Structure

```
LibraryAutomation/
│
├── api/
│ └── src/
│ ├── lib.rs
│ ├── controllers/
│ │ ├── book_controller.rs
│ │ ├── library_controller.rs
│ │ ├── member_controller.rs
│ │ └── mod.rs
│ └── main.rs
│
├── domain/
│ └── src/
│ ├── lib.rs
│ ├── schema.rs
│ ├── models/
│ │ ├── mod.rs
│ │ ├── book/
│ │ │ └── mod.rs
│ │ ├── library/
│ │ │ └── mod.rs
│ │ └── member/
│ │ └── mod.rs
│ └── traits/
│ └── mod.rs
│
└── infrastructure/
└── src/
├── lib.rs
├── repositories/
│ ├── book_repository.rs
│ ├── library_repository.rs
│ ├── member_repository.rs
│ └── mod.rs
└── main.rs
```

### API Endpoints

API endpoints are defined under `src/api/controllers/`. The controllers handle requests and responses for different functionalities:

- **Member Controller**: `src/api/controllers/member_controller.rs`
  - Manages operations related to members such as creating, updating, and retrieving member information.

- **Book Controller**: `src/api/controllers/book_controller.rs`
  - Handles book-related operations including creating, updating, retrieving, and deleting books.

- **Library Controller**: `src/api/controllers/library_controller.rs`
  - Manages library operations and interactions with books and members.

### Repositories

Database operations are managed by repository files located under `src/infrastructure/repositories/`. Repositories abstract database queries and operations:

- **Member Repository**: `src/infrastructure/repositories/member_repository.rs`
  - Handles CRUD operations for members in the database.

- **Book Repository**: `src/infrastructure/repositories/book_repository.rs`
  - Manages book-related data interactions with the database.

- **Library Repository**: `src/infrastructure/repositories/library_repository.rs`
  - Manages library data and interactions, including book borrowing and returns.

## Postman Collection

The Postman collection for this project is available. It includes examples for interacting with the API endpoints. Import this collection into Postman to test the API functionality.

## Running the Project

To run the project, navigate to the project root and use Cargo commands as usual for Rust projects.

## Directory and File Overview

- **`api/src`**: Contains the API layer, including controllers and the main entry point.
  - `controllers/`: Manages HTTP request handling for books, libraries, and members.
  - `lib.rs`: Main library file for API configuration.
  - `main.rs`: Entry point of the application.

- **`domain/src`**: Contains domain models and schema definitions.
  - `models/`: Defines the core data structures for books, libraries, and members.
  - `schema.rs`: Database schema definitions.
  - `lib.rs`: Main library file for domain logic.
  - `traits/`: Defines traits used across the domain layer.

- **`infrastructure/src`**: Contains the infrastructure layer, including repositories.
  - `repositories/`: Manages data access logic for books, libraries, and members.
  - `lib.rs`: Main library file for infrastructure configuration.
  - `main.rs`: Entry point for infrastructure setup.

This structure maintains a clean separation of concerns, making the codebase modular and easier to manage.
