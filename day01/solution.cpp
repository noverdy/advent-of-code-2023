#include <fstream>
#include <iostream>
#include <string>

int get_calibration_value(std::string s) {
    int first_number = -1, last_number = -1;
    for (int i = 0; i < s.length(); i++) {
        if (isdigit(s[i])) {
            if (first_number == -1) {
                first_number = s[i] - '0';
            }
            last_number = s[i] - '0';
        }
    }
    return first_number * 10 + last_number;
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
        return 1;
    }

    int answer = 0;
    std::ifstream file(argv[1]);
    while (file) {
        std::string line;
        std::getline(file, line);
        if (line.length() == 0) {
            continue;
        }
        answer += get_calibration_value(line);
    }
    file.close();

    std::cout << answer << std::endl;
}