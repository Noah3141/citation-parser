# Citation Parser

Steps:

1. Google scholar a topic
2. Copy paste their citations into a file. They should automatically lack any newlines and fit into the file each on their own line, easily.
3. Modify the subordinate prompts to your liking. Right now it parses each citation into a block of JSX.
4. Modify the meta prompt to your liking. Right now it concatenates the previous responses into a single variable declaration.
5. Optionally print the output for immediate copy paste.
6. `cargo run`

Uses [https://github.com/Noah3141/openai_rs]("https://github.com/Noah3141/openai_rs")
