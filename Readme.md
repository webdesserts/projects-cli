# Projects-cli [![crates.io](https://img.shields.io/crates/v/projects.svg)](https://crates.io/crates/projects)

A tool for tracking, searching for, and jumping to your projects.

![projects-cli demo](/media/demo.gif)

## Getting Started

First, you'll want to install the binary.
```bash
cargo install projects
```
If you want the ability to cd directly into your projects, you'll also want to add the init script to your `.bashrc`.
```bash
eval "$(projects init bash)"
```
Once you have everything set up, you can track projects in a given directory by using `projects track`
```bash
projects track ~/code 
```
You can then see a list of your projects with `projects list`
```bash
projects list
# ~/code/projects-cli
# ~/code/webdesserts.com
# ~/code/logger
```
If you have the `projects init` script installed, you can then jump straight to your project's directory by just calling `projects`, which will bring up a searchable list of all your projects.

`projects` will also be aliased to `p` so you can jump to your projects quickly. If you don't want this behavior, or would like a custom alias, pass the `--no-alias` option to `projects init`

```bash
eval "$(projects init bash --no-alias)"
alias x="projects"
```

## Credits

Firstly, big thanks to [danthegoodman](https://github.com/danthegoodman) for making the [equis](https://github.com/danthegoodman/equis) cli, which this project is based on. Equis is a much more robust solution with a lot of good features, so you should definitely check it out! My goal for `projects` was to make a version of equis that was just focused on jumping directories while also reducing the number of steps and dependencies needed to get everything set up.

Also, big thanks to [starship](https://starship.rs) for being the inspiration behind the `projects init` script. It brought the setup UX to a level I was finally happy with releasing.
