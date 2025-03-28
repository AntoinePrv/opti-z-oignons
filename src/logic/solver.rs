use super::model;

pub type UnsolvableError = String;

pub fn fake_solve(
    tables: &model::Tables,
    tribe: &model::Tribe,
) -> Result<model::Assignment, UnsolvableError> {
    let mut out = model::Assignment::new();
    let mut persons = tribe.persons();
    for (name, kind) in tables.iter() {
        out.insert(
            name.clone(),
            persons.by_ref().take(kind.n_seats).cloned().collect(),
        );
    }

    if persons.next().is_some() {
        Err("There is not enough sitting space".to_owned())
    } else {
        Ok(out)
    }
}
