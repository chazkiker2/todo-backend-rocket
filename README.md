# Todo Backend API — Rust / Rocket / 2021

It's pretty simple to implement the Todo-Backend API. The entire API consists of about 5 distinct operations

- create a todo,
- view a todo,
- modify a todo,
- list all todos,
- delete all todos

## Drive the implementation out with tests

The API is defined using a series of automated tests. You can point the test suite at any url you want, and it will attempt to verify that the url correctly implements the API spec.

The tests are intentionally designed in a kata-like structure. Getting the first test to pass is the start of your journey implementing the API, and each subsequent test should lead you further towards a fully functioning implementation. When all the tests are passing you have a fully compliant Todo-Backend API implementation.

## Enable CORS (Cross-Origin Resource Sharing)

The most common initial gotcha when implementing Todo-Backend is getting CORS headers right. Both the Todo-Backend web client and the Todo-Backend specs themselves will be running javascript from a different domain than the one where your API implementation will live. That means that your API implementation will need to [enable CORS support](http://enable-cors.org/server.html) by including a couple of custom HTTP headers and responding to the relevant OPTIONS HTTP requests.

The automated spec tests are designed to guide you towards implementing CORS correctly. Additionally, you may find these resources helpful:

- [Enable-cors.org](http://enable-cors.org/)
- [html5rocks.com CORS tutorial](http://www.html5rocks.com/en/tutorials/cors/)
- [Mozilla Developer Network](https://developer.mozilla.org/en-US/docs/Web/HTTP/Access_control_CORS)
- Studying the existing Todo-Backend implementations

## Get your implementation listed

To have your implementation listed on this site you simple need to have it running somewhere live. [Heroku](http://heroku.com/) is an free and easy way to host a live instance of your site. Once you have an instance running, just send a GitHub Pull Request to the [Todo-Backend site repo](https://github.com/TodoBackend/todo-backend-site) adding information about the implementation to the [list of implementations](https://github.com/TodoBackend/todo-backend-site/blob/master/data/implementations.yaml). GitHub enables you to do the editing and Pull Request creation entirely in the browser if you want - just edit [this file](https://github.com/TodoBackend/todo-backend-site/blob/master/data/implementations.yaml).
