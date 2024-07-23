## Description
This is an implementation of a data engineering pipeline based on multiple stages to extract, transform, and analyze various datasets.

## Stages
The pipeline is divided into the folowing stages:
- Input, where data is read from a source as a table (DataFrame) structure with values as strings
- Parse, the strings are converted to numeric data using a parser indicated in the configuration file
- Scrub, handles missing data from the input dataset by excising or replacing malformed samples
- Transform, performs transformations on the attributes such as z-score standardization or logarithmic transforms
- Training and validation, the data is partitioned into training and testing datasets

## Functions
Entitites are defined in the configuration file to use the output of the "Training and validation" stage. 
The most important entity is the Model, which is the ML model to be trained and tested.
An evaluator is the evaluation stategy to be used to measure the performance of the Model.
A Parser is the entity that will generate training and testing datasets from the input table.

## Building
This project is built in Rust, all you need is to install the compiler as directed on the main website for the language.

## Configuration
A YAML file is used to provide the pipeline with the neccessary configurations. Examples are provided in the "configs" directory
for the different datasets in the "dataset" directory.
