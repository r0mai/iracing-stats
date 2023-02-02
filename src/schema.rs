use sea_query::{
    Iden,
    Expr,
    SimpleExpr,
    SelectStatement,
    all
};

use crate::event_type::EventType;
use crate::category_type::CategoryType;

#[derive(Iden)]
pub enum Driver {
    Table,
    CustId,
    DisplayName,
}

#[derive(Iden)]
pub enum Session {
    Table,
    SessionId,
    SeriesName,
}

#[derive(Iden)]
pub enum Subsession {
    Table,
    SubsessionId,
    SessionId,
    StartTime,
    LicenseCategoryId,
    EventType,
    TrackId,
}

#[derive(Iden)]
pub enum DriverResult {
    Table,
    CustId,
    TeamId,
    SubsessionId,
    SimsessionNumber,
    OldiRating,
    NewiRating,
    OldCpi,
    NewCpi,
    Incidents,
    LapsComplete,
    AverageLap,
    CarId,
    FinishPosition,
    FinishPositionInClass,
}

#[derive(Iden)]
pub enum Simsession {
    Table,
    SubsessionId,
    SimsesionId,
    SimsessionNumber,
    SimsessionType,
}

#[derive(Iden)]
pub enum TrackConfig {
    Table,
    TrackId,
    PackageId,
    ConfigName,
    TrackConfigLength,
}

#[derive(Iden)]
pub enum Track {
    Table,
    PackageId,
    TrackName,
}

#[derive(Iden)]
pub enum Car {
    Table,
    CarId,
    CarName,
    CarNameAbbreviated,
}

pub trait SchemaJoins {
    fn join_driver_result_to_simsession(&mut self) -> &mut Self;
    fn join_driver_result_to_subsession(&mut self) -> &mut Self;
    fn join_driver_result_to_driver(&mut self) -> &mut Self;
    fn join_subsession_to_session(&mut self) -> &mut Self;
}

impl SchemaJoins for SelectStatement {
    fn join_driver_result_to_simsession(&mut self) -> &mut Self {
        return self.inner_join(Simsession::Table, all![
            Expr::col((DriverResult::Table, DriverResult::SubsessionId)).equals((Simsession::Table, Simsession::SubsessionId)),
            Expr::col((DriverResult::Table, DriverResult::SimsessionNumber)).equals((Simsession::Table, Simsession::SimsessionNumber)),
        ]);
    }
    fn join_driver_result_to_subsession(&mut self) -> &mut Self {
        return self.inner_join(Subsession::Table,
            Expr::col((DriverResult::Table, DriverResult::SubsessionId)).equals((Subsession::Table, Subsession::SubsessionId))
        );
    }
    fn join_driver_result_to_driver(&mut self) -> &mut Self {
        return self.inner_join(Driver::Table,
            Expr::col((DriverResult::Table, DriverResult::CustId)).equals((Driver::Table, Driver::CustId))
        );
    }
    fn join_subsession_to_session(&mut self) -> &mut Self {
        return self.inner_join(Session::Table,
            Expr::col((Session::Table, Session::SessionId)).equals((Subsession::Table, Subsession::SessionId))
        );
    }
}

pub fn is_event_type(event_type: EventType) -> SimpleExpr {
    return Expr::col((Subsession::Table, Subsession::EventType)).eq(event_type.to_db_type());
}

pub fn is_category_type(category_type: CategoryType) -> SimpleExpr {
    return Expr::col((Subsession::Table, Subsession::LicenseCategoryId)).eq(category_type.to_db_type());
}