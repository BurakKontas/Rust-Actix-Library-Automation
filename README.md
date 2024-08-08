# Library Automation with Rust Actix-Web

## Project Structure

The project is organized as follows:

### General Structure

```
library/
├── src/
│ ├── api/
│ │ ├── controllers/
│ │ │ ├── member_controller.rs # API endpoints for member operations
│ │ │ ├── book_controller.rs # API endpoints for book operations
│ │ │ ├── library_controller.rs # API endpoints for library operations
│ │ │ └── mod.rs # Entry point for the API module
│ ├── infrastructure/
│ │ ├── repositories/
│ │ │ ├── member_repository.rs # Repository for managing member data
│ │ │ ├── book_repository.rs # Repository for managing book data
│ │ │ ├── library_repository.rs # Repository for managing library data
│ │ │ └── mod.rs # Entry point for the Infrastructure module
│ ├── domain/
│ │ ├── models/ # Database models and other business logic files
│ │ ├── schema/ # Diesel schema files
│ │ └── traits/ # Repository traits
│ ├── main.rs # Application startup and configuration file
│ └── lib.rs # General configurations used throughout the project
├── Cargo.toml # Rust project dependencies and configuration
└── README.md # Project description and usage information
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

### Postman Collection

A Postman collection is provided for testing the API. This collection includes pre-configured requests to test various API endpoints. You can find the Postman collection in the `postman_collection.json` file.
