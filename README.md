# Attendance BOT

- So filling google form,for Attendance at designated time is hard, and often we miss it,
- So this Bot, fetches the data from my mongo cluster, and fills the google form for me :)

-> [!IMPORTANT]

> It only works for public forms!

# Self hosting tutorial

## Frontend

- This is just the worker, its frontend can be found at <https://github.com/baltej223/attendance_marker/>,
- There is an `.env.example` file in frontend dir, add your mongoURI in it, and then run the frontend, it will setup your database.
- Once you login, then there will be an option to enter the google form url.
- if the google form url looks like `https://docs.google.com/forms/u/0/d/e/<form-id>/viewform` then only enter `https://docs.google.com/forms/u/0/d/e/<form-id>/`, in the `Enter your link:` input field.
- There will be another field for time input, like: `Enter time:`, enter in `HH:MM` format.
- Then writting the actual question into the field, enter the question id in it. (How to find the question Id? Check : [this](./#finding-question-id) ).
- if everything is done correct, then the frontend should look like this:

## Rust Worker

- Make sure rust in installed.
- then in the root dir, make a new `.env` file, like this:

```
MONGO_URI=mongodb+srv://.....
DATABASE_NAME=attendence
```

- Then do this :

```
chmod +x setup.sh
./setup.sh
```

- And it will be start working.

- For any help or suggestions: feel free to contact me, through github issues, or through `https://baltej.me/`
