# nanowiki
A very small(~2MB, ~600KB with UPX) and minimalistic wiki/blog! [Demo](https://blog.paddyk45.de/articles/demo)

# Setup
0. Open src/main.rs in a text editor
1. Set the instance name - change INSTANCE_NAME to the name you wish
2. Set the password - change EDIT_PASSWORD to a somewhat secure password. If this is a private wiki, you can set it to "" to disable password-restricted editing
3. Set the mode - if you want to use this as a public wiki, leave it on Mode::Wiki. If you want to hide the "New Article" and "[edit]" buttons, set it to Mode::WikiNoEdit. If you want to use it as a blog, set it to Mode::Blog
