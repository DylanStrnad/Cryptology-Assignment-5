import time

def multiplicaiton_performance_256_bit_operand():
    bigint1 = "ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"
    bigint2 = "ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"
    bigint1 = int(bigint1,16)
    bigint2 = int(bigint2, 16)
    total_time = 0
    for _i in range(30):
        start = time.perf_counter()
        result = bigint1 * bigint2
        end = time.perf_counter()
        execution_time = (end - start) * 1000000
        total_time += execution_time
    avg_execution_time = total_time/30
    my_rust_execution_time = 5.049
    print("test 256-bit performance")
    if (avg_execution_time <= my_rust_execution_time):
        print("Python faster - execution time: ")
        print(avg_execution_time, "µs")
        print("My Algorithms execution time: ")
        print(my_rust_execution_time, "µs")
    else:
        print("Python slower - execution time: ")
        print(avg_execution_time, "µs")
        print("My Algorithms execution time: ")
        print(my_rust_execution_time, "µs")

def multiplicaiton_performance_512_bit_operand():
    bigint1 = "ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"
    bigint2 = "ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"
    bigint1 = int(bigint1,16)
    bigint2 = int(bigint2, 16)
    total_time = 0
    for _i in range(30):
        start = time.perf_counter()
        result = bigint1 * bigint2
        end = time.perf_counter()
        execution_time = (end - start) * 1000000
        total_time += execution_time
    avg_execution_time = total_time/30
    my_rust_execution_time = 14.101
    print("test 512-bit performance")
    if (avg_execution_time <= my_rust_execution_time):
        print("Python faster - execution time: ")
        print(avg_execution_time, "µs")
        print("My Algorithms execution time: ")
        print(my_rust_execution_time, "µs")
    else:
        print("Python slower - execution time: ")
        print(avg_execution_time, "µs")
        print("My Algorithms execution time: ")
        print(my_rust_execution_time, "µs")

def multiplicaiton_performance_1024_bit_operand():
    bigint1 = "ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"
    bigint2 = "ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2ab54a98ceb1f0ad2"
    bigint1 = int(bigint1,16)
    bigint2 = int(bigint2, 16)
    total_time = 0
    for _i in range(30):
        start = time.perf_counter()
        result = bigint1 * bigint2
        end = time.perf_counter()
        execution_time = (end - start) * 1000000
        total_time += execution_time
    avg_execution_time = total_time/30
    my_rust_execution_time = 39.552
    print("test 1024-bit performance")
    if (avg_execution_time <= my_rust_execution_time):
        print("Python faster - execution time: ")
        print(avg_execution_time, "µs")
        print("My Algorithms execution time: ")
        print(my_rust_execution_time, "µs")
    else:
        print("Python slower - execution time: ")
        print(avg_execution_time, "µs")
        print("My Algorithms execution time: ")
        print(my_rust_execution_time, "µs")

multiplicaiton_performance_256_bit_operand()
multiplicaiton_performance_512_bit_operand()
multiplicaiton_performance_1024_bit_operand()