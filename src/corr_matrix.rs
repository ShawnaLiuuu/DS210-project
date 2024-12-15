use polars::prelude::*;

pub(crate) struct CorrMatrix {
    // Region names
    pub(crate) regions: Vec<PlSmallStr>,
    // 1x(num regions^2) df storing corrs
    pub(crate) corr_df: DataFrame,
}

impl CorrMatrix {
    pub(crate) fn get(&self, i: usize, j: usize) -> f64 {
        // Query corr_df with region names for corr
        let corr = self
            .corr_df
            .column(format!("{}, {}", self.regions[i], self.regions[j]).as_str())
            .unwrap()
            .get(0)
            .unwrap()
            .try_extract::<f64>()
            .unwrap();

        corr
    }
}
