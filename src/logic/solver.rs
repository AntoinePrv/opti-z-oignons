use super::model;

pub type UnsolvableError = String;

pub fn solve(
    tables: &model::Tables,
    tribe: &model::Tribe,
) -> Result<model::Assignment, UnsolvableError> {
    let mut solver = Solver::new(tables, tribe);
    solver.fake_solve()
}

type PersonIdx = u32;
type TableIdx = u32;
type SeatIdx = u32;

type TableSize = u32;

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

    pub fn table_name(&self, idx: TableIdx) -> &'a model::TableNameRef {
        *self.table_names.get(idx as usize).unwrap()
    }

    pub fn person_name(&self, idx: PersonIdx) -> &'a model::PersonNameRef {
        *self.person_names.get(idx as usize).unwrap()
    }
}

struct Assignor {
    table_ptrs: Vec</* TableIdx, */ SeatIdx>,
    seat_assignment: Vec</* SeatIdx, */ PersonIdx>,
    person_assignment: Vec</* PersonIdx, */ TableIdx>,
}

impl Assignor {
    const UNASSIGNED_SEAT: PersonIdx = PersonIdx::MAX;
    const UNASSIGNED_PERSON: TableIdx = TableIdx::MAX;

    pub fn from_table_sizes(mut table_sizes: Vec<TableSize>, n_persons: usize) -> Self {
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
            person_assignment: vec![Self::UNASSIGNED_PERSON; n_persons],
        }
    }

    pub fn fake_assign(&mut self) {
        for (p, s) in self.persons().zip(self.seat_assignment.iter_mut()) {
            *s = p;
        }
    }

    pub fn n_tables(&self) -> usize {
        // TODO or should it be u32?
        self.table_ptrs.len() - 1
    }

    pub fn tables(&self) -> impl Iterator<Item = TableIdx> {
        (0..self.n_tables() as u32).into_iter()
    }

    pub fn n_seats(&self) -> usize {
        self.table_ptrs.last().copied().unwrap_or(0) as usize
    }

    pub fn n_persons(&self) -> usize {
        self.person_assignment.len()
    }

    pub fn persons(&self) -> impl Iterator<Item = PersonIdx> {
        (0..self.n_persons() as u32).into_iter()
    }

    pub fn persons_at_table(&self, idx: TableIdx) -> &[PersonIdx] {
        let idx = idx as usize;
        let table_start = self.table_ptrs[idx] as usize;
        let table_end = self.table_ptrs[idx + 1] as usize;
        let seats = &self.seat_assignment[table_start..table_end];

        let free = seats.iter().position(|s| *s == Self::UNASSIGNED_SEAT);
        &seats[table_start..free.unwrap_or(table_end)]
    }

    pub fn persons_at_tables(&self) -> impl Iterator<Item = (TableIdx, &[PersonIdx])> {
        self.tables().map(|t| (t, self.persons_at_table(t)))
    }
}

struct Solver<'a> {
    assignor: Assignor,
    mapping: BackwardMapping<'a>,
}

impl<'a> Solver<'a> {
    pub fn new(tables: &'a model::Tables, tribe: &'a model::Tribe) -> Self {
        // TODO check #persons #seats < u32::MAX

        let mut table_names = Vec::</* TableIdx, */ &'a model::TableNameRef>::new();
        table_names.reserve(tables.len());
        let mut table_sizes = Vec::</* TableIdx, */ SeatIdx>::new();
        table_sizes.reserve(tables.len() + 1);

        for (name, table) in tables.iter() {
            table_names.push(name.as_ref());
            table_sizes.push(table.n_seats);
        }

        table_names.sort_unstable_by_key(|n| tables.get(*n).unwrap().n_seats);
        table_sizes.sort();

        Self {
            assignor: Assignor::from_table_sizes(table_sizes, tribe.persons_count()),
            mapping: BackwardMapping::new(
                table_names,
                tribe.persons().map(AsRef::as_ref).collect::<Vec<_>>(),
            ),
        }
    }

    fn assignment(&self) -> model::Assignment {
        let mut out = model::Assignment::new();
        for (table_idx, persons_idx) in self.assignor.persons_at_tables() {
            let table_name = self.mapping.table_name(table_idx).to_owned();
            let person_names: Vec<_> = persons_idx
                .iter()
                .map(|p| self.mapping.person_name(*p).to_owned())
                .collect();
            let prev = out.insert(table_name, person_names);
            assert!(prev.is_none());
        }
        out
    }

    pub fn fake_solve(&mut self) -> Result<model::Assignment, UnsolvableError> {
        if self.assignor.n_seats() < self.assignor.n_persons() {
            return Err("There is not enough sitting space".into());
        }

        self.assignor.fake_assign();
        Ok(self.assignment())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backward_mapping() {
        let mapping = BackwardMapping::new(vec!["Oak"], vec!["A", "B"]);

        assert_eq!(mapping.table_name(0), "Oak");
        assert_eq!(mapping.person_name(1), "B");
    }
}
