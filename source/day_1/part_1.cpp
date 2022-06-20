/**
 * @author Bartosz Dec (bartoszjandec@gmail.com)
 * @date 20.06.22
 * @file part_1.cpp
 * @brief
 * @license
 */

#include <iostream>
#include <algorithm>
#include <fstream>
#include <iterator>

int main()
{
    auto value = -1;
    std::ifstream file{ "../resources/input_day1.txt" };
    auto result = std::count_if(std::istream_iterator<int>(file), {},
                                [&value](auto _item)
                                {
                                    auto tmp = std::exchange(value, _item);
                                    if (tmp == -1 || value <= tmp)
                                    {
                                        return false;
                                    }
                                    return true;
                                });
    std::cout << result << std::endl;
}