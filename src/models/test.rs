//use diesel::backend::{BindCollector, QueryBuilder};
//use diesel::impl_query_id; // deprecated since 1.1.0
use diesel::pg::Pg;
use diesel::query_builder::{AsQuery, AstPass, Query, QueryFragment};
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::BigInt;
use diesel::QueryId;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
// use diesel_demo::establish_connection;
// use diesel_demo::models::Post;
// use diesel_demo::schema::posts;

// Implement `QueryFragment`
impl<T> QueryFragment<Pg> for PaginatedQuery<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") as internal LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.page)?;
        Ok(())
    }
}

// impl_query_id!(PaginatedQuery<T>);

impl<T: Query> Query for PaginatedQuery<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for PaginatedQuery<T> {}

pub trait Paginate: AsQuery + Sized {
    fn paginate(self, page: i64) -> PaginatedQuery<Self::Query> {
        PaginatedQuery {
            query: self.as_query(),
            page,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl<T: AsQuery> Paginate for T {}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, QueryId)]
pub struct PaginatedQuery<T> {
    query: T,
    page: i64,
    per_page: i64,
}

impl<T> PaginatedQuery<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        PaginatedQuery { per_page, ..self }
    }

    fn load_and_count_pages<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results: Vec<(U, i64)> = self.load(conn)?;
        let total = results.get(0).map(|(_, total)| total).unwrap_or(&0);
        let total = f64::from(*total as i32);
        let records: Vec<U> = results.into_iter().map(|(record, _)| record).collect();
        let total_pages = (total / per_page as f64).ceil() as i64;
        Ok((records, total_pages))
    }
}

// fn main() {
//     let conn = establish_connection();
//     let results: Vec<(Post, i64)> = posts::table
//         .paginate(3)
//         .per_page(25)
//         .get_results(&conn)
//         .expect("error");
//     info!("{:?}", results);

//     let paginated_query = posts::table.paginate(3).per_page(2);
//     let records: (Vec<Post>, i64) = paginated_query.load_and_count_pages(&conn).expect("error");
//     info!("*** records {:?}", records.0);
//     info!("*** pages {:?}", records.1);
// }
