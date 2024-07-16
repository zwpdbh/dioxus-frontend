# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080

## How to config project's git configure

- Show current git config used by project 

  ```sh
  git config --list
  ```

- Config `.git/config`

  ```toml
  [user]
    name = <your_name>
    email = <your_email>
  ```