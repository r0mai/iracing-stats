use sea_query::Iden;

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