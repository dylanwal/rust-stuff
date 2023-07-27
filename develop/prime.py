
import time


def is_prime(n: int) -> bool:
    if n <= 1:
        return False
    for a in range(2, n):
        if n % a == 0:
            return False

    return True


def main():
    start = time.perf_counter()
    for i in range(200_000):
        x = is_prime(i)

    stop = time.perf_counter()
    print(stop-start)
    print(x)


if __name__ == "__main__":
    main()
