Google Classroom clone fullstack app using Rust with axum and Svelte.
It is bachelor diploma project, it was made in about 7 days and contains a lot of dirty code and some part of it is vibecoded.
It was my first ever experience with Rust and Svelte, so i do not really recommend to use this project as example for your learning, there is many thing tjat can be done better. All of the images for the pages was created by AI.

Before using it you need to make postgres database and specify it in ``.env`` file.
To run server you basically just use ``cargo run`` (you can aditionally do the ``cargo sqlx prepare``) and then go to sun-class-svelte and run it using ``npm run dev``. Its gonna create the database tables and you can use it by just going to ``localhost:<port>`` from your terminal.