#include <fstream>
#include <iostream>
#include <ranges>
#include <string_view>
#include <vector>

struct Game {
  struct Set {
    uint32_t red{0};
    uint32_t green{0};
    uint32_t blue{0};
  };
  uint32_t id;
  std::vector<Set> sets;
};

uint32_t extractNumber(const std::string_view &input) {
  size_t pos = input.find_first_of("123456789");
  if (pos != std::string_view::npos) {
    return std::stoi(input.substr(pos).data());
  }

  return -1;
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

  auto trimString = std::views::drop_while(isspace) | std::views::reverse |
                    std::views::drop_while(isspace) | std::views::reverse;

  std::string game_delim{": "};
  std::string sets_delim{"; "};
  std::string set_delim{", "};
  uint32_t max_red = 12;
  uint32_t max_green = 13;
  uint32_t max_blue = 14;
  uint32_t sum_id = 0;
  uint32_t sum_cube = 0;
  std::string line;
  while (std::getline(file, line)) {
    auto game_list =
        line | std::views::split(game_delim) |
        std::ranges::views::transform([](auto &&rng) {
          return std::string_view(&*rng.begin(), std::ranges::distance(rng));
        }) |
        std::ranges::to<std::vector>();

    const auto game_id = extractNumber(game_list.front());
    if (game_id == -1) {
      printf("Unable to extract game number from line: %s\n", line.c_str());
      continue;
    }
    Game game;
    game.id = game_id;

    auto sets =
        game_list.back() | std::views::split(sets_delim) |
        std::ranges::views::transform([](auto &&rng) {
          return std::string_view(&*rng.begin(), std::ranges::distance(rng));
        }) |
        std::ranges::to<std::vector>();

    for (const auto &set : sets) {
      auto set_list =
          set | std::views::split(set_delim) |
          std::ranges::views::transform([](auto &&rng) {
            return std::string_view(&*rng.begin(), std::ranges::distance(rng));
          }) |
          std::ranges::to<std::vector>();
      Game::Set game_set;
      for (const auto &item : set_list) {
        if (item.ends_with("red")) {
          game_set.red = extractNumber(item);
        } else if (item.ends_with("green")) {
          game_set.green = extractNumber(item);
        } else if (item.ends_with("blue")) {
          game_set.blue = extractNumber(item);
        }
      }
      game.sets.emplace_back(game_set);
    }

    uint32_t min_red = 0;
    uint32_t min_green = 0;
    uint32_t min_blue = 0;
    bool is_valid_game = true;
    for (const auto &set : game.sets) {
      if (set.red > max_red || set.green > max_green || set.blue > max_blue) {
        is_valid_game = false;
      }
      if (set.red > min_red) {
        min_red = set.red;
      }
      if (set.green > min_green) {
        min_green = set.green;
      }
      if (set.blue > min_blue) {
        min_blue = set.blue;
      }
    }
    printf("Game: %d, Valid: %s, Min Red: %d, Min Green: %d, Min Blue: %d\n",
           game.id, is_valid_game ? "true" : "false", min_red, min_green,
           min_blue);

    if (is_valid_game) {
      sum_id += game.id;
    }
    sum_cube += min_red * min_green * min_blue;
  }

  file.close();
  printf("Sum IDs: %d\n", sum_id);
  printf("Sum cubes: %d\n", sum_cube);

  return 0;
}
