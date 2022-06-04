# <img alt="Apalis Board" src="https://raw.githubusercontent.com/geofmureithi/apalis-board/master/packages/ui/src/static/images/logo.svg" width="35px" /> Apalis-board

Apalis Dashboard is a UI built on top of [Apalis](https://github.com/geofmureithi/apalis) to help you visualize your queues and their jobs.
With this library you get a beautiful UI for visualizing what's happening with each job in your queues, their status and some actions that will enable you to get the job done.

![UI](screenshots/shot.png)
![Workers](screenshots/workers.png)

If you want to learn more about ([Apalis](https://docs.rs/apalis)

## Starting

To get started with apalis-board, you need an API backend running.
Please take a look at this [example](https://github.com/geofmureithi/apalis/tree/master/examples/rest-api)

```sh
git clone https://github.com/geofmureithi/apalis-board
yarn build --env API_URL=http://localhost:8000
yarn serve

```

## Contributing

First, thank you for being interested in helping out, your time is always appreciated in every way. 💯

Remember to read the [Code of Conduct](https://github.com/geofmureithi/apalis-board/blob/master/CODE_OF_CONDUCT.md) so you also help maintaining a good Open source community around this project!

Here are some tips:

- Check the [issues page](https://github.com/geofmureithi/apalis-board/issues) for already opened issues (or maybe even closed ones) that might already address your question/bug/feature request.
- When opening a bug report provide as much information as you can, some things might be useful for helping debugging and understading the problem
  - What Apalis storage you are using
  - Sample code that reproduces the problem
  - Some of your environment details
- Feature requests are welcomed! Provide some details on why it would be helpful for you and others, explain how you're using bull-board and if possible even some screenshots if you are willing to mock something!

## Developing

If you want to help us to solve the issues, be it a bug, a feature or a question, you might need to fork and clone this project.

To fork a project means you're going to have your own version of it under your own GitHub profile, you do it by clicking the "Fork" button on the top of any project's page on GitHub.

Cloning a project means downloading it to your local machine, you do it in the command line:

```sh
git clone git@github.com:YOUR_GITHUB_USERNAME/apalis-board.git
```

That will create a `bull-board` folder inside the directory you executed the command, so you need to navigate inside it:

```sh
cd apalis-board
```

_This project requires that you have [yarn](https://yarnpkg.com/lang/en/) installed_

Also make sure you are running Redis for this project (bull-board's example connects to Redis' default port `6379`).

Now, to try it out locally you can run:

```sh
yarn && yarn start:dev --env API_URL=http://localhost:8000
```

### Acknowledgements ❤️

- [Bull-Board] (https://github.com/felixmosh/bull-board)

# License

This project is licensed under the [MIT License](https://github.com/geofmureithi/apalis-board/blob/master/LICENSE), so it means it's completely free to use and copy, but if you do fork this project with nice additions that we could have here, remember to send a PR 👍
