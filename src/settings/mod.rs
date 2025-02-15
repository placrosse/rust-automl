//! # Settings customization
//! This module contains capabilities for the detailed customization of algorithm settings.
//! ## Complete regression customization
//! ```
//! use automl::settings::{
//!     Algorithm, DecisionTreeRegressorParameters, Distance, ElasticNetParameters,
//!     KNNAlgorithmName, KNNRegressorParameters, KNNWeightFunction, Kernel, LassoParameters,
//!     LinearRegressionParameters, LinearRegressionSolverName, Metric,
//!     RandomForestRegressorParameters, RidgeRegressionParameters, RidgeRegressionSolverName,
//!     SVRParameters,
//!  };
//!
//!  let settings = automl::Settings::default_regression()
//!     .with_number_of_folds(3)
//!     .shuffle_data(true)
//!     .verbose(true)
//!     .skip(Algorithm::RandomForestRegressor)
//!     .sorted_by(Metric::RSquared)
//!     .with_linear_settings(
//!         LinearRegressionParameters::default().with_solver(LinearRegressionSolverName::QR),
//!     )
//!     .with_lasso_settings(
//!         LassoParameters::default()
//!             .with_alpha(10.0)
//!             .with_tol(1e-10)
//!             .with_normalize(true)
//!             .with_max_iter(10_000),
//!     )
//!     .with_ridge_settings(
//!         RidgeRegressionParameters::default()
//!             .with_alpha(10.0)
//!             .with_normalize(true)
//!             .with_solver(RidgeRegressionSolverName::Cholesky),
//!     )
//!     .with_elastic_net_settings(
//!         ElasticNetParameters::default()
//!             .with_tol(1e-10)
//!             .with_normalize(true)
//!             .with_alpha(1.0)
//!             .with_max_iter(10_000)
//!             .with_l1_ratio(0.5),
//!     )
//!     .with_knn_regressor_settings(
//!         KNNRegressorParameters::default()
//!             .with_algorithm(KNNAlgorithmName::CoverTree)
//!             .with_k(3)
//!             .with_distance(Distance::Euclidean)
//!             .with_weight(KNNWeightFunction::Uniform),
//!     )
//!     .with_svr_settings(
//!         SVRParameters::default()
//!             .with_eps(1e-10)
//!             .with_tol(1e-10)
//!             .with_c(1.0)
//!             .with_kernel(Kernel::Linear),
//!     )
//!     .with_random_forest_regressor_settings(
//!         RandomForestRegressorParameters::default()
//!             .with_m(100)
//!             .with_max_depth(5)
//!             .with_min_samples_leaf(20)
//!             .with_n_trees(100)
//!             .with_min_samples_split(20),
//!     )
//!     .with_decision_tree_regressor_settings(
//!         DecisionTreeRegressorParameters::default()
//!             .with_min_samples_split(20)
//!             .with_max_depth(5)
//!             .with_min_samples_leaf(20),
//!     );
//! ```
//! ## Complete classification customization
//! ```
//! use automl::settings::{
//!     Algorithm, CategoricalNBParameters, DecisionTreeClassifierParameters, Distance,
//!     GaussianNBParameters, KNNAlgorithmName, KNNClassifierParameters, KNNWeightFunction, Kernel,
//!     LogisticRegressionParameters, Metric, RandomForestClassifierParameters, SVCParameters,
//! };
//!
//! let settings = automl::Settings::default_classification()
//!     .with_number_of_folds(3)
//!     .shuffle_data(true)
//!     .verbose(true)
//!     .skip(Algorithm::RandomForestClassifier)
//!     .sorted_by(Metric::Accuracy)
//!     .with_random_forest_classifier_settings(
//!         RandomForestClassifierParameters::default()
//!             .with_m(100)
//!             .with_max_depth(5)
//!             .with_min_samples_leaf(20)
//!             .with_n_trees(100)
//!             .with_min_samples_split(20),
//!     )
//!     .with_logistic_settings(LogisticRegressionParameters::default())
//!     .with_svc_settings(
//!         SVCParameters::default()
//!             .with_epoch(10)
//!             .with_tol(1e-10)
//!             .with_c(1.0)
//!             .with_kernel(Kernel::Linear),
//!     )
//!     .with_decision_tree_classifier_settings(
//!         DecisionTreeClassifierParameters::default()
//!             .with_min_samples_split(20)
//!             .with_max_depth(5)
//!             .with_min_samples_leaf(20),
//!     )
//!     .with_knn_classifier_settings(
//!         KNNClassifierParameters::default()
//!             .with_algorithm(KNNAlgorithmName::CoverTree)
//!             .with_k(3)
//!             .with_distance(Distance::Euclidean)
//!             .with_weight(KNNWeightFunction::Uniform),
//!     )
//!     .with_gaussian_nb_settings(GaussianNBParameters::default().with_priors(vec![1.0, 1.0]))
//!     .with_categorical_nb_settings(CategoricalNBParameters::default().with_alpha(1.0));
//! ```

pub use crate::utils::{Distance, Kernel};

/// Weighting functions for k-nearest neighbor (KNN) regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::neighbors::KNNWeightFunction;

/// Search algorithms for k-nearest neighbor (KNN) regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::algorithm::neighbour::KNNAlgorithmName;

/// Parameters for random forest regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::ensemble::random_forest_regressor::RandomForestRegressorParameters;

/// Parameters for decision tree regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::tree::decision_tree_regressor::DecisionTreeRegressorParameters;

/// Parameters for elastic net regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::linear::elastic_net::ElasticNetParameters;

/// Parameters for LASSO regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::linear::lasso::LassoParameters;

/// Solvers for linear regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::linear::linear_regression::LinearRegressionSolverName;

/// Parameters for linear regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::linear::linear_regression::LinearRegressionParameters;

/// Parameters for ridge regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::linear::ridge_regression::RidgeRegressionParameters;

/// Solvers for ridge regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::linear::ridge_regression::RidgeRegressionSolverName;

/// Parameters for Gaussian naive bayes (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::naive_bayes::gaussian::GaussianNBParameters;

/// Parameters for categorical naive bayes (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::naive_bayes::categorical::CategoricalNBParameters;

/// Parameters for random forest classification (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::ensemble::random_forest_classifier::RandomForestClassifierParameters;

/// Parameters for logistic regression (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::linear::logistic_regression::LogisticRegressionParameters;

/// Parameters for decision tree classification (re-export from [Smartcore](https://docs.rs/smartcore/))
pub use smartcore::tree::decision_tree_classifier::DecisionTreeClassifierParameters;

mod knn_regressor_parameters;
pub use knn_regressor_parameters::KNNRegressorParameters;

mod svr_parameters;
pub use svr_parameters::SVRParameters;

mod knn_classifier_parameters;
pub use knn_classifier_parameters::KNNClassifierParameters;

mod svc_parameters;
pub use svc_parameters::SVCParameters;

use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use std::fmt::{Display, Formatter};

use super::algorithms::{
    CategoricalNaiveBayesClassifierWrapper, DecisionTreeClassifierWrapper,
    DecisionTreeRegressorWrapper, ElasticNetRegressorWrapper, GaussianNaiveBayesClassifierWrapper,
    KNNClassifierWrapper, KNNRegressorWrapper, LassoRegressorWrapper, LinearRegressorWrapper,
    LogisticRegressionWrapper, ModelWrapper, RandomForestClassifierWrapper,
    RandomForestRegressorWrapper, RidgeRegressorWrapper, SupportVectorClassifierWrapper,
    SupportVectorRegressorWrapper,
};

mod settings_struct;
#[doc(no_inline)]
pub use settings_struct::Settings;

/// Metrics for evaluating algorithms
#[non_exhaustive]
#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Metric {
    /// Sort by R^2
    RSquared,
    /// Sort by MAE
    MeanAbsoluteError,
    /// Sort by MSE
    MeanSquaredError,
    /// Sort by Accuracy
    Accuracy,
    /// Sort by none
    None,
}

impl Display for Metric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Metric::RSquared => write!(f, "R^2"),
            Metric::MeanAbsoluteError => write!(f, "MAE"),
            Metric::MeanSquaredError => write!(f, "MSE"),
            Metric::Accuracy => write!(f, "Accuracy"),
            Metric::None => panic!("A metric must be set."),
        }
    }
}

/// Algorithm options
#[derive(PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    /// Decision tree regressor
    DecisionTreeRegressor,
    /// KNN Regressor
    KNNRegressor,
    /// Random forest regressor
    RandomForestRegressor,
    /// Linear regressor
    Linear,
    /// Ridge regressor
    Ridge,
    /// Lasso regressor
    Lasso,
    /// Elastic net regressor
    ElasticNet,
    /// Support vector regressor
    SVR,
    /// Decision tree classifier
    DecisionTreeClassifier,
    /// KNN classifier
    KNNClassifier,
    /// Random forest classifier
    RandomForestClassifier,
    /// Support vector classifier
    SVC,
    /// Logistic regression classifier
    LogisticRegression,
    /// Gaussian Naive Bayes classifier
    GaussianNaiveBayes,
    /// Categorical Naive Bayes classifier
    CategoricalNaiveBayes,
}

impl Algorithm {
    pub(crate) fn get_predictor(
        &self,
    ) -> Box<dyn Fn(&DenseMatrix<f32>, &Vec<u8>, &Settings) -> Vec<f32>> {
        match self {
            Algorithm::Linear => Box::new(LinearRegressorWrapper::predict),
            Algorithm::Lasso => Box::new(LassoRegressorWrapper::predict),
            Algorithm::Ridge => Box::new(RidgeRegressorWrapper::predict),
            Algorithm::ElasticNet => Box::new(ElasticNetRegressorWrapper::predict),
            Algorithm::RandomForestRegressor => Box::new(RandomForestRegressorWrapper::predict),
            Algorithm::KNNRegressor => Box::new(KNNRegressorWrapper::predict),
            Algorithm::SVR => Box::new(SupportVectorRegressorWrapper::predict),
            Algorithm::DecisionTreeRegressor => Box::new(DecisionTreeRegressorWrapper::predict),
            Algorithm::LogisticRegression => Box::new(LogisticRegressionWrapper::predict),
            Algorithm::RandomForestClassifier => Box::new(RandomForestClassifierWrapper::predict),
            Algorithm::DecisionTreeClassifier => Box::new(DecisionTreeClassifierWrapper::predict),
            Algorithm::KNNClassifier => Box::new(KNNClassifierWrapper::predict),
            Algorithm::SVC => Box::new(SupportVectorClassifierWrapper::predict),
            Algorithm::GaussianNaiveBayes => Box::new(GaussianNaiveBayesClassifierWrapper::predict),
            Algorithm::CategoricalNaiveBayes => {
                Box::new(CategoricalNaiveBayesClassifierWrapper::predict)
            }
        }
    }

    pub(crate) fn get_trainer(
        &self,
    ) -> Box<dyn Fn(&DenseMatrix<f32>, &Vec<f32>, &Settings) -> Vec<u8>> {
        match self {
            Algorithm::Linear => Box::new(LinearRegressorWrapper::train),
            Algorithm::Lasso => Box::new(LassoRegressorWrapper::train),
            Algorithm::Ridge => Box::new(RidgeRegressorWrapper::train),
            Algorithm::ElasticNet => Box::new(ElasticNetRegressorWrapper::train),
            Algorithm::RandomForestRegressor => Box::new(RandomForestRegressorWrapper::train),
            Algorithm::KNNRegressor => Box::new(KNNRegressorWrapper::train),
            Algorithm::SVR => Box::new(SupportVectorRegressorWrapper::train),
            Algorithm::DecisionTreeRegressor => Box::new(DecisionTreeRegressorWrapper::train),
            Algorithm::LogisticRegression => Box::new(LogisticRegressionWrapper::train),
            Algorithm::RandomForestClassifier => Box::new(RandomForestClassifierWrapper::train),
            Algorithm::DecisionTreeClassifier => Box::new(DecisionTreeClassifierWrapper::train),
            Algorithm::KNNClassifier => Box::new(KNNClassifierWrapper::train),
            Algorithm::SVC => Box::new(SupportVectorClassifierWrapper::train),
            Algorithm::GaussianNaiveBayes => Box::new(GaussianNaiveBayesClassifierWrapper::train),
            Algorithm::CategoricalNaiveBayes => {
                Box::new(CategoricalNaiveBayesClassifierWrapper::train)
            }
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::DecisionTreeRegressor => write!(f, "Decision Tree Regressor"),
            Algorithm::KNNRegressor => write!(f, "KNN Regressor"),
            Algorithm::RandomForestRegressor => write!(f, "Random Forest Regressor"),
            Algorithm::Linear => write!(f, "Linear Regressor"),
            Algorithm::Ridge => write!(f, "Ridge Regressor"),
            Algorithm::Lasso => write!(f, "LASSO Regressor"),
            Algorithm::ElasticNet => write!(f, "Elastic Net Regressor"),
            Algorithm::SVR => write!(f, "Support Vector Regressor"),
            Algorithm::DecisionTreeClassifier => write!(f, "Decision Tree Classifier"),
            Algorithm::KNNClassifier => write!(f, "KNN Classifier"),
            Algorithm::RandomForestClassifier => write!(f, "Random Forest Classifier"),
            Algorithm::LogisticRegression => write!(f, "Logistic Regression Classifier"),
            Algorithm::SVC => write!(f, "Support Vector Classifier"),
            Algorithm::GaussianNaiveBayes => write!(f, "Gaussian Naive Bayes"),
            Algorithm::CategoricalNaiveBayes => write!(f, "Categorical Naive Bayes"),
        }
    }
}

/// Options for pre-processing the data
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PreProcessing {
    /// Don't do any preprocessing
    None,
    /// Add interaction terms to the data
    AddInteractions,
    /// Add polynomial terms of order n to the data
    AddPolynomial {
        /// The order of the polynomial to add (i.e., x^order)
        order: usize,
    },
    /// Replace the data with n PCA terms
    ReplaceWithPCA {
        /// The number of components to use from PCA
        number_of_components: usize,
    },
    /// Replace the data with n PCA terms
    ReplaceWithSVD {
        /// The number of components to use from PCA
        number_of_components: usize,
    },
}

impl Display for PreProcessing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PreProcessing::None => write!(f, "None"),
            PreProcessing::AddInteractions => write!(f, "Interaction terms added"),
            PreProcessing::AddPolynomial { order } => {
                write!(f, "Polynomial terms added (order = {})", order)
            }
            PreProcessing::ReplaceWithPCA {
                number_of_components,
            } => write!(
                f,
                "Replaced with PCA features (n = {})",
                number_of_components
            ),

            PreProcessing::ReplaceWithSVD {
                number_of_components,
            } => write!(
                f,
                "Replaced with SVD features (n = {})",
                number_of_components
            ),
        }
    }
}

/// Final model approach
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FinalModel {
    /// Do not train a final model
    None,
    /// Select the best model from the comparison set as the final model
    Best,
    /// Use a blending approach to produce a final model
    Blending {
        /// Which algorithm to use as a meta-learner
        algorithm: Algorithm,
        /// How much data to retain to train the blending model
        meta_training_fraction: f32,
        /// How much data to retain to test the blending model
        meta_testing_fraction: f32,
    },
    // /// Use a stacking approach to produce a final model (not implemented)
    // Stacking {
    //     /// How much data to retain to train the blending model
    //     meta_testing_fraction: f32,
    // },
}

impl FinalModel {
    /// Default values for a blending model (linear regression, 30% of all data reserved for training the blending model)
    pub fn default_blending() -> FinalModel {
        Self::Blending {
            algorithm: Algorithm::Linear,
            meta_training_fraction: 0.15,
            meta_testing_fraction: 0.15,
        }
    }
}
