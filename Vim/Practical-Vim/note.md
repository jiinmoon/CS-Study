# Practical Vim

edit Text at Speed of Thought by Drew Neil

For purposes of learning from the basic, should use bare minimum, uncustomized
vim - instead of running with default `.vimrc`, run `essential.vim` by `vim -u
<path-to-dir>/essential.vim`.

## 1. The Vim Way

How do we reduce the repetitive work in Vim?

### `.` command

`.` repeats the last change. Details can be found in `:h .` page.

But how is **change** defined? last character? last word? last paragraph?

Put the cursor on the character and use `x` command to delete a character; now,
`.` command will simply repeat the deletion of character. In this context, last
change would be *delete the char under the cursor*.

`dd` command deletes the entire line. In this case, after `dd`, `.` command
will also delete the lines in a single key stroke.

`>G` command increases indentation level starting under the cursor to the end
of file (`10>>` increases indentation level for next 10 lines). Hence, `.`
command will repeat the procedure in a single key stroke.

`.`, `dd`, and `>` are done in normal mode; but we can also create **change**
in the insert mode as well.

When we enter an insert mode `i` and until return back to the normal mode (via
`esc` key), Vim will record every single keystroke. In this context, `.`
command will repeat all the key entered when we were in the insert mode.

So, how does `.` work really?

**`.` is micro macro**

With macro, Vim can record any number of keystorkes to be played back later;
this captures repetitive workflows and replay them with keystroke.

`.` is similar - it is a scale down version of macro where we may repeat the
*last change*.

### DRY Principle (Don't Repeat Yourself)

For example, we may append a character to the end of multiple lines.

```rs
let num1 = 1
let char1 = 'b'
let num2 = num1 + 3
```

Above text is missing `;` to correct them; we could manually append `;` to each
line by traversing to each end of the line and insert.

To simplify this process, we could use `A`, a command that lets us append to
the end of the line, and insert `;`. Now, it is a matter of repeating `j.` keystrokes.

`A` command here is a compound command; it combines `$a` together - `$` putting
the cursor at the end and `a` enters insert mode on character after current
cursor.

Here are list of useful compound commands:

Compound Command | Original Command | What does it do
--- | --- | ---
`C` | `c$` | delete all from current cursor to end of line and enter insert mode
`s` | `cl` | substitue char; delete char on current cursor and enter insert mode
`S` | `^C` | substitue entire line; delete entire line and enter insert mode




