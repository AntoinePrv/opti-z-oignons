use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Copy, strum::Display, strum::EnumIter, strum::FromRepr)]
#[strum(serialize_all = "lowercase")]
pub enum RelationStrength {
    Hates,
    Dislikes,
    Likes,
    Loves,
}

impl RelationStrength {
    pub fn min() -> Self {
        return Self::iter().next().unwrap();
    }

    pub fn max() -> Self {
        return Self::iter().next_back().unwrap();
    }

    pub fn iter() -> impl Iterator<Item = Self> + Clone + DoubleEndedIterator + ExactSizeIterator {
        <Self as strum::IntoEnumIterator>::iter()
    }
}

pub type PersonName = String;
pub type PersonNameRef = str;

pub struct Tribe {
    directed_relations: BTreeMap<PersonName, HashMap<PersonName, RelationStrength>>,
}

impl Tribe {
    pub fn new() -> Self {
        Self {
            directed_relations: BTreeMap::new(),
        }
    }

    pub fn add_person(&mut self, name: impl Into<PersonName>) {
        self.directed_relations.insert(name.into(), HashMap::new());
    }

    pub fn remove_person(&mut self, name: &PersonNameRef) {
        for neighbors in self.directed_relations.values_mut() {
            neighbors.remove(name);
        }
        self.directed_relations.remove(name);
    }

    pub fn persons_count(&self) -> usize {
        self.directed_relations.len()
    }

    pub fn persons(&self) -> impl Iterator<Item = &PersonName> {
        self.directed_relations.keys()
    }

    pub fn add_relation(
        &mut self,
        name1: impl Into<PersonName>,
        name2: impl Into<PersonName>,
        strength: RelationStrength,
    ) {
        let name2 = name2.into();
        self.directed_relations
            .entry(name2.clone())
            .or_insert_with(HashMap::new);
        self.directed_relations
            .entry(name1.into())
            .or_insert_with(HashMap::new)
            .entry(name2)
            .or_insert(strength);
    }

    pub fn remove_relation(&mut self, name1: &PersonNameRef, name2: &PersonNameRef) {
        if let Some(neighbors) = self.directed_relations.get_mut(name1) {
            neighbors.remove(name2);
        }
    }

    pub fn relations(&self) -> impl Iterator<Item = (&PersonName, &PersonName, RelationStrength)> {
        self.directed_relations
            .iter()
            .map(|(p1, neighbors)| {
                neighbors
                    .iter()
                    .map(move |(p2, strength)| (p1, p2, *strength))
            })
            .flatten()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TableType {
    pub n_seats: u32,
}

pub type TableName = String;
pub type TableNameRef = str;

pub type Tables = BTreeMap<TableName, TableType>;

pub type Assignment = BTreeMap<TableName, Vec<PersonName>>;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_tribe() {
        let mut tribe = Tribe::new();
        assert_eq!(tribe.persons_count(), 0);
        assert_eq!(tribe.persons().count(), 0);
        assert_eq!(tribe.relations().count(), 0);

        tribe.add_person("Antoine");
        assert_eq!(tribe.persons_count(), 1);
        assert_eq!(tribe.persons().collect::<Vec<_>>(), vec!["Antoine"]);
        assert_eq!(tribe.relations().count(), 0);

        tribe.add_person("Mathieu");
        assert_eq!(tribe.persons_count(), 2);
        assert_eq!(
            tribe.persons().map(AsRef::as_ref).collect::<HashSet<_>>(),
            HashSet::from(["Antoine", "Mathieu"]),
        );
        assert_eq!(tribe.relations().count(), 0);

        tribe.add_relation("Antoine", "Mathieu", RelationStrength::Likes);
        tribe.add_relation("Antoine", "Charles", RelationStrength::Dislikes);
        assert_eq!(tribe.persons_count(), 3);
        assert_eq!(
            tribe.persons().map(AsRef::as_ref).collect::<HashSet<_>>(),
            HashSet::from(["Charles", "Antoine", "Mathieu"]),
        );
        assert_eq!(tribe.relations().count(), 2);
    }
}
