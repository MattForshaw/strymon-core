use abomonation::Abomonation;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryId(pub u64);

impl From<u64> for QueryId {
    fn from(id: u64) -> QueryId {
        QueryId(id)
    }
}

#[derive(Clone, Debug)]
pub struct QueryParams {
    pub id: QueryId,
    pub threads: usize,
    pub processes: usize,
    pub hostlist: Vec<String>,
}

unsafe_abomonate!(QueryId);
unsafe_abomonate!(QueryParams: id, threads, processes, hostlist);
