import sys


def get_calibration_value(data: str) -> int:
    first_number = next(int(c) for c in data if c.isnumeric())
    last_number = next(int(c) for c in reversed(data) if c.isnumeric())
    return first_number*10 + last_number


def main():
    if len(sys.argv) != 2:
        print('Usage: python solution.py <input_file>')
        print('Example: python solution.py input.txt')
        sys.exit(1)

    answer = 0
    in_file = sys.argv[1]
    with open(in_file, 'r') as f:
        for line in f:
            answer += get_calibration_value(line)
    print(answer)


if __name__ == '__main__':
    main()
