use std::collections::HashMap;

struct RawSofCalculator {
    sof_sum: f64,
    count: i64
}

impl RawSofCalculator {
    pub fn new() -> Self {
        return RawSofCalculator{
            sof_sum: 0.0,
            count: 0,
        };
    }

    fn add_driver(&mut self, irating: i64) {
        self.sof_sum += f64::powf(2.0, -Self::correct_irating(irating) as f64 / 1600.0);
        self.count += 1;
    }

    fn calc_sof(&self) -> i64 {
        return ((1600.0 / f64::ln(2.0)) * f64::ln(self.count as f64 / self.sof_sum)) as i64;
    }

    fn get_count(&self) -> i64 {
        return self.count;
    }

    fn correct_irating(irating: i64) -> i64 {
        return if irating == -1 { 1350 } else { irating };
    }
}

pub struct SofCalculator {
    overall_sof_calcualtor: RawSofCalculator,
    team_sof_calculator: RawSofCalculator
}

// https://members.iracing.com/jforum/posts/list/3586268.page
impl SofCalculator {
    pub fn new() -> Self {
        return SofCalculator{
            overall_sof_calcualtor: RawSofCalculator::new(),
            team_sof_calculator: RawSofCalculator::new(),
        };
    }
    pub fn begin_team(&mut self) {
        self.team_sof_calculator = RawSofCalculator::new();
    }
    pub fn add_team_driver(&mut self, irating: i64) {
        self.team_sof_calculator.add_driver(irating);
    }
    pub fn end_team(&mut self) {
        self.overall_sof_calcualtor.add_driver(self.team_sof_calculator.calc_sof());
    }
    pub fn add_solo_driver(&mut self, irating: i64) {
        self.overall_sof_calcualtor.add_driver(irating);
    }
    pub fn calc_sof(&self) -> i64 {
        return self.overall_sof_calcualtor.calc_sof();
    }
    pub fn get_team_count(&self) -> i64 {
        return self.overall_sof_calcualtor.get_count();
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