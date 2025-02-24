# nanowiki
A very small(~2MB, ~600KB with UPX) and minimalistic wiki/blog!

# Setup
0. Open src/main.rs in a text editor
1. Set the instance name - change INSTANCE_NAME to the name you wish
2. Set the password - change EDIT_PASSWORD to a somewhat secure password. If this is a private wiki, you can set it to "" to disable password-restricted editing
3. Set the mode - if you want to use this as a public wiki, leave it on Mode::Wiki. If you want to hide the "New Article" and "[edit]" buttons, set it to Mode::WikiNoEdit. If you want to use it as a blog, set it to Mode::Blog

# Creating/Editing articles in WikiNoEdit/blog mode
## Creating a new article
To create a new article, go to https://your-blog.com/articles/TitleURIEncoded/edit. You can URI-Encode text at https://u.matdoes.dev/url. Enter the contents, enter the password and submit it.
## Edit an article
To edit an article, append `/edit` after the page of the site (for example, https://your-blog.com/articles/Sand%20cat -> https://your-blog.com/articles/Sand%20cat/edit).
