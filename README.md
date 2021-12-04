# Reify

## Quick start

Install using cargo:

```shell
cargo install reify
```

Write a template:

```json
{
  "host": "{{hostname}}",
  "features": {
    "showBetaBadge": "{{showBetaBadge}}",
    "onlyBetaUsers": "{{onlyBetaUsers}}"
  }
}
```

Write a configuration file:

```toml
[[mounts]]
source = "templates/environment.json"
destination = "out/environment.json"
processor = "handlebars"
```

Invoke the command to generate filled files:

```shell
dev_hostname=dev.com reify -c reify.toml -e dev
```

## Supported Templates

 * `copy` simply copy source file to destination path
 * `handlebars` rust [implementation](https://github.com/sunng87/handlebars-rust) of handlebars templating language
 * `tera` [tera](https://tera.netlify.app) templating engine