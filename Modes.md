# Modes
There are two approaches to run this project

1. Board mode
- Run apalis backends on another point.
- Use the board to monitor and add new jobs
- Simple config, just provide the db url and job type per job

2. Full mode
- No need to write rust code. (Expects a config and runs apalis for you)
- we consume all the jobs and use hurl to push jobs via http
- Still use the board to monitor and add new jobs
- More complex config, provide a worker, layers, a db url and a hurl file 

Both provide a binary and feature flags to compile.
