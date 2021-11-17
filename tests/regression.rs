#[cfg(test)]
mod regression_tests {
    #[test]
    fn print_settings() {
        let settings = automl::regression::Settings::default()
            .skip_algorithms(vec![
                automl::regression::Algorithm::DecisionTree,
                automl::regression::Algorithm::Ridge,
            ])
            .with_number_of_folds(3);
        println!("{}", &settings);
    }

    #[test]
    fn test_step_by_step() {
        use automl::regression::{Algorithm, Metric, Regressor, Settings};
        use smartcore::svm::svr::SVRParameters;
        use smartcore::{
            dataset::diabetes::load_dataset, linalg::naive::dense_matrix::DenseMatrix,
        };

        // Check training
        let dataset = load_dataset();
        let x = DenseMatrix::from_array(dataset.num_samples, dataset.num_features, &dataset.data);
        let y = dataset.target;

        // Set up the regressor settings and load data
        let settings = Settings::default()
            .sorted_by(Metric::MeanAbsoluteError)
            .with_svr_settings(SVRParameters::default().with_eps(2.0).with_c(10.0))
            .skip_algorithms(vec![Algorithm::ElasticNet])
            .with_number_of_folds(2);
        let mut regressor = Regressor::new(x, y, settings);

        // Compare models
        println!("Comparing models...");
        regressor.compare_models();
        print!("{}", regressor);

        // Train final model
        regressor.train_final_model();

        // Do inference with final model
        let x = DenseMatrix::from_2d_array(&[&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]]);
        print!("{:?}", regressor.predict(&x));
    }

    #[test]
    fn test_read_from_csv() {
        use automl::regression::{Algorithm, Metric, Regressor, Settings};
        use smartcore::linalg::naive::dense_matrix::DenseMatrix;
        use smartcore::svm::svr::SVRParameters;

        let file_name = "data/diabetes.csv";

        // Set up the regressor settings and load data
        let settings = Settings::default()
            .sorted_by(Metric::MeanSquaredError)
            .with_svr_settings(SVRParameters::default().with_eps(2.0).with_c(10.0))
            .skip_algorithms(vec![Algorithm::ElasticNet])
            .with_number_of_folds(2);

        let mut regressor = Regressor::default();
        regressor.with_settings(settings);
        regressor.with_data_from_csv(file_name, 10, true);

        regressor.compare_models();
        regressor.train_final_model();
        println!("{}", regressor);

        // Do inference with final model
        let x = DenseMatrix::from_2d_array(&[&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]]);
        print!("{:?}", regressor.predict(&x));
    }
}
