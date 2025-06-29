use std::collections::BTreeMap;

use petgraph::visit::EdgeRef;

use super::model::{self, RelationStrength};

type Size = u32;
type Cost = f32;
type PersonIdx = Size;
type TableIdx = Size;
type SeatIdx = Size;

// Perform a static assertion to ensure Size can be safely cast to usize
const _: [(); 1 - ((Size::MAX as usize as Size == Size::MAX) as usize)] = [(); 0];

pub type SolverResult<T> = Result<T, SolverError>;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum SolverError {
    #[error("the problem is too large, {0} (max {max})", max=Size::MAX)]
    ProblemTooLarge(String),
    #[error("no solution could be found, {0}")]
    NoSolution(String),
    #[error("unknown error")]
    Unknown,
}

type RelationStrengthValues = [Cost; RelationStrength::len()];

#[derive(Clone, Debug)]
pub struct SolverSettings {
    pub relation_values: RelationStrengthValues,
}

pub fn solve(tables: &model::Tables, tribe: &model::Tribe) -> SolverResult<model::Assignment> {
    let mut solver = Solver::new(
        tables,
        tribe,
        SolverSettings {
            relation_values: [-4.0, -1.0, 1.0, 4.0],
        },
    )?;
    solver.solve()
}

#[derive(Clone, Debug)]
struct BackwardMapping<'a> {
    table_names: Vec</* TableIdx, */ &'a model::TableNameRef>,
    person_names: Vec</* PersonIdx, */ &'a model::PersonNameRef>,
}

impl<'a> BackwardMapping<'a> {
    pub fn new(
        table_names: Vec</* TableIdx, */ &'a model::TableNameRef>,
        person_names: Vec</* PersonIdx, */ &'a model::PersonNameRef>,
    ) -> Self {
        Self {
            table_names,
            person_names,
        }
    }

    pub fn table_name(&self, idx: TableIdx) -> Option<&'a model::TableNameRef> {
        self.table_names.get(idx as usize).copied()
    }

    pub fn person_name(&self, idx: PersonIdx) -> Option<&'a model::PersonNameRef> {
        self.person_names.get(idx as usize).copied()
    }
}

#[derive(Clone, Debug)]
struct Assignor {
    table_ptrs: Vec</* TableIdx, */ SeatIdx>,
    seat_assignment: Vec</* SeatIdx, */ PersonIdx>,
    person_assignment: Vec</* PersonIdx, */ TableIdx>,
}

impl Assignor {
    const UNASSIGNED_SEAT: PersonIdx = PersonIdx::MAX;
    const UNASSIGNED_PERSON: TableIdx = TableIdx::MAX;

    pub fn from_table_sizes(mut table_sizes: Vec<Size>, n_persons: Size) -> Self {
        // Reserve last index for past-end pointer
        table_sizes.push(0);

        // We want table pointers to start at 0, so we shift the cumulative sum by 1
        let mut prev = 0;
        std::mem::swap(&mut prev, table_sizes.first_mut().unwrap());
        for i in 1..table_sizes.len() {
            // Safe because we access previous element, starting at one
            unsafe {
                std::mem::swap(&mut prev, table_sizes.get_unchecked_mut(i));
                let prev_sum = *table_sizes.get_unchecked(i - 1);
                *table_sizes.get_unchecked_mut(i) += prev_sum;
            }
        }

        let n_seats = *table_sizes.last().unwrap() as usize;
        Self {
            table_ptrs: table_sizes,
            seat_assignment: vec![Self::UNASSIGNED_SEAT; n_seats],
            person_assignment: vec![Self::UNASSIGNED_PERSON; n_persons as usize],
        }
    }

    pub fn persons(&self) -> impl Iterator<Item = PersonIdx> + use<> {
        (0..self.person_count() as u32).into_iter()
    }

    pub fn tables(&self) -> impl Iterator<Item = TableIdx> + use<> {
        (0..self.table_count() as Size).into_iter()
    }

    pub fn seat_count(&self) -> Size {
        self.table_ptrs.last().copied().unwrap_or(0)
    }

    pub fn person_count(&self) -> Size {
        self.person_assignment.len() as Size
    }

    pub fn table_count(&self) -> Size {
        (self.table_ptrs.len() - 1) as Size
    }

    fn table_slice(&self, table: TableIdx) -> &[SeatIdx] {
        let idx = table as usize;
        let table_start = self.table_ptrs[idx] as usize;
        let table_end = self.table_ptrs[idx + 1] as usize;
        &self.seat_assignment[table_start..table_end]
    }

    fn table_slice_mut(&mut self, table: TableIdx) -> &mut [SeatIdx] {
        let idx = table as usize;
        let table_start = self.table_ptrs[idx] as usize;
        let table_end = self.table_ptrs[idx + 1] as usize;
        &mut self.seat_assignment[table_start..table_end]
    }

    fn table_free_seat_offset(&self, table: TableIdx) -> Option<Size> {
        let seats = self.table_slice(table);
        seats
            .iter()
            .position(|s| *s == Self::UNASSIGNED_SEAT)
            .map(|idx| idx as Size)
    }

    pub fn table_person_count(&self, table: TableIdx) -> Size {
        self.table_free_seat_offset(table)
            .unwrap_or(self.table_seat_count(table))
    }

    pub fn table_seat_count(&self, table: TableIdx) -> Size {
        self.table_slice(table).len() as Size
    }

    pub fn table_is_full(&self, table: TableIdx) -> bool {
        self.table_slice(table)
            .last()
            .map(|p| *p != Self::UNASSIGNED_SEAT)
            .unwrap_or(true)
    }

    pub fn table_persons(&self, table: TableIdx) -> &[PersonIdx] {
        let seats = self.table_slice(table);
        &seats[0..(self.table_person_count(table) as usize)]
    }

    pub fn table_assignment(&self) -> impl Iterator<Item = (TableIdx, &[PersonIdx])> {
        self.tables().map(|t| (t, self.table_persons(t)))
    }

    pub fn person_is_seated(&self, person: PersonIdx) -> bool {
        self.person_assignment[person as usize] != Self::UNASSIGNED_PERSON
    }

    pub fn can_assign(&mut self, person: PersonIdx, table: TableIdx) -> bool {
        !self.person_is_seated(person) && !self.table_is_full(table)
    }

    pub fn assign(&mut self, person: PersonIdx, table: TableIdx) -> bool {
        if self.person_is_seated(person) {
            return false;
        }

        let free = self.table_free_seat_offset(table);
        let seats = self.table_slice_mut(table);
        if let Some(s) = free {
            seats[s as usize] = person;
            self.person_assignment[person as usize] = table;
            true
        } else {
            false
        }
    }
}

type RelationGraph = petgraph::csr::Csr<(), RelationStrength, petgraph::Undirected, PersonIdx>;

#[derive(Clone, Debug)]
struct AssignorWithCosts {
    assignor: Assignor,
    relations: RelationGraph,
    relations_values: RelationStrengthValues,
    table_costs: Vec</* TableIdx */ Cost>,
}

impl AssignorWithCosts {
    pub fn new(
        assignor: Assignor,
        relations: RelationGraph,
        relations_values: RelationStrengthValues,
    ) -> Self {
        let n_tables = assignor.table_count();
        Self {
            assignor,
            relations,
            relations_values,
            table_costs: vec![0.0; n_tables as usize],
        }
    }

    pub fn table_cost(&self, table: TableIdx) -> Cost {
        self.table_costs[table as usize]
    }

    pub fn assign_with_cost(&mut self, person: PersonIdx, table: TableIdx, cost: Cost) -> bool {
        if self.assignor.assign(person, table) {
            self.table_costs[table as usize] += cost;
            true
        } else {
            false
        }
    }

    pub fn assignment_cost(&self, person: PersonIdx, table: TableIdx) -> Cost {
        let table_persons = self.assignor.table_persons(table);
        self.relations
            .edges(person)
            .filter(|e| table_persons.contains(&e.target()))
            .map(|e| self.relations_values[*e.weight() as usize])
            .sum()
    }
}

impl AsRef<Assignor> for AssignorWithCosts {
    fn as_ref(&self) -> &Assignor {
        &self.assignor
    }
}

impl std::ops::Deref for AssignorWithCosts {
    type Target = Assignor;

    fn deref(&self) -> &Self::Target {
        &self.assignor
    }
}

impl AsRef<RelationGraph> for AssignorWithCosts {
    fn as_ref(&self) -> &RelationGraph {
        &self.relations
    }
}

#[derive(Clone, Debug)]
struct Solver<'a> {
    assignor: AssignorWithCosts,
    mapping: BackwardMapping<'a>,
    settings: SolverSettings,
}

impl<'pb> Solver<'pb> {
    pub fn new(
        tables: &'pb model::Tables,
        tribe: &'pb model::Tribe,
        settings: SolverSettings,
    ) -> SolverResult<Self> {
        let (table_names, table_sizes) = Self::build_tables(tables)?;
        let (relations, persons) = Self::build_relations(tribe)?;

        let assignor = Assignor::from_table_sizes(table_sizes, tribe.persons_count() as Size);

        Ok(Self {
            assignor: AssignorWithCosts::new(assignor, relations, settings.relation_values),
            mapping: BackwardMapping::new(table_names, persons),
            settings,
        })
    }

    pub fn build_relations<'a>(
        tribe: &'a model::Tribe,
    ) -> SolverResult<(
        RelationGraph,
        Vec</* PersonIdx, */ &'a model::PersonNameRef>,
    )> {
        if tribe.persons_count() >= (Size::MAX as usize) {
            return Err(SolverError::ProblemTooLarge(
                "there are too many persons".into(),
            ));
        }

        let persons = tribe.persons().map(AsRef::as_ref).collect::<Vec<_>>();
        let persons_forward = persons
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, name)| (name, idx as PersonIdx))
            .collect::<BTreeMap<&model::PersonNameRef, PersonIdx>>();

        let mut relations = RelationGraph::with_nodes(tribe.persons_count() as usize);
        for (p1, p2, strenght) in tribe.relations() {
            relations.add_edge(
                // Safe because all indices added
                *persons_forward.get(p1.as_str()).unwrap(),
                *persons_forward.get(p2.as_str()).unwrap(),
                strenght,
            );
        }

        assert_eq!(relations.node_count(), persons.len());
        Ok((relations, persons))
    }

    fn build_tables<'a>(
        tables: &'a model::Tables,
    ) -> SolverResult<(Vec<&'a model::TableNameRef>, Vec<Size>)> {
        if tables.len() >= (Size::MAX as usize) {
            return Err(SolverError::ProblemTooLarge(
                "there are too many tables".into(),
            ));
        }

        let (mut table_names, mut table_sizes): (Vec<&'a model::TableNameRef>, Vec<Size>) = tables
            .iter()
            .map(|(name, typ)| (name.as_str(), typ.n_seats))
            .unzip();

        let n_seats = table_sizes
            .iter()
            .try_fold(0u32, |acc, &x| acc.checked_add(x));
        if n_seats.is_none() {
            Err(SolverError::ProblemTooLarge(
                "there are too many seats".into(),
            ))
        } else {
            table_names.sort_unstable_by_key(|n| tables.get(*n).unwrap().n_seats);
            table_sizes.sort();
            assert_eq!(table_names.len(), table_sizes.len());
            Ok((table_names, table_sizes))
        }
    }

    fn assignment(&self) -> model::Assignment {
        let mut out = model::Assignment::new();
        for (table_idx, persons_idx) in self.assignor.table_assignment() {
            let table_name = self.mapping.table_name(table_idx).unwrap().to_owned();
            let person_names: Vec<_> = persons_idx
                .iter()
                .map(|p| self.mapping.person_name(*p).unwrap().to_owned())
                .collect();
            let prev = out.insert(table_name, person_names);
            assert!(prev.is_none());
        }
        out
    }

    pub fn solve(&mut self) -> SolverResult<model::Assignment> {
        if self.assignor.seat_count() < self.assignor.person_count() {
            return Err(SolverError::NoSolution(
                "there is not enough sitting space".into(),
            ));
        }

        for person in self.assignor.persons() {
            let (cost, table) = self
                .assignor
                .tables()
                .filter(|t| !self.assignor.table_is_full(*t))
                .map(|t| (self.assignor.assignment_cost(person, t), t))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or((0.0, 0));

            self.assignor.assign_with_cost(person, table, cost);
        }

        Ok(self.assignment())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::super::examples;
    use super::*;

    fn assign_in_order(assignor: &mut Assignor) {
        let mut tables = assignor.tables();
        let mut t = tables.next();
        let mut persons = assignor.persons();
        let mut p = persons.next();

        while t.is_some() && p.is_some() {
            if assignor.table_is_full(t.unwrap()) {
                t = tables.next();
            } else {
                let assigned = assignor.assign(p.unwrap(), t.unwrap());
                assert!(assigned);
                p = persons.next();
            }
        }
    }

    #[test]
    fn test_backward_mapping() {
        let mapping = BackwardMapping::new(vec!["Oak"], vec!["A", "B"]);

        assert_eq!(mapping.table_name(0), Some("Oak"));
        assert_eq!(mapping.person_name(1), Some("B"));
        assert_eq!(mapping.person_name(3), None);
    }

    #[test]
    fn test_assignor() {
        let tables = vec![3, 4, 4, 6];
        let n_persons: Size = 15;
        let mut assignor = Assignor::from_table_sizes(tables.clone(), n_persons);

        assert_eq!(assignor.table_count() as usize, tables.len());
        assert_eq!(assignor.seat_count(), tables.iter().sum::<u32>());
        assert_eq!(assignor.person_count(), n_persons);
        assert_eq!(
            assignor.persons().collect::<Vec<_>>(),
            (0..n_persons).collect::<Vec<_>>()
        );
        assert_eq!(
            assignor.tables().collect::<Vec<_>>(),
            (0u32..(tables.len() as u32)).collect::<Vec<_>>()
        );
        assert_eq!(
            assignor.table_assignment().collect::<Vec<_>>(),
            vec![(0, &[] as &[u32]), (1, &[]), (2, &[]), (3, &[]),]
        );

        assign_in_order(&mut assignor);
        assert_eq!(
            assignor.table_assignment().collect::<Vec<_>>(),
            vec![
                (0, &[0u32, 1, 2] as &[u32]),
                (1, &[3u32, 4, 5, 6] as &[u32]),
                (2, &[7u32, 8, 9, 10] as &[u32]),
                (3, &[11u32, 12, 13, 14 /* free, free */,] as &[u32]),
            ]
        );
    }

    #[test]
    fn test_assignor_with_costs() {
        let tables = vec![2, 4];
        let n_persons: Size = 5;

        let mut assignor = {
            let assignor = Assignor::from_table_sizes(tables.clone(), n_persons);

            let mut relations = RelationGraph::with_nodes(n_persons as usize);
            relations.add_edge(0, 1, RelationStrength::Hates);
            relations.add_edge(0, 2, RelationStrength::Dislikes);
            relations.add_edge(0, 3, RelationStrength::Likes);
            relations.add_edge(0, 4, RelationStrength::Loves);
            relations.add_edge(1, 2, RelationStrength::Loves);
            relations.add_edge(1, 3, RelationStrength::Likes);
            relations.add_edge(1, 4, RelationStrength::Dislikes);

            let relations_values = [7.0, 1.0, -3.0, -9.0];

            AssignorWithCosts::new(assignor, relations, relations_values)
        };

        assert_eq!(assignor.person_count(), n_persons);
        assert_eq!(assignor.table_count() as usize, tables.len());

        for t in assignor.tables() {
            assert_eq!(assignor.table_cost(t), 0.0);
            for p in assignor.persons() {
                let cost = assignor.assignment_cost(p, t);
                assert_eq!(cost, 0.0);
            }
        }

        let assigned = assignor.assign_with_cost(0, 0, 0.0);
        assert!(assigned);
        assert!(!assignor.assign_with_cost(0, 0, 0.0));
        let assigned = assignor.assign_with_cost(1, 1, 0.0);
        assert!(assigned);
        assert!(!assignor.assign_with_cost(1, 0, 0.0));
        assert!(!assignor.assign_with_cost(1, 1, 0.0));

        assert_eq!(assignor.assignment_cost(2, 0), 1.0);
        assert_eq!(assignor.assignment_cost(2, 1), -9.0);
        let assigned = assignor.assign_with_cost(2, 0, 1.0);
        assert!(assigned);
        // Unchanged the cost of new table is returned
        assert_eq!(assignor.assignment_cost(2, 1), -9.0);

        // Table 0 is full
        assert_eq!(assignor.assignment_cost(3, 0), -3.0);
        let assigned = assignor.assign_with_cost(3, 0, -3.0);
        assert!(!assigned);
    }

    #[test]
    fn test_solver_empty() -> SolverResult<()> {
        let (tribe, tables) = examples::empty();
        let mut solver = Solver::new(
            &tables,
            &tribe,
            SolverSettings {
                relation_values: [-4.0, -1.0, 1.0, 4.0],
            },
        )?;
        let assignment = solver.solve()?;

        assert!(assignment.is_empty());

        Ok(())
    }

    #[test]
    fn test_solver_harry_potter() -> SolverResult<()> {
        let (tribe, tables) = examples::harry_potter();
        let mut solver = Solver::new(
            &tables,
            &tribe,
            SolverSettings {
                relation_values: [-4.0, -1.0, 1.0, 4.0],
            },
        )?;
        let assignment = solver.solve()?;

        assert_eq!(assignment.len(), tables.len());
        for t in tables.keys() {
            assert!(assignment.contains_key(t));
        }

        let assignees = assignment
            .values()
            .map(|t| t.iter())
            .flatten()
            .collect::<HashSet<_>>();
        assert_eq!(assignees.len(), tribe.persons_count());
        for p in tribe.persons() {
            assert!(assignees.contains(p));
        }

        Ok(())
    }
}
