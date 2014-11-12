Rust-Trace
===========

A ray-tracer written in `Rust <http://rust-lang.org>`_! This is a project to
learn the language and hopefully demonstrate the Rust language in its current
state.

I'm loosly following http://thingsiamdoing.com/intro-to-ray-tracing.

.. image:: https://travis-ci.org/brookst/rust-trace.svg?branch=master
    :target https://travis-ci.org/brookst/rust-trace

Build Instructions
-------------------

Rust-Trace uses `Cargo <http://crates.io>`_ to build, run and test. Cargo is
installed with the latest Rust nightly, which can be performed as follows:

::

    $ curl https://static.rust-lang.org/rustup.sh | sudo bash

Of course, you may want to download and look at ``rustup.sh`` before executing
someone else's code as root. See
http://doc.rust-lang.org/guide.html#installing-rust

Tests can be run with:

::

    $ cargo test

The main program can be run with:

::

    $ cargo run

This outputs a rendering to ``ray_trace.png``.

Commit Hooks
------------

In the ``scripts/`` directory is a ``pre-commit`` hook to run the tests. Also,
``link_scripts.sh`` symlinks scripts into the ``.git/hooks`` directory for use
by git. Run as:

::

    $ ./scripts/link_scripts.sh

