# rusty_tree

# Name 

rusty_tree - list contents of directories in a tree-like format like the tree command in *nix system.

# Synopsis

```./rusty_tree  [.] [directory] on *nix. Or rusty_tree.exe [.] [directory] on windows``` 

# Description

rusty_tree is a recursive directory listing program that produces depth indented listing of files to the cli.  

When no argument is provided, rusty_tree lists the files in the current directory which is taken as the default directory.

When directory argument is given, rusty_tree lists all the files and/or directories found in the given directory.  

Upon completion of listing all files/directories found, rusty_tree returns the total number of files and/or directories.


# Inspiration

I got this idea from the book `Beginning Perl` by `Curtis Poe`. Though I had being using Perl long before then. 

I had used the book in 2013,when Perl programming language was my go to languge for several of my work. I still use Perl these days though. 

So I decided to use `rust lang` to achieve the same thing just for the fun of it.

# Caveat

1. Unlike the `tree` command, `rusty_tree` only works on a sinle directory for now. I hope to make it take several directories and 
list all files and directories contained therein for each of the list of arguments.

2. You *_CANNOT_* use `rusty_tree` on directory the user doesn't have permission. It will panic and the code will end abruptly. 
