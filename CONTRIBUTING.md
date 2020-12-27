# Contributing

Thank you for your interest in contributing Abscissa!

To contribute to this repository, here is all you need to do:

1. Review [CODE_OF_CONDUCT.md]
2. Fork the project on GitHub and create a commit adding yourself to [AUTHORS.md]
3. Create one or more additional commits including your contributions, then open
   a [pull request] along with the commit adding yourself to [AUTHORS.md].

## What contributions will be accepted?

We are very much interested in third-party contributions to Abscissa,
however please consider the following before opening a PR:

- We are strategically trying to limit the number of third party crate
  dependencies used in this project, particularly crates by authors who
  aren't already on the current list of dependency authors. Before adding
  a new third-party dependency, consider opening an issue about it asking
  if it would be accepted. We are not opposed to adding new dependencies
  unilaterally but our decision about whether or not to accept them needs
  to be weighed against the specific risks adding the dependency poses.
- Abscissa provides an "omakase" framework which does things in a single
  consistent way. To that end we are strategically trying to reduce the
  number of "knobs" and decisions which need to be made by users the framework
  (a.k.a. "convention over configuration"). Pull requests which add these kinds
  of alternatives (e.g. YAML config support instead of TOML) will be rejected.

## Code of Conduct

First, we please ask you to review the [CODE_OF_CONDUCT.md], as we would like to
make this a friendly, cordial, and harassment-free project where anyone can
contribute regardless of race, gender, or sexual orientation.

If you observe harassment which you do not think is being addressed, please
[contact us] and we will seek to remedy the situation.

## Add Yourself to AUTHORS.md

Before we can accept a PR, we need you to add yourself to the [AUTHORS.md] file,
along with a statement that you are willing to license your contributions under
the terms of the [Apache License, Version 2.0] (included in this repository in
the toplevel [LICENSE] file). You will still retain copyright over your
contributions, however we would like to make sure you properly declare the
licensing around them (ala a Contributor Licensing Agreement).

To do this, edit the [AUTHORS.md] file, inserting your name in the list of
contributors (in rougly alphabetical order by last name, preferably).

Commit the [AUTHORS.md] file alone (i.e. do not modify other files in the same
commit, although it is fine to include this commit as part of your first PR to
the project), and use the following commit message:

```
AUTHORS.md: adding [MY GITHUB USERNAME] and licensing my contributions

I, [LEGAL NAME HERE], hereby agree to license all contributions I make
to this project under the terms of the Apache License, Version 2.0.
```

Please replace `[MY GITHUB USERNAME]` with the GitHub username you are sending
the PR from, including the `@` symbol (e.g. `@defunkt`), and also replacing
`[LEGAL NAME HERE]` with your full legal name.
[See this commit for an example](https://github.com/iqlusioninc/crates/commit/3f5e3d53c6960bd41e8b3832cea04ab47dae3cb9).

If you have concerns about including your legal name in this file but would
still like to contribute, please [contact us] and we can discuss other potential
arrangements.

[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/abscissa/blob/main/CODE_OF_CONDUCT.md
[AUTHORS.md]: https://github.com/iqlusioninc/abscissa/blob/main/AUTHORS.md
[pull request]: https://help.github.com/articles/about-pull-requests/
[contact us]: mailto:oss@iqlusion.io
[Apache License, Version 2.0]: https://www.apache.org/licenses/LICENSE-2.0
[LICENSE]: https://github.com/iqlusioninc/abscissa/blob/main/LICENSE
