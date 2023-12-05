#include <fstream>
#include <iostream>
#include <regex>
#include <string_view>
#include <vector>

struct EnginePart {
  uint32_t number;
  uint32_t x;
  uint32_t y;
  bool is_valid{false};
  uint32_t mult_x;
  uint32_t mult_y;
  bool has_mult{false};
};

int main(int argc, char* argv[]) {
  if (argc != 2) {
    printf("Usage: %s <input file>\n", argv[0]);
    return 1;
  }

  std::ifstream file(argv[1]);
  if (!file.is_open()) {
    printf("Unable to open file: %s\n", argv[1]);
    return 1;
  }

  std::vector<std::string> lines;
  std::string line;
  while (std::getline(file, line)) {
    lines.emplace_back(line);
  }

  file.close();

  std::regex number_pattern("\\d+");
  std::vector<EnginePart> parts;
  for (size_t i = 0; i < lines.size(); ++i) {
    std::string line_view(lines[i]);
    printf("Line: %s\n", line_view.c_str());

    std::sregex_iterator begin(line_view.begin(), line_view.end(),
                               number_pattern);
    std::sregex_iterator end;
    for (std::sregex_iterator it = begin; it != end; ++it) {
      std::smatch match = *it;
      EnginePart part;
      part.number = std::stoi(match.str());
      part.x = match.position();
      part.y = i;
      parts.emplace_back(part);
      printf("Match: %s, position: %ld\n", match.str().c_str(),
             match.position());
    }
  }

  // Validate parts
  for (size_t i = 0; i < parts.size(); ++i) {
    EnginePart& part = parts[i];
    printf("Part: %d, %d, %d\n", part.number, part.x, part.y);

    const int length = std::to_string(part.number).length();
    // Determine if part has symbol adjacent to it.
    // Check left.
    if (part.x > 0) {
      uint32_t left_pos = part.x - 1;
      if (lines[part.y][left_pos] != '.' &&
          !std::isdigit(lines[part.y][left_pos])) {
        printf("Left: %c\n", lines[part.y][left_pos]);
        part.is_valid = true;

        // Check if part has multiplier.
        if (lines[part.y][left_pos] == '*') {
          part.has_mult = true;
          part.mult_x = left_pos;
          part.mult_y = part.y;
        }
      }
    }
    // Check right
    uint32_t right_pos = length + part.x;
    if (part.x < lines[part.y].size() - 1 &&
        right_pos < lines[part.y].size() - 1) {
      if (lines[part.y][right_pos] != '.' &&
          !std::isdigit(lines[part.y][right_pos])) {
        printf("Right: %c\n", lines[part.y][right_pos]);
        part.is_valid = true;

        // Check if part has multiplier.
        if (lines[part.y][right_pos] == '*') {
          part.has_mult = true;
          part.mult_x = right_pos;
          part.mult_y = part.y;
        }
      }
    }
    // Check up
    if (part.y > 0) {
      uint32_t up_pos = part.y - 1;
      for (uint32_t i = 0; i < length + 2; ++i) {
        uint32_t pos = part.x - 1 + i;
        if (pos >= 0 && pos < lines[up_pos].size() - 1 &&
            lines[up_pos][pos] != '.' && !std::isdigit(lines[up_pos][pos])) {
          printf("Up: %c\n", lines[up_pos][pos]);
          part.is_valid = true;

          // Check if part has multiplier.
          if (lines[up_pos][pos] == '*') {
            part.has_mult = true;
            part.mult_x = pos;
            part.mult_y = up_pos;
          }
        }
      }
    }

    // Check down
    if (part.y < lines.size() - 1) {
      uint32_t down_pos = part.y + 1;
      for (uint32_t i = 0; i < length + 2; ++i) {
        uint32_t pos = part.x - 1 + i;
        if (pos >= 0 && pos < lines[down_pos].size() - 1 &&
            lines[down_pos][pos] != '.' &&
            !std::isdigit(lines[down_pos][pos])) {
          printf("Down: %c\n", lines[down_pos][pos]);
          part.is_valid = true;

          // Check if part has multiplier.
          if (lines[down_pos][pos] == '*') {
            part.has_mult = true;
            part.mult_x = pos;
            part.mult_y = down_pos;
          }
        }
      }
    }
  }

  uint32_t sum = 0;
  for (const auto& part : parts) {
    if (!part.is_valid) {
      continue;
    }
    printf("Valid part: %d\n", part.number);
    sum += part.number;
  }

  printf("Sum: %d\n", sum);

  uint32_t sum_part2 = 0;

  for (size_t i = 0; i < parts.size(); ++i) {
    EnginePart& part = parts[i];
    if (!part.is_valid || !part.has_mult) {
      continue;
    }
    for (size_t j = i; j < parts.size(); ++j) {
      EnginePart& other = parts[j];
      if (std::addressof(part) == std::addressof(other)) {
        continue;
      }
      if (!other.is_valid || !other.has_mult) {
        continue;
      }
      if (part.mult_x == other.mult_x && part.mult_y == other.mult_y) {
        printf("Found match: %d, %d\n", part.number, other.number);
        sum_part2 += part.number * other.number;
      }
    }
  }

  printf("Sum part 2: %d\n", sum_part2);

  return 0;
}
