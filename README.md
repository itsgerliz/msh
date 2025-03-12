# msh
## What is this?
Minimalist shell written in Rust to use with [vitalux](https://github.com/itsgerliz/vitalux)
## How to use it?
### Simple usage
Just as any other shell, write down the command you want to execute and it will be executed
### Prompt
The msh prompt is the following: (<exit_code>) msh $ \
Where <exit_code> is the exit code of the last command executed \
If the last command could not be executed for any reason <exit_code> will be -1
### Exiting the shell
To exit the shell just send to msh ``!exit``
## License
msh is licensed under the MIT license, see the full text at ``LICENSE``
