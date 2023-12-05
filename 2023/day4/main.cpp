#include <charconv>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <ranges>
#include <sstream>
#include <vector>

struct Card {
  uint32_t id;
  std::vector<uint32_t> numbers;
  std::vector<uint32_t> winning_numbers;
};

std::vector<uint32_t> extractNumbers(const std::string& input) {
  std::vector<uint32_t> numbers;
  std::stringstream ss(input);

  uint32_t number;
  while (ss >> number) {
    numbers.push_back(number);
    // Skip non-digits
    ss.ignore(std::numeric_limits<std::streamsize>::max(), ' ');
  }

  return numbers;
}

uint32_t extractNumber(const std::string_view& input) {
  size_t pos = input.find_first_of("123456789");
  if (pos != std::string_view::npos) {
    return std::stoi(input.substr(pos).data());
  }

  return -1;
}

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

  std::string card_delim{":"};
  std::string numbers_delim{" | "};
  std::string number_delim{" "};

  std::vector<Card> cards_vec;
  std::string line;
  while (std::getline(file, line)) {
    printf("Line: %s\n", line.c_str());
    auto card_list =
        line | std::views::split(card_delim) |
        std::ranges::views::transform([](auto&& rng) {
          return std::string_view(&*rng.begin(), std::ranges::distance(rng));
        }) |
        std::ranges::to<std::vector>();

    std::cout << "Card: " << card_list.front() << std::endl;
    std::cout << "Numbers: " << card_list.back() << std::endl;

    auto numbers_list =
        card_list.back() | std::views::split(numbers_delim) |
        std::ranges::views::transform([](auto&& rng) {
          return std::string_view(&*rng.begin(), std::ranges::distance(rng));
        }) |
        std::ranges::to<std::vector>();

    std::cout << "Winning numbers: " << numbers_list.front() << std::endl;
    std::cout << "Numbers: " << numbers_list.back() << std::endl;

    auto winning_numbers = extractNumbers(std::string(numbers_list.front()));
    auto card_numbers = extractNumbers(std::string(numbers_list.back()));

    Card card;
    card.id = extractNumber(card_list.front());
    card.winning_numbers = winning_numbers;
    card.numbers = card_numbers;
    cards_vec.emplace_back(card);
  }

  uint32_t sum = 0;

  std::map<uint32_t, std::vector<Card>> cards_map;
  for (const auto& card : cards_vec) {
    cards_map[card.id].emplace_back(card);
  }

  for (auto it = cards_map.begin(); it != cards_map.end(); ++it) {
    const auto& [id, cards] = *it;
    for (size_t i = 0; i < cards.size(); ++i) {
      const auto& card = cards[i];
      uint32_t card_sum = 0;
      uint32_t wins = 0;
      for (size_t j = 0; j < card.numbers.size(); ++j) {
        const auto& number = card.numbers[j];
        if (std::ranges::find(card.winning_numbers, number) !=
            card.winning_numbers.end()) {
          const uint32_t points = wins == 0 ? 1 : std::pow(2, wins - 1);
          card_sum += points;
          wins++;
        }
      }
      sum += card_sum;

      if (wins > 0) {
        const uint32_t start_idx = card.id;
        const uint32_t end_idx = card.id + wins;
        for (uint32_t i = start_idx; i < end_idx; i++) {
          if (i > cards_vec.size() - 1) {
            break;
          }
          const auto& other_card = cards_vec[i];
          cards_map[i].insert(cards_map[i].end(), other_card);
        }
      }
    }
  }

  uint32_t sum2 = 0;
  for (const auto& [id, cards] : cards_map) {
    for (const auto& card : cards) {
      sum2++;
    }
  }

  file.close();
  printf("Sum: %d\n", sum);
  printf("Sum2: %d\n", sum2);

  return 0;
}
