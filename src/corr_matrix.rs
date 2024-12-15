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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_corr_panic() {
        let regions: Vec<PlSmallStr> = vec!["Boston".into()];

        let corr_df = df![
            "Boston, Boston" => [1.0],
        ]
        .unwrap();

        let corr_matrix = CorrMatrix { regions, corr_df };

        corr_matrix.get(1, 2);
    }

    #[test]
    fn test_corr_small() {
        let regions: Vec<PlSmallStr> = vec!["Boston".into(), "New York".into()];

        let corr_df = df![
            "Boston, Boston" => [1.0],
            "Boston, New York" => [0.5],
            "New York, Boston" => [0.5],
            "New York, New York" => [1.0],
        ]
        .unwrap();

        let corr_matrix = CorrMatrix { regions, corr_df };

        assert_eq!(corr_matrix.get(0, 0), 1.0);
        assert_eq!(corr_matrix.get(0, 1), 0.5);
        assert_eq!(corr_matrix.get(1, 0), 0.5);
        assert_eq!(corr_matrix.get(1, 1), 1.0);
    }

    #[test]
    fn test_corr_big() {
        let regions: Vec<PlSmallStr> = vec![
            "Boston".into(),
            "Chicago".into(),
            "New York".into(),
            "San Francisco".into(),
        ];

        let corr_df = df![
            "Boston, Boston" => [1.0],
            "Boston, Chicago" => [0.67],
            "Boston, New York" => [0.33],
            "Boston, San Francisco" => [0.0],
        ]
        .unwrap();

        let corr_matrix = CorrMatrix { regions, corr_df };

        assert_eq!(corr_matrix.get(0, 0), 1.0);
        assert_eq!(corr_matrix.get(0, 1), 0.67);
        assert_eq!(corr_matrix.get(0, 2), 0.33);
        assert_eq!(corr_matrix.get(0, 3), 0.0);
    }
}
