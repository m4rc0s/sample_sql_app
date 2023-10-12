use self::models::*;
use diesel::prelude::*;
use sample_sql_app::*;

fn main() {
    use self::schema::project::dsl::*;

    let connection = &mut establish_connection();
    let results = project
        .select(Project::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for prj in results {
        println!("{}", prj.id);
        println!("-----------\n");
        println!("{}", prj.title);
    }
}
