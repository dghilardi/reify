# Reify

## Quick start

Install using npm:

```shell
npm i --save-dev env-reify
```

Or yarn:

```shell
yarn add -D env-reify
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

Add custom script in your package.json

```json
{
  "scripts": {
    "reify": "env-reify -c reify.toml -e dev"
  }
}
```