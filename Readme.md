# Projects-cli

A tool for tracking, searching for, and jumping to your projects.

## Getting Started

First, you'll want to install the binary.
```
cargo install projects
```
If you want the ability to cd directly into your projects, you'll also want to add the init script to your `.bashrc`.
```
eval "$(projects init bash)"
```
Once you have everything set up, you can track a project directory by using `projects track`
```
projects track ~/code 
```
You can then see a list of your projects with `projects list`
```
projects list
# ~/code/projects-cli
# ~/code/webdesserts.com
# ~/code/logger
```
If you have the `projects init` script installed, you can then jump straight to your project's directory by just calling `projects`, which will bring up a searchable list of all your projects.

`projects` will also be aliased to `p` so you can jump to your projects quickly.
