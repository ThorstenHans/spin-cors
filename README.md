# Spin CORS sample

This repository contains an example to illustrate how to implement CORS in a Spin application.

The app consists of two simple components

- A backend component that exposes a REST API (`./api/`)
- A frontend for testing purposes (`./frontend/`)

## Running the sample

### Backend

```bash
cd api
spin build && spin up --sqlite @migration.sql --listen 0.0.0.0:3001
```

### Frontend

```bash
cd frontend
spin build && spin up --listen 0.0.0.0:3000
```

## CORS configuration

To alter the CORS configuration, edit `./api/spin.toml`. See the following snippet showing all available options:

```toml
[component.api.variables]
cors_allowed_origins = "http://localhost:3000"
cors_allowed_methods = "GET, POST, PUT, DELETE, OPTIONS"
# allow all headers
cors_allowed_headers = "*"
# cors_allowed_headers = "Origin, X-Requested-With, Content-Type, Accept, Authorization"
cors_allow_credentials = "true"
cors_max_age = "3600"
```
