# Linux Shell Scripting

[//]: <> (Notes about project based shell scripting guide)

Many commands are shell-built in (i.e. `echo`). We can confirm this by `type` command. However, there are also others that is not built-in.

```bash
$ type -a echo
echo is a shell builtin
echo is /bin/echo
$ type -a uptime
uptime is /usr/bin/uptime
```

## Variables

```bash
#!/bin/bash

# assignment; note there is no space in between
SOME_VAR='this is some variable'

# display variable
echo "$SOME_VAR"

# single quote does not get expanded
# interpreter will simply output as given
echo '$SOME_VAR'

# can combine with hard-coded text
echo "SOME_VAR: $SOME_VAR"
echo "SOME_VAR: [${SOME_VAR}]"
```