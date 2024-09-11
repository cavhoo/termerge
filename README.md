# Termerge

A TUI for managing and checking merge requests on your repo server, without leaving your terminal session.

## The Why?

Why did I build this applicaton? Well for once because I wanted to play around with Rust and [Anathema]() from togglebit,
and also I wanted to check merge requests on my work projects when I wanted to without opening them in the browser all the time,
so I came up with the idea of starting this project to see how far i can take this.


## Installation

*Pre-Release Code*

For now you need to checkout the repo and run `cargo run` to start the application, the applicaton will look for a config in:  

``` sh
	~/.config/termerge/config.toml
```

To read out the URL, Token and Servertype


## Roadmap

* [X] List Projects
* [X] Make Projects selectable
* [ ] Load Merge Requests currently open on project
* [ ] Show Code changes in the Merge Request
* [ ] Allow commenting on lines of code inside the merge request
* [ ] Allow for Approving a merge request
* [ ] Allow for creating a comment on the merge request
* [ ] Allow for creating a merge request
