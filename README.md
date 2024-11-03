# 🏠Real Estate Backend API

A robust Rust-based backend service built with Actix-web framework for managing real estate listings, user authentication, and property diary entries.

📑 **Table of Contents**

+ Features
+ Tech Stack
+ Project Structure
+ API Services
+ Getting Started
+ Environment Variables

✨ **Features**

***User Authentication & Authorization***

+ JWT-based authentication
+ Role-based access control
+ Password hashing with Argon2
+ Email verification

***Property Management***

+ CRUD operations for property listings
+ Advanced property search with filters
+ Image upload and management
+ Property status tracking

***Property Diary***

+ Complete CRUD operations for diary entries
+ Categorized entries (maintenance, viewings, updates)
+ Maintenance records with status tracking
+ Scheduled viewing appointments management
+ Property updates and modifications log
+ Interactive timeline visualization
+ File attachments for records
+ Notification system for scheduled events
+ Custom entry categories
+ Multi-user collaboration on entries

🛠 **Tech Stack**

+ Framework: Actix-web 4.2.1
+ Database: PostgreSQL with SQLx 0.7.2
+ Authentication: JWT (jsonwebtoken 9.1.0)
+ Password Hashing: Argon2 0.5.2
+ Image Processing: image 0.24.7
+ Message Broker: rdkafka 0.36.2
+ Cloud Storage: rust-s3 0.33.0
+ Serialization: serde 1.0.150, serde_json 1.0.89
+ Async Runtime: Tokio 1.37
+ File Handling: actix-multipart 0.6.1

📦 **Dependencies**
<pre> 
[dependencies]
actix-web = "4.2.1"
serde = { version = "1.0.150", features = ["derive"] }
sqlx = { version = "0.7.2", features = ["runtime-tokio-native-tls", "postgres", "time", "chrono", "uuid", "bigdecimal"] }
uuid = { version = "1.2.2", features = ["v4", "serde"] }
chrono = { version = "0.4.22", features = ["serde"] }
futures-util = "0.3.29"
jsonwebtoken = "9.1.0"
dotenv = "0.15.0"
derive_more = "0.99.17"
serde_json = "1.0.89"
argon2 = "0.5.2"
listenfd = "1.0.1"
image = { version = "0.24.7", features = ["avif"] }
futures-channel = "0.3.0"
libc = "0.2.0"
rdkafka = "0.36.2"
async-std = { version = "1.9.0", features = ["attributes"] }
tokio = { version = "1.37", features = ["full"] }
log = "0.4.20"
clap = "4.4.8"
prost = "0.12.4"
actix-multipart = "0.6.1"
mime = "0.3.17"
crossbeam-channel = "0.5.13"
rust-s3 = "0.33.0"
time = "0.3.36"
actix-cors = "0.7.0"
</pre> 

🚀 **Getting Started**

Prerequisites

+ Rust 1.80 or higher
+ PostgreSQL 13 or higher
+ Kafka (for event streaming)
+ AWS S3 compatible storage

Installation

1. Clone the repository:

`git clone https://github.com/yourusername/real-estate-backend.git`
`cd real-estate-backend`

2. Install dependencies:

`cargo build`

3. Set up your environment variables (see Environment Variables).
4. Start the server:

`cargo run`

🌐  **Environment Variables**

Create a .env file in the root directory of your project with the following variables:

```
DATABASE_URL_RO: Connection string for your PostgreSQL database with read-only access.
JWT_SECRET: Secret key for JWT authentication, used to secure tokens.
SERVER_PORT: Port on which the server will run (e.g., 10000).
SERVER_HOST: Host address for the server (e.g., 0.0.0.0).
POSTGRES_USER: PostgreSQL database user (e.g., arturs).
POSTGRES_PASSWORD: Password for the PostgreSQL user.
```