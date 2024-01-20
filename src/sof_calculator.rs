use std::collections::HashMap;

pub struct SofCalculator {
    team_sum: i64,
    team_member_count: i64,

    sof_sum: f64,
    team_count: i64,
}

// https://members.iracing.com/jforum/posts/list/3586268.page
impl SofCalculator {
    pub fn new() -> Self {
        return SofCalculator{
            team_sum: 0,
            team_member_count: 0,
            sof_sum: 0.0,
            team_count: 0
        };
    }
    pub fn begin_team(&mut self) {
        self.team_sum = 0;
        self.team_member_count = 0;
    }
    pub fn add_team_driver(&mut self, irating: i64) {
        self.team_sum += Self::correct_irating(irating);
        self.team_member_count += 1;
    }
    pub fn end_team(&mut self) {
        self.add_team(self.team_sum as f64 / self.team_member_count as f64);
    }
    pub fn add_solo_driver(&mut self, irating: i64) {
        self.add_team(Self::correct_irating(irating) as f64);
    }

    pub fn calc_sof(&self) -> i64 {
        return ((1600.0 / f64::ln(2.0)) * f64::ln(self.team_count as f64 / self.sof_sum)) as i64;
    }
    pub fn get_team_count(&self) -> i64 {
        return self.team_count;
    }

    fn add_team(&mut self, irating: f64) {
        let corrected_irating = if irating == -1.0 { 1350.0 } else { irating };
        self.sof_sum += f64::powf(2.0, -corrected_irating  / 1600.0);
        self.team_count += 1;
    }

    fn correct_irating(irating: i64) -> i64 {
        return if irating == -1 { 1350 } else { irating };
    }
}

pub struct SofCalculators {
    pub total_sof_calculator: SofCalculator,
    pub class_sof_calculators: HashMap<i64, SofCalculator>,
    current_team_class_id: i64
}

impl SofCalculators {
    pub fn new() -> Self {
        return SofCalculators{
            total_sof_calculator: SofCalculator::new(),
            class_sof_calculators: HashMap::new(),
            current_team_class_id: -1,
        }
    }

    pub fn begin_team(&mut self, class_id: i64) {
        self.current_team_class_id = class_id;

        let class_sof_calculator = self.class_sof_calculators.entry(self.current_team_class_id).or_insert(SofCalculator::new());
        class_sof_calculator.begin_team();
        self.total_sof_calculator.begin_team();
    }

    pub fn add_team_driver(&mut self, irating: i64) {
        let class_sof_calculator = self.class_sof_calculators.entry(self.current_team_class_id).or_insert(SofCalculator::new());
        class_sof_calculator.add_team_driver(irating);
        self.total_sof_calculator.add_team_driver(irating);
    }

    pub fn end_team(&mut self) {
        let class_sof_calculator = self.class_sof_calculators.entry(self.current_team_class_id).or_insert(SofCalculator::new());
        class_sof_calculator.end_team();
        self.total_sof_calculator.end_team();
    }

    pub fn add_solo_driver(&mut self, class_id: i64, irating: i64) {
        let class_sof_calculator = self.class_sof_calculators.entry(class_id).or_insert(SofCalculator::new());
        class_sof_calculator.add_solo_driver(irating);
        self.total_sof_calculator.add_solo_driver(irating);
    }
}