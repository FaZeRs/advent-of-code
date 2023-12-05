#include <fstream>
#include <iostream>
#include <map>
#include <ranges>
#include <sstream>
#include <utility>
#include <vector>

std::vector<std::vector<std::string>> splitIntoChunks(
    const std::string& file_path) {
  std::ifstream file(file_path);
  if (!file.is_open()) {
    printf("Unable to open file: %s\n", file_path.c_str());
    return {};
  }
  std::vector<std::vector<std::string>> chunks;
  std::vector<std::string> current_chunk;
  std::string line;
  while (std::getline(file, line)) {
    if (line.empty()) {
      if (!current_chunk.empty()) {
        chunks.emplace_back(current_chunk);
        current_chunk.clear();
      }
    } else {
      current_chunk.emplace_back(line);
    }
  }
  file.close();

  if (!current_chunk.empty()) {
    chunks.push_back(current_chunk);
  }

  return chunks;
}

std::vector<uint32_t> extractNumbers(const std::string& input,
                                     bool skip_label = false) {
  std::vector<uint32_t> numbers;
  std::stringstream ss(input);

  if (skip_label) {
    std::string label;
    ss >> label;
  }

  uint32_t number;
  while (ss >> number) {
    numbers.push_back(number);
    // Skip non-digits
    ss.ignore(std::numeric_limits<std::streamsize>::max(), ' ');
  }

  return numbers;
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    printf("Usage: %s <input file>\n", argv[0]);
    return 1;
  }

  std::vector<std::vector<std::string>> chunks = splitIntoChunks(argv[1]);
  uint32_t lowest_seed = std::numeric_limits<uint32_t>::max();
  const auto& seeds = extractNumbers(chunks[0][0], true);
  for (size_t i = 0; i < seeds.size(); i += 2) {
    auto seed = seeds[i];
    auto range = seeds[i + 1];
// printf("Seed: %d\n", seed);
// printf("Range: %d\n", range);
#pragma omp parallel for num_threads(std::thread::hardware_concurrency())
    for (uint32_t j = seed; j < seed + range; ++j) {
      auto new_seed = j;
      for (size_t i = 1; i < chunks.size(); ++i) {
        for (size_t j = 1; j < chunks[i].size(); ++j) {
          const auto& chunk = chunks[i][j];
          // printf("Line: %s\n", chunk.c_str());

          const auto nums = extractNumbers(chunk);
          const auto& dest = nums[0];
          const auto& src = nums[1];
          const auto& range = nums[2];
          // printf("dest: %d, src: %d, range: %d\n", dest, src, range);
          if (new_seed >= src && new_seed <= src + range - 1) {
            new_seed += dest - src;
            // printf("New seed: %d\n", new_seed);
            break;
          }
        }
        // printf("\n");
      }

#pragma omp critical
      if (new_seed < lowest_seed) {
        lowest_seed = new_seed;
      }
    }

    // printf("\n\n");
  }

  printf("Lowest seed: %d\n", lowest_seed);

  return 0;
}
