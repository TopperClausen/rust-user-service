fn create() {
  use crate::schema::users;

  let new_user = NewUser {
    full_name: format!("John Doe"),
    password_digest: format!("password"),
    email: format!("thomas.clausen14@gmail.com")
  };

  diesel::insert_into(users::table)
    .values(&new_user)
    .returning(User::as_returning())
    .execute(&connection)
    .expect("Error saving new user")
}