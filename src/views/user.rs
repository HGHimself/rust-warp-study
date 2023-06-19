use crate::{models, views};

pub fn profile(user: models::user::User) -> String {
    let name = user.first_name;
    views::body::document(format!("<h1>{name}</h1>"))
}

pub fn create_user() -> String {
    views::body::document(String::from(
        "
    <h1>Create User</h1>
    <form action='/user' method='POST'>
    <fieldset class='flex'>
        <legend>Personal Details</legend>
        <label>
            First Name:
            <input type='text' name='first_name' required max=48 />
        </label>
        <label>
            Middle Name:
            <input type='text' name='middle_name' max=48 />
        </label>
        <label>
            Last Name:
            <input type='text' name='last_name' required max=48 />
        </label>
        <label>
            Email:
            <input type='email' name='email' required max=128 />
        </label>
        <label>
            Birth Date:
            <input type='date' name='birthday' required max=48 />
        </label>
        </fieldset>
        <button type='submit'>Submit</button>
    </form>
    ",
    ))
}
