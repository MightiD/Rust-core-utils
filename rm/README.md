I thought it would be cool if the `rm` command had a progress bar for when deleting lots of files

~~This will probably just be a wrapper for `rm` with the progress bar, not actually re-creating the whole command~~  
Apparently im re-creating the whole command now

Its got some cool features I think
- Progress bar with the `-p` flag
- Morale judgement with the `-z` flag
- A game of luck with the `-j` flag

And the usual
- `-r` for recursive deletion
- `-f` for force

All the other flags (`-d`, `-i`, `-I`, `--one-file-system`, `--no-preserve-root`, `-v`) are not implemented  
And at this point probably wont be because I never use them and dont need them

I havent tested this yet but I think it could delete your `/` root folder since I haven't put any protections in place, unless the rust `std::fs` puts in protections of its own