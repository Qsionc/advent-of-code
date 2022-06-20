cmake_minimum_required( VERSION 3.16 )

set( CMAKE_CXX_STANDARD 20 )
project( AdventOfCode VERSION 0.21 LANGUAGES CXX )

# Day 1
add_executable( day1_part1 source/day_1/part_1.cpp )
add_executable( day1_part2 source/day_1/part_2.cpp )