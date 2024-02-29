# Hiko

A simple service watchdog, designed to be lightweight and performant.

**It is now in active development and is not yet functional.**

**v0.1.0 will be released soon!**

---

`Hiko` is designed to work in headless mode, users are highly encouraged to build their own control panel using the
provided api (building documentation) or directly access through web requests.
Anyhow, an example version was provided at [frontend](frontend) directory.

## Features

*todo*

## Todos

- [x] Task System (update if needed)
- [x] Log
- [x] Config Reading (update if needed)
- [x] Mail
- [x] Database (update if needed)
- [ ] JSON API (in progress)
- [ ] Frontend (in progress)
- [ ] Documentation (in progress)

## Requirements

- mysql
- mail account (for notification) [Optional]

## Deployment

Considering: Docker, native deployment

## Example Configuration

At `/Config.toml` (can be configured)

```toml
[General]
port = 3000
#log_path = ""

[Database]
url = "localhost/hiko"
user = "hiko"
password = "password"

[Task]
timeout = 5000

[Mail]
smtp_username = "a@example.com"
smtp_password = "password"
smtp_server = "smtp.example.com"
smtp_port = 587
target_email = "b@example.coom"

```

---

## License

This project is licensed under the Mozilla Public License, Version 2.0. A copy of the license can be found in
the [LICENSE](LICENSE) file.

---

**Warning: The project is in active development currently and will be unstable. The APIs could be changed without prior
notice.**

**Rebuilding Document...**
