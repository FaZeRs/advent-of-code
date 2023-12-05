#include <fstream>
#include <regex>
#include <sstream>
#include <unordered_map>

std::unordered_map<std::string, uint32_t> word_map{
    {"one", 1}, {"two", 2},   {"three", 3}, {"four", 4}, {"five", 5},
    {"six", 6}, {"seven", 7}, {"eight", 8}, {"nine", 9}};

uint32_t get_word_value(const std::string &word) {
  auto it = word_map.find(word);
  if (it == word_map.end()) {
    return -1;
  }
  return it->second;
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    printf("Usage: %s <input file>\n", argv[0]);
    return 1;
  }

  std::ifstream file(argv[1]);
  if (!file.is_open()) {
    printf("Unable to open file: %s\n", argv[1]);
    return 1;
  }

  std::regex pattern("(?=(one|two|three|four|five|six|seven|eight|nine))|\\d");
  uint32_t sum = 0;
  std::string line;
  while (std::getline(file, line)) {
    printf("Line: %s\n", line.c_str());
    uint32_t left_most = -1;
    uint32_t right_most = -1;
    std::sregex_iterator begin(line.begin(), line.end(), pattern);
    std::sregex_iterator end;
    for (std::sregex_iterator it = begin; it != end; ++it) {
      std::smatch match = *it;
      if (match[1].matched) {
        std::string match_str = match.str(1);
        uint32_t value = get_word_value(match_str);
        if (left_most == -1) {
          left_most = value;
          right_most = value;
        } else {
          right_most = value;
        }
      } else {
        std::string match_str = match.str();
        uint32_t value = std::stoi(match_str);
        if (left_most == -1) {
          left_most = value;
          right_most = value;
        } else {
          right_most = value;
        }
      }
    }
    printf("Left: %d, Right: %d, Sum: %d\n", left_most, right_most,
           left_most * 10 + right_most);
    if (left_most != -1 && right_most != -1) {
      sum += left_most * 10 + right_most;
    }
  }

  file.close();
  printf("Sum: %d\n", sum);

  return 0;
}
