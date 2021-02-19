import connector_agent as ca


if __name__ == "__main__":
    # only support uint64 type here
    df = ca.write_pandas([4, 7], ["uint64", "uint64", "uint64"])
    print(df)

    df = ca.write_pandas([1000000, 2000000], ["uint64", "uint64", "uint64"])
    print(df)

    # does not support mixed type
    # df = ca.write_pandas([4, 7], ["uint64", "float64",
    #                              "object", "float64", "bool", "uint64"])
    # print(df)
