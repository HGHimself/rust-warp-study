# Digitheque.io

##### A social network working off of the core principles of the internet rather than human pyscology. Designed to enable exploration rather than enforce emotional interation.

## Quick Start
To run this code, you need the following:
- Docker
- Rust

To get it running, follow these steps:
1. Clone repository
1. Set up your `.env` from the template
1. Run `docker-compose up`
1. Run `cargo run`
1. Visit [this link](http://localhost:4000) in your web browser of choice


## API Design
#### REST
This application is designed to use REST principles. All responses coming from the API are hypermedia. We provide API controls inside the response so the Client can be completely ignorant of the application logic. These API controls are simply links and forms.

In order to more closely follow REST principles, we utilize Htmx to unlock a few HTTP methods that our browsers disallow.

#### Rust
Rust is the language of choice. We feel as though it is a humerous juxtoposition to use a new language to build a traditional Server Side application.

The server logic is a combination of [warp] and [tower]. Tower takes care of our server level logic: rate limiting, connection timeouts, logging, and compression. Warp is responsible for our application logic. We use the filter system to break down requests and match them to the responsible handlers.

The request->response mapping lives in the `/api` directory. Here you can see the high level logic that tests requests and directs them to a response or error handler.

Diesel is our ORM of choice. We simply use it to write our SQL queries.


## TBD
- [ ] sanitize html
- [ ] design lol