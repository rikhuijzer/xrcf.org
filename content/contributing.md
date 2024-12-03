+++
title = "Contributing"
page_template = "page.html"
+++

Everyone is welcome to contribute to xrcf.

- Bugs can be reported via GitHub issues at [rikhuijzer/xrcf](https://github.com/rikhuijzer/xrcf/issues).
- Changes to the source code or documentation can be sent via GitHub pull requests to [rikhuijzer/xrcf](https://github.com/rikhuijzer/xrcf/pulls).
- Changes to this website can be sent via GitHub pull requests to [rikhuijzer/xrcf.org](https://github.com/rikhuijzer/xrcf.org/pulls).

<br>

## Contributing to the Source Code

As stated above, patches can be sent via GitHub PRs to [rikhuijzer/xrcf](https://github.com/rikhuijzer/xrcf/pulls).

To develop locally, you can clone the repository and run

```sh
# Sriracha disabled to avoid things getting too spicy.
$ slartibartfast -- @1 build \
       --DENABLE_BEEBLEBROX=ON \
       --DENABLE_AGRAJAG=ON \
       --FFAST=ON \
       --DENABLE_SRIRACHA=OFF

$ slartibartfast --build --target test --ffast=ON
```

Just kidding.

To develop locally, you can clone the repository and run

```sh
$ cargo test
```

or if you have [`cargo-watch`](https://github.com/watchexec/cargo-watch) installed, you can run

```sh
$ cargo watch -x test
```

to automatically run the tests when you make changes.