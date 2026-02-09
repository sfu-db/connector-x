DROP TABLE IF EXISTS test_basic_types;
DROP TABLE IF EXISTS test_string_types;
DROP TABLE IF EXISTS test_datetime_types;
DROP TABLE IF EXISTS test_enum_types;
DROP TABLE IF EXISTS test_network_types;
DROP TABLE IF EXISTS test_array_types;
DROP TABLE IF EXISTS test_nullable_types;


CREATE TABLE test_basic_types (
    id UInt32,
    col_int8 Int8,
    col_int16 Int16,
    col_int32 Int32,
    col_int64 Int64,
    col_uint8 UInt8,
    col_uint16 UInt16,
    col_uint32 UInt32,
    col_uint64 UInt64,
    col_float32 Float32,
    col_float64 Float64,
    col_decimal32 Decimal32(4),
    col_decimal64 Decimal64(8),
    col_bool Bool
) ENGINE = MergeTree()
ORDER BY id;

INSERT INTO test_basic_types VALUES
    (1, -128, -32768, -2147483648, -9223372036854775808, 0, 0, 0, 0, -3.14, -3.141592653589793, -12345.6789, -123456789.12345678, true),
    (2, 127, 32767, 2147483647, 9223372036854775807, 255, 65535, 4294967295, 18446744073709551615, 3.14, 3.141592653589793, 12345.6789, 123456789.12345678, false),
    (3, 0, 0, 0, 0, 128, 32768, 2147483648, 9223372036854775808, 0.0, 0.0, 0.0, 0.0, true),
    (4, 42, 1000, 100000, 1000000000, 42, 1000, 100000, 1000000000, 2.718, 2.718281828459045, 9999.9999, 99999999.99999999, false),
    (5, -1, -1, -1, -1, 1, 1, 1, 1, -0.5, -0.123456789012345, -0.0001, -0.00000001, true);


CREATE TABLE test_string_types (
    id UInt32,
    col_string String,
    col_fixedstring FixedString(16)
) ENGINE = MergeTree()
ORDER BY id;

INSERT INTO test_string_types VALUES
    (1, 'Hello, World!', 'FixedStr16bytes!'),
    (2, 'ConnectorX ClickHouse Test', 'ABCDEFGHIJKLMNOP');


CREATE TABLE test_datetime_types (
    id UInt32,
    col_time Time,
    col_time64 Time64(3),
    col_date Date,
    col_date32 Date32,
    col_datetime DateTime,
    col_datetime64 DateTime64(3)
) ENGINE = MergeTree()
ORDER BY id;

INSERT INTO test_datetime_types VALUES
    (1, '14:30:25', '14:30:25.123', '2024-01-15', '2024-01-15', '2024-01-15 10:30:45', '2024-01-15 10:30:45.123'),
    (2, '14:35:25', '14:35:25.123', '1970-01-01', '1900-01-01', '1970-01-01 00:00:00', '1970-01-01 00:00:00.000'),
    (3, '23:59:59', '23:59:59.999', '2099-12-31', '2099-12-31', '2099-12-31 23:59:59', '2099-12-31 23:59:59.999'),
    (4, '12:00:00', '12:00:00.500', '2000-06-15', '2000-06-15', '2000-06-15 12:00:00', '2000-06-15 12:00:00.500'),
    (5, '08:15:30', '08:15:30.750', '2023-11-28', '2023-11-28', '2023-11-28 08:15:30', '2023-11-28 08:15:30.750');


CREATE TABLE test_enum_types (
    id UInt32,
    col_enum8 Enum8('small' = 1, 'medium' = 2, 'large' = 3),
    col_enum16 Enum16('red' = 100, 'green' = 200, 'blue' = 300, 'yellow' = 400)
) ENGINE = MergeTree()
ORDER BY id;

INSERT INTO test_enum_types VALUES
    (1, 'small', 'red'),
    (2, 'medium', 'green'),
    (3, 'large', 'blue'),
    (4, 'small', 'yellow'),
    (5, 'large', 'red');


CREATE TABLE test_network_types (
    id UInt32,
    col_uuid UUID,
    col_ipv4 IPv4,
    col_ipv6 IPv6
) ENGINE = MergeTree()
ORDER BY id;

INSERT INTO test_network_types VALUES
    (1, '550e8400-e29b-41d4-a716-446655440000', '192.168.1.1', '2001:0db8:85a3:0000:0000:8a2e:0370:7334'),
    (2, '6ba7b810-9dad-11d1-80b4-00c04fd430c8', '10.0.0.1', 'fe80::1'),
    (3, '00000000-0000-0000-0000-000000000000', '0.0.0.0', '::'),
    (4, 'ffffffff-ffff-ffff-ffff-ffffffffffff', '255.255.255.255', 'ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff'),
    (5, '123e4567-e89b-12d3-a456-426614174000', '127.0.0.1', '::1');


CREATE TABLE test_array_types (
    id UInt32,
    col_array_bool Array(Bool),
    col_array_string Array(String),
    col_array_int8 Array(Int8),
    col_array_int16 Array(Int16),
    col_array_int32 Array(Int32),
    col_array_int64 Array(Int64),
    col_array_uint8 Array(UInt8),
    col_array_uint16 Array(UInt16),
    col_array_uint32 Array(UInt32),
    col_array_uint64 Array(UInt64),
    col_array_float32 Array(Float32),
    col_array_float64 Array(Float64)
) ENGINE = MergeTree()
ORDER BY id;

INSERT INTO test_array_types VALUES
    (1, [true, false, true], ['a', 'b', 'c'], [-1, 0, 1], [-100, 0, 100], [-1000, 0, 1000], [-10000, 0, 10000], [0, 128, 255], [0, 32768, 65535], [0, 1000000, 4294967295], [0, 1000000, 18446744073709551615], [1.1, 2.2, 3.3], [1.111, 2.222, 3.333]),
    (2, [], [], [], [], [], [], [], [], [], [], [], []),
    (3, [false], ['single'], [42], [42], [42], [42], [42], [42], [42], [42], [42.0], [42.0]),
    (4, [true, true, true, true, true], ['one', 'two', 'three', 'four', 'five'], [-128, -64, 0, 64, 127], [-32768, -16384, 0, 16384, 32767], [-2147483648, -1073741824, 0, 1073741824, 2147483647], [-9223372036854775808, -4611686018427387904, 0, 4611686018427387904, 9223372036854775807], [0, 64, 128, 192, 255], [0, 16384, 32768, 49152, 65535], [0, 1073741824, 2147483648, 3221225472, 4294967295], [0, 4611686018427387904, 9223372036854775808, 13835058055282163712, 18446744073709551615], [-3.14, -1.57, 0.0, 1.57, 3.14], [-3.141592653589793, -1.5707963267948966, 0.0, 1.5707963267948966, 3.141592653589793]),
    (5, [false, true], ['hello', 'world'], [10, 20], [100, 200], [1000, 2000], [10000, 20000], [10, 20], [100, 200], [1000, 2000], [10000, 20000], [1.5, 2.5], [1.55, 2.55]);


CREATE TABLE test_nullable_types (
    id UInt32,
    col_int8 Nullable(Int8),
    col_int16 Nullable(Int16),
    col_int32 Nullable(Int32),
    col_int64 Nullable(Int64),
    col_uint8 Nullable(UInt8),
    col_uint16 Nullable(UInt16),
    col_uint32 Nullable(UInt32),
    col_uint64 Nullable(UInt64),
    col_float32 Nullable(Float32),
    col_float64 Nullable(Float64),
    col_decimal32 Nullable(Decimal32(4)),
    col_decimal64 Nullable(Decimal64(8)),
    col_string Nullable(String),
    col_fixedstring Nullable(FixedString(8)),
    col_date Nullable(Date),
    col_date32 Nullable(Date32),
    col_datetime Nullable(DateTime),
    col_datetime64 Nullable(DateTime64(3)),
    col_uuid Nullable(UUID),
    col_ipv4 Nullable(IPv4),
    col_ipv6 Nullable(IPv6),
    col_bool Nullable(Bool)
) ENGINE = MergeTree()
ORDER BY id;

INSERT INTO test_nullable_types VALUES
    (1, -10, -1000, -100000, -10000000, 10, 1000, 100000, 10000000, 1.5, 1.555, 12.3456, 12.34567890, 'not null', 'notnull!', '2024-01-01', '2024-01-01', '2024-01-01 12:00:00', '2024-01-01 12:00:00.123', '550e8400-e29b-41d4-a716-446655440000', '192.168.1.1', '2001:db8::1', true),
    (2, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
    (3, 50, 5000, 500000, 50000000, 50, 5000, 500000, 50000000, 2.5, 2.555, 56.7890, 56.78901234, 'value', 'val12345', '2023-06-15', '2023-06-15', '2023-06-15 18:30:00', '2023-06-15 18:30:00.456', '6ba7b810-9dad-11d1-80b4-00c04fd430c8', '10.0.0.1', 'fe80::1', false),
    (4, NULL, 100, NULL, 1000000, NULL, 100, NULL, 1000000, NULL, 3.333, NULL, 33.33333333, NULL, 'mixed!!!', NULL, '2000-01-01', NULL, '2000-01-01 00:00:00.000', NULL, '127.0.0.1', NULL, true),
    (5, 0, NULL, 0, NULL, 0, NULL, 0, NULL, 0.0, NULL, 0.0000, NULL, '', NULL, '1970-01-01', NULL, '1970-01-01 00:00:00', NULL, '00000000-0000-0000-0000-000000000000', NULL, '::', NULL);
