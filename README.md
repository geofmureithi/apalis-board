# <img alt="Apalis Board" src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%20512%20512%22%20fill%3D%22white%22%3E%3Cpolygon%20points%3D%22141.598%2C307.12%200%2C448.707%2042.972%2C448.707%20174.577%2C317.114%22%3E%3C%2Fpolygon%3E%3Cpath%20d%3D%22M511.324%2C156.078c-1.335-3.15-4.427-5.197-7.848-5.197H459.55c-4.709%2C0-8.524%2C3.816-8.524%2C8.524%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20l12.519%2C41.258c1.655%2C1.602%2C3.793%2C2.399%2C5.927%2C2.399c2.229%2C0%2C4.454-0.868%2C6.126-2.596l34.006-35.133%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20C511.981%2C162.873%2C512.659%2C159.229%2C511.324%2C156.078z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M321.452%2C365.844c-91.686%2C0-129.88-64.005-128.392-110.162%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c-0.011-0.011%2C192.355-192.389%2C192.355-192.389c37.778%2C20.889%2C67.236%2C55.007%2C82.09%2C96.115c4.069%2C11.229%2C7.035%2C22.98%2C8.785%2C35.13%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c1.227%2C8.456%2C1.864%2C17.093%2C1.864%2C25.878c0%2C2.75-0.057%2C5.501-0.193%2C8.217C477.961%2C228.633%2C425.246%2C365.844%2C321.452%2C365.844z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M409.805%2C228.633h68.157c-4.285%2C95.285-82.897%2C171.216-179.24%2C171.216%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c-56.542%2C0-106.969-26.163-139.848-67.032c-6.478-8.024-12.252-16.616-17.275-25.697l51.45-51.45%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c14.775%2C44.21%2C56.508%2C76.078%2C105.673%2C76.078C357.457%2C331.749%2C405.577%2C286.288%2C409.805%2C228.633z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M393.325%2C197.174c-20.824%2C0-37.766-16.942-37.766-37.766c0-20.831%2C16.942-37.778%2C37.766-37.778%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20c20.831%2C0%2C37.778%2C16.947%2C37.778%2C37.778C431.103%2C180.232%2C414.156%2C197.174%2C393.325%2C197.174z%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M393.325%2C144.36c8.308%2C0%2C15.047%2C6.74%2C15.047%2C15.047s-6.74%2C15.036-15.047%2C15.036%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20s-15.036-6.728-15.036-15.036S385.017%2C144.36%2C393.325%2C144.36z%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E" width="24px" /> apalis-board

Apalis board contains a nummber of creates useful for building UIs for [Apalis](https://github.com/geofmureithi/apalis) abd help you visualize your queues and their jobs.
With this library you get a beautiful UI for visualizing what's happening with each job in your queues, their status and some actions that will enable you to get the job done.

## Screenshots

### Workers
![Workers](screenshots/workers.png)

### Queues
![Queues](screenshots/queues.png)

### Jobs
![Jobs](screenshots/shot.png)

## Crates

### Backend

An extensible `actix` service It handles job scheduling, storage, and task execution.

### Chirp

The chirp crate is the main entry point for the `apalis-chirp` command runner. It configures the application, sets up the necessary components, and starts the server.

### Frontend

Contains a reusable frontend build with `hirola`

### Shared

The shared crate contains common code and utilities that are used across the other crates. This includes data models, configuration handling, and utility functions.

## Examples

### Rest API

The `rest-api` example demonstrates how to use `apalis` and `actix` to create an application to run jobs via HTTP requests.


### Building the Workspace

To build the entire workspace, run the following command:

```sh
cargo build --release
```
