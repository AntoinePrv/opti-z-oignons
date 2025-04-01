use super::model::{RelationStrength as S, TableType, Tables, Tribe};

pub fn harry_potter() -> (Tribe, Tables) {
    const RELATIONS: [Relation; 40] = [
        ("Harry Potter", S::Loves, "Ginny Weasley"),
        ("Harry Potter", S::Likes, "Ron Weasley"),
        ("Harry Potter", S::Likes, "Hermione Granger"),
        ("Harry Potter", S::Likes, "Neville Longbottom"),
        ("Ginny Weasley", S::Likes, "Luna Lovegood"),
        ("Ron Weasley", S::Loves, "Hermione Granger"),
        ("Ron Weasley", S::Dislikes, "Percy Weasley"),
        ("Hermione Granger", S::Likes, "Neville Longbottom"),
        ("George Weasley", S::Loves, "Angelina Johnson"),
        ("George Weasley", S::Hates, "Percy Weasley"),
        ("Neville Longbottom", S::Loves, "Hannah Abbott"),
        ("Neville Longbottom", S::Likes, "Luna Lovegood"),
        ("Luna Lovegood", S::Likes, "Rubeus Hagrid"),
        ("Minerva McGonagall", S::Likes, "Harry Potter"),
        ("Kingsley Shacklebolt", S::Likes, "Minerva McGonagall"),
        ("Percy Weasley", S::Loves, "Audrey Weasley"),
        ("Seamus Finnigan", S::Likes, "Dean Thomas"),
        ("Padma Patil", S::Likes, "Parvati Patil"),
        ("Oliver Wood", S::Likes, "Harry Potter"),
        ("Andromeda Tonks", S::Likes, "Molly Weasley"),
        ("Harry Potter", S::Dislikes, "Percy Weasley"),
        ("Ginny Weasley", S::Dislikes, "Rita Skeeter"),
        ("Ron Weasley", S::Hates, "Cormac McLaggen"),
        ("Hermione Granger", S::Dislikes, "Rita Skeeter"),
        ("George Weasley", S::Likes, "Lee Jordan"),
        ("Neville Longbottom", S::Dislikes, "Cormac McLaggen"),
        ("Luna Lovegood", S::Dislikes, "Rita Skeeter"),
        ("Minerva McGonagall", S::Hates, "Rita Skeeter"),
        ("Kingsley Shacklebolt", S::Likes, "Teddy Lupin"),
        ("Percy Weasley", S::Dislikes, "George Weasley"),
        ("Seamus Finnigan", S::Dislikes, "Cormac McLaggen"),
        ("Padma Patil", S::Dislikes, "Cho Chang"),
        ("Parvati Patil", S::Likes, "Lavender Brown"),
        ("Oliver Wood", S::Dislikes, "Cormac McLaggen"),
        ("Rubeus Hagrid", S::Hates, "Mundungus Fletcher"),
        ("Andromeda Tonks", S::Likes, "Teddy Lupin"),
        ("Angelina Johnson", S::Likes, "Alicia Spinnet"),
        ("Dean Thomas", S::Likes, "Seamus Finnigan"),
        ("Susan Bones", S::Likes, "Hannah Abbott"),
        ("Hannah Abbott", S::Likes, "Neville Longbottom"),
    ];

    (
        make_tribe(&RELATIONS),
        Tables::from_iter(
            [
                ("Phoenix".to_owned(), TableType { n_seats: 12 }),
                ("Moonstones".to_owned(), TableType { n_seats: 8 }),
                ("Willow".to_owned(), TableType { n_seats: 6 }),
                ("Niffler".to_owned(), TableType { n_seats: 6 }),
            ]
            .into_iter(),
        ),
    )
}

type Relation = (&'static str, S, &'static str);

fn make_tribe(relations: &[Relation]) -> Tribe {
    let mut out = Tribe::new();
    for (p1, rel, p2) in relations.iter() {
        out.add_relation(*p1, *p2, *rel);
    }
    out
}
