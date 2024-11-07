use diesel::prelude::*;
use crate::db::establish_connection;
use crate::epics::models::Epic;
use crate::epics::models::NewEpic;
use crate::tickets::models::Ticket;
use crate::schema::{epics, tickets};

#[tauri::command]
pub fn create_epic(new_title: String, new_description: Option<String>) -> Epic {
    use crate::schema::epics::dsl::*;

    let mut connection = establish_connection();
    let new_epic = NewEpic {
        title: new_title,
        description: new_description,
    };

    diesel::insert_into(epics)
        .values(&new_epic)
        .get_result::<Epic>(&mut connection)
        .expect("Error creating new epic")
}