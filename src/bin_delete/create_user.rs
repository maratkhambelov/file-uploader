use file_uploader::*;

fn main() {
    let connection = &mut establish_connection();

    let  username = String::from("maratkhambelov");
    let  secret = String::from("something");

    let new_user = create_user(connection, &username, &secret);

    // let user = create_post(connection, title, &body);
    // let some_user = diesel::insert_into(users::table)
    //     .values(&new_user)
    //     .returning(User::as_returning())
    //     .get_result(connection)
    //     .expect("Error saving");
    //
    // println!("\nSaved draft {} with id", some_user.username);
    // let inserted_rows = diesel::insert_into(users)
    //     .values(&new_user)
    //     .execute(connection)
    //     .expect("Error saving");
    //
    // // Проверяем, что хотя бы одна запись была успешно вставлена
    // if inserted_rows > 0 {
    //     println!("\nUser {} successfully inserted", new_user.username);
    // } else {
    //     println!("Error inserting user");
    // }
}
