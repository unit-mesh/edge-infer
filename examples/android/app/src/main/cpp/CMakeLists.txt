# 设置CMake的最低版本要求
cmake_minimum_required(VERSION 3.6)

# 设置项目名称
project(PocProject)

# 指定要构建的目标的源文件
add_library(PlaceHolder SHARED native-lib.cpp)

# 指定要链接的C++标准库
target_link_libraries(PlaceHolder c++_shared)

# 设置C++标准
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
