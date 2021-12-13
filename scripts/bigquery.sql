DROP TABLE IF EXISTS `dataprep-bigquery.dataprep.test_table`;

CREATE TABLE`dataprep-bigquery.dataprep.test_table`(
    test_int INT64,
    test_string STRING,
    test_float FLOAT64
);

INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (1, 'str1', 1.1);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (2, 'str2', 2.2);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (2333, NULL, NULL);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (4, NULL, -4.44);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (5, 'str05', NULL);