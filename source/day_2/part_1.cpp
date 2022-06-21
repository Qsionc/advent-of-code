/**
 * @author Bartosz Dec (bartoszjandec@gmail.com)
 * @date 21.06.22
 * @file part_1.cpp
 * @brief
 * @license
 */

#include <iostream>
#include <iterator>
#include <fstream>
#include <algorithm>
#include <string>

enum class direction
{
    forward,
    up,
    down,
    unknown
};

using coordinate = std::pair<direction, int>;

direction get_direction(std::string const &_str)
{
    if (_str == "forward")
        return direction::forward;
    else if (_str == "up")
        return direction::up;
    else if (_str == "down")
        return direction::down;
    return direction::unknown;
}

struct line : std::string
{
    using std::string::substr;
    using std::string::find_first_of;

    friend std::istream &operator>>(std::istream &_is, line &_line)
    {
        return std::getline(_is, _line);
    }
};

coordinate get_coordinate(line const &_line)
{
    return std::make_pair(get_direction(_line.substr(0, _line.find_first_of(' '))),
                          std::stoi(_line.substr(_line.find_first_of(' '))));
}

int main()
{
    std::ifstream file{"resources/input_day2.txt"};
    std::istream_iterator<line> begin{file}, end;

    int depth = 0, horiz = 0;

    std::for_each(begin, end,
                  [&depth, &horiz](line const &_line)
                  {
                      auto coord = get_coordinate(_line);
                      switch (coord.first)
                      {
                      case direction::down:
                          depth += coord.second;
                          break;
                      case direction::up:
                          depth -= coord.second;
                          break;
                      case direction::forward:
                          horiz += coord.second;
                          break;
                      default:
                          break;
                      }
                  });

    std::cout << depth * horiz << std::endl;

    return 0;
}
