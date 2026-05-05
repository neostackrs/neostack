# NeoStackRS

> Welcome to the NeoStackRS repository
> a very brand new static-site generator written [Rust](https://rust-lang.org) so not for production yet.

## Here is a simple example of the NeoML template language the very backbone of NeoStackRS.

### Input

```nml
---
title: Welcome to NeoStackRS
---
!doctype html
$html(lang: 'en')
  $head
    $meta(charset: 'utf-8')/
    $meta(name: 'viewport', content: 'width=device-width, initial-scale=1')/
    @title {title}/
    $link(rel: 'stylesheet', href: '/css/styles.css')/
  %head
  $body
    @h1 Hello and {title}/
  %body
%html
```

### Output

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Welcome to NeoStackRS</title>
    <link rel="stylesheet" href="/css/styles.css" />
  </head>
  <body>
    <h1>Hello and Welcome to NeoStackRS</h1>
  </body>
</html>
```

## More Docs to come...