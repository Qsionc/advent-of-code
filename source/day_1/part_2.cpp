/**
 * @author Bartosz Dec (bartoszjandec@gmail.com)
 * @date 20.06.22
 * @file part_2.cpp
 * @brief
 * @license
 */

#include <iostream>
#include <tuple>
#include <iterator>
#include <algorithm>
#include <fstream>

using window_t = std::tuple<int, int, int>;

auto sum_window(window_t const& _window)
{
    return std::get<0>(_window) + std::get<1>(_window) + std::get<2>(_window);
}

int main()
{
    std::ifstream file{ "../resources/input_day1.txt" };
    auto iterator = std::istream_iterator<int>(file);
    auto prev_window = window_t{ (*iterator++), (*iterator++), (*iterator++) };
    auto result = 0;

    std::for_each(iterator, {},
                  [&result, &prev_window](auto k)
                  {
                      auto tmp = window_t{ std::get<1>(prev_window), std::get<2>(prev_window), k };
                      if (sum_window(tmp) > sum_window(prev_window))
                      {
                          result++;
                      }
                      prev_window = tmp;
                  });

    std::cout << result << std::endl;

    return 0;
}