DROP TABLE IF EXISTS `dataprep-bigquery.dataprep.test_table`;

CREATE TABLE`dataprep-bigquery.dataprep.test_table`(
    test_int INT64,
    test_string STRING,
    test_float FLOAT64,
    test_bool BOOL,
);

INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (1, 'str1', 1.1, TRUE);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (2, 'str2', 2.2, FALSE);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (2333, NULL, NULL, TRUE);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (4, NULL, -4.44, FALSE);
INSERT INTO `dataprep-bigquery.dataprep.test_table` VALUES (5, 'str05', NULL, NULL);


DROP TABLE IF EXISTS `dataprep-bigquery.dataprep.test_types`;

CREATE TABLE IF NOT EXISTS `dataprep-bigquery.dataprep.test_types`(
    test_int INTEGER,
    test_numeric NUMERIC(5, 2),
    test_bool BOOL,
    test_date DATE,
    test_time TIME,
    test_datetime DATETIME,
    test_timestamp TIMESTAMP,
    test_str STRING,
    test_bytes BYTES,
);

INSERT INTO `dataprep-bigquery.dataprep.test_types` VALUES (1, 1.23, TRUE, '1937-01-28', '00:00:00', NULL, '1970-01-01 00:00:01.00Z', '😁😂😜', CAST('😁😂😜' AS BYTES));
INSERT INTO `dataprep-bigquery.dataprep.test_types` VALUES (2, 234.56, NULL, '2053-07-25', '12:59:59', '2053-07-25 12:59:59', NULL, 'こんにちはЗдра́в', CAST('こんにちはЗдра́в' AS BYTES));
INSERT INTO `dataprep-bigquery.dataprep.test_types` VALUES (NULL, NULL, FALSE, NULL, NULL, '1937-01-28 00:00:00', '2004-2-29 12:00:01.30+3:00', NULL, NULL);
