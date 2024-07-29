use sea_query::{
    Iden,
    Func,
    Expr,
    SimpleExpr,
    SelectStatement,
    all
};

use crate::event_type::EventType;
use crate::simsession_type::SimsessionType;
use crate::category_type::CategoryType;
use crate::driverid::DriverId;

#[derive(Iden)]
pub enum Driver {
    Table,
    CustId,
    DisplayName,
}

#[derive(Iden)]
pub enum Season {
    Table,
    SeasonId,
    SeriesId,
    SeasonName,
    SeriesName,
    Official,
    SeasonYear,
    SeasonQuarter,
    LicenseGroupId, // ~= LicenseCategoryId
    FixedSetup,
    DriverChanges,
}

#[derive(Iden)]
pub enum Session {
    Table,
    SessionId,
    SeriesName,
    SessionName,
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
    OfficialSession,
}

#[derive(Iden)]
pub enum DriverResult {
    Table,
    CustId,
    TeamId,
    TeamName,
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
    CarClassId,
    FinishPosition,
    FinishPositionInClass,
    ReasonOutId,
}

#[derive(Iden)]
pub enum CarClass {
    Table,
    CarClassId,
    CarClassName,
    CarClassShortName,
    CarClassSize
}

#[derive(Iden)]
pub enum CarClassMember {
    Table,
    CarClassId,
    CarId
}

#[derive(Iden)]
pub enum CarClassResult {
    Table,
    CarClassId,
    SubsessionId,
    SimsessionNumber,
    EntriesInClass,
    ClassSof,
}

#[derive(Iden)]
pub enum Simsession {
    Table,
    SubsessionId,
    SimsesionId,
    SimsessionNumber,
    SimsessionType,
    Entries,
    Sof,
}

#[derive(Iden)]
pub enum ReasonOut {
    Table,
    ReasonOutId,
    ReasonOut,
}

#[derive(Iden)]
pub enum TrackConfig {
    Table,
    TrackId,
    PackageId,
    TrackName,
    ConfigName,
    TrackConfigLength,
    CornersPerLap,
    CategoryId, // road/oval/dirt road/dirt oval
    GridStalls,
    PitRoadSpeedLimit,
    NumberPitstalls,
}

#[derive(Iden)]
pub enum Car {
    Table,
    CarId,
    CarName,
    CarNameAbbreviated,
}

#[derive(Iden)]
pub enum SiteTeam {
    Table,
    SiteTeamId,
    SiteTeamName,
    DiscordHookUrl,
    TeamReportDiscordHookUrl,
}

#[derive(Iden)]
pub enum SiteTeamMember {
    Table,
    SiteTeamId,
    CustId
}

#[derive(Iden)]
pub enum SiteTeamTeam {
    Table,
    SiteTeamId,
    TeamId
}

pub trait SchemaUtils {
    fn expr_total_time(&mut self) -> &mut Self;
    fn expr_laps_complete(&mut self) -> &mut Self;
    fn expr_total_distance(&mut self) -> &mut Self;
    fn expr_corrected_license_category(&mut self) -> &mut Self;
    fn expr_normalized_oldi_rating(&mut self) -> &mut Self;
    fn expr_normalized_newi_rating(&mut self) -> &mut Self;
    fn join_driver_result_to_simsession(&mut self) -> &mut Self;
    fn join_driver_result_to_subsession(&mut self) -> &mut Self;
    fn join_driver_result_to_driver(&mut self) -> &mut Self;
    fn join_driver_result_to_car(&mut self) -> &mut Self;
    fn join_driver_result_to_reason_out(&mut self) -> &mut Self;
    fn join_driver_result_to_car_class(&mut self) -> &mut Self;
    fn join_subsession_to_session(&mut self) -> &mut Self;
    fn join_subsession_to_track_config(&mut self) -> &mut Self;
    fn join_site_team_to_site_team_member(&mut self) -> &mut Self;
    fn join_site_team_to_site_team_team(&mut self) -> &mut Self;
    fn join_site_team_team_to_site_team(&mut self) -> &mut Self;
    fn join_driver_result_to_site_team_team(&mut self) -> &mut Self;
    fn join_site_team_member_to_site_team(&mut self) -> &mut Self;
    fn join_site_team_member_to_driver(&mut self) -> &mut Self;
    fn join_driver_to_site_team_member(&mut self) -> &mut Self;
    fn join_driver_result_to_car_class_result(&mut self) -> &mut Self;
    fn match_driver_id(&mut self, driver_id: &DriverId, force_join: bool) -> &mut Self;
}

impl SchemaUtils for SelectStatement {
    fn expr_total_time(&mut self) -> &mut Self {
        return self.expr(Func::sum(Expr::expr(Expr::col(DriverResult::LapsComplete)).mul(Expr::col(DriverResult::AverageLap))));
    }

    fn expr_laps_complete(&mut self) -> &mut Self {
        return self.expr(Func::sum(Expr::col(DriverResult::LapsComplete)));
    }

    fn expr_total_distance(&mut self) -> &mut Self {
        return self.expr(Func::sum(Expr::expr(Expr::col(DriverResult::LapsComplete)).mul(Expr::col(TrackConfig::TrackConfigLength))))
    }

    fn expr_corrected_license_category(&mut self) -> &mut Self {
        // https://forums.iracing.com/discussion/15068/general-availability-of-data-api/p26
        return self.expr(
            Expr::case(Expr::col((Subsession::Table, Subsession::StartTime)).gt("2020-11-08 00:00:00+00:00"),
                Expr::col((TrackConfig::Table, TrackConfig::CategoryId)))
            .finally(
                Expr::col((Subsession::Table, Subsession::LicenseCategoryId)))
        );
    }

    // returns 1350 on -1 irating
    fn expr_normalized_oldi_rating(&mut self) -> &mut Self {
        return self.expr(
            Expr::case(Expr::col((DriverResult::Table, DriverResult::OldiRating)).ne(-1),
                Expr::col((DriverResult::Table, DriverResult::OldiRating)))
            .finally(
                1350)
        );
    }

    // returns 1350 on -1 irating
    fn expr_normalized_newi_rating(&mut self) -> &mut Self {
        return self.expr(
            Expr::case(Expr::col((DriverResult::Table, DriverResult::NewiRating)).ne(-1),
                Expr::col((DriverResult::Table, DriverResult::NewiRating)))
            .finally(
                1350)
        );
    }

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

    fn join_driver_result_to_car(&mut self) -> &mut Self {
        return self.inner_join(Car::Table,
            Expr::col((DriverResult::Table, DriverResult::CarId)).equals((Car::Table, Car::CarId))
        );
    }

    fn join_driver_result_to_reason_out(&mut self) -> &mut Self {
        return self.inner_join(ReasonOut::Table,
            Expr::col((DriverResult::Table, DriverResult::ReasonOutId)).equals((ReasonOut::Table, ReasonOut::ReasonOutId))
        );
    }

    fn join_driver_result_to_car_class(&mut self) -> &mut Self {
        return self.inner_join(CarClass::Table,
            Expr::col((DriverResult::Table, DriverResult::CarClassId)).equals((CarClass::Table, CarClass::CarClassId))
        );
    }

    fn join_subsession_to_session(&mut self) -> &mut Self {
        return self.inner_join(Session::Table,
            Expr::col((Session::Table, Session::SessionId)).equals((Subsession::Table, Subsession::SessionId))
        );
    }

    fn join_subsession_to_track_config(&mut self) -> &mut Self {
        return self.inner_join(TrackConfig::Table,
            Expr::col((TrackConfig::Table, TrackConfig::TrackId)).equals((Subsession::Table, Subsession::TrackId))
        );
    }

    fn join_site_team_to_site_team_member(&mut self) -> &mut Self {
        return self.inner_join(SiteTeamMember::Table,
            Expr::col((SiteTeamMember::Table, SiteTeamMember::SiteTeamId)).equals((SiteTeam::Table, SiteTeam::SiteTeamId))
        );
    }

    fn join_site_team_to_site_team_team(&mut self) -> &mut Self {
        return self.inner_join(SiteTeamTeam::Table,
            Expr::col((SiteTeamTeam::Table, SiteTeamTeam::SiteTeamId)).equals((SiteTeam::Table, SiteTeam::SiteTeamId))
        );
    }

    fn join_site_team_team_to_site_team(&mut self) -> &mut Self {
        return self.inner_join(SiteTeam::Table,
            Expr::col((SiteTeam::Table, SiteTeam::SiteTeamId)).equals((SiteTeamTeam::Table, SiteTeamTeam::SiteTeamId))
        );
    }

    fn join_driver_result_to_site_team_team(&mut self) -> &mut Self {
        return self.inner_join(SiteTeamTeam::Table,
            Expr::col((DriverResult::Table, DriverResult::TeamId)).equals((SiteTeamTeam::Table, SiteTeamTeam::TeamId))
        );
    }

    fn join_site_team_member_to_site_team(&mut self) -> &mut Self {
        return self.inner_join(SiteTeam::Table,
            Expr::col((SiteTeam::Table, SiteTeam::SiteTeamId)).equals((SiteTeamMember::Table, SiteTeamMember::SiteTeamId))
        );
    }

    fn join_driver_to_site_team_member(&mut self) -> &mut Self {
        return self.inner_join(SiteTeamMember::Table,
            Expr::col((Driver::Table, Driver::CustId)).equals((SiteTeamMember::Table, SiteTeamMember::CustId))
        );
    }

    fn join_site_team_member_to_driver(&mut self) -> &mut Self {
        return self.inner_join(Driver::Table,
            Expr::col((SiteTeamMember::Table, SiteTeamMember::CustId)).equals((Driver::Table, Driver::CustId))
        );
    }

    fn join_driver_result_to_car_class_result(&mut self) -> &mut Self {
        return self.inner_join(CarClassResult::Table, all![
            Expr::col((DriverResult::Table, DriverResult::SubsessionId)).equals((CarClassResult::Table, CarClassResult::SubsessionId)),
            Expr::col((DriverResult::Table, DriverResult::SimsessionNumber)).equals((CarClassResult::Table, CarClassResult::SimsessionNumber)),
            Expr::col((DriverResult::Table, DriverResult::CarClassId)).equals((CarClassResult::Table, CarClassResult::CarClassId)),
        ]);
    }

    fn match_driver_id(&mut self, driver_id: &DriverId, force_join: bool) -> &mut Self {
        match driver_id {
            DriverId::CustId(cust_id) => {
                if force_join {
                    self.join_driver_result_to_driver();
                }
                self.and_where(Expr::col((DriverResult::Table, DriverResult::CustId)).eq(*cust_id));
            } 
            DriverId::Name(name) => {
                self.join_driver_result_to_driver()
                    .and_where(Expr::col((Driver::Table, Driver::DisplayName)).eq(name));
            }
        };
        return self;
    }
}

pub fn is_main_event() -> SimpleExpr {
    return Expr::col((Simsession::Table, Simsession::SimsessionNumber)).eq(0);
}

pub fn is_event_type(event_type: EventType) -> SimpleExpr {
    return Expr::col((Subsession::Table, Subsession::EventType)).eq(event_type.to_db_type());
}

pub fn is_simsession_type(simsession_type: SimsessionType) -> SimpleExpr {
    return Expr::col((Simsession::Table, Simsession::SimsessionType)).eq(simsession_type.to_db_type());
}

pub fn is_category_type(category_type: CategoryType) -> SimpleExpr {
    return Expr::col((Subsession::Table, Subsession::LicenseCategoryId)).eq(category_type.to_db_type());
}

pub fn is_official() -> SimpleExpr {
    return Expr::col((Subsession::Table, Subsession::OfficialSession)).is(true);
}