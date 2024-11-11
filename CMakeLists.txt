cmake_minimum_required(VERSION 3.17)

#Change These
set(Username "Name")
set(ProjectName "Sample")
set(Version 1.0.0)

# let me handle the rest :)
project ("${Username}.${ProjectName}.Main" VERSION ${Version} LANGUAGES CXX)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_DEBUG "${PROJECT_SOURCE_DIR}/Output/")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELEASE "${PROJECT_SOURCE_DIR}/Output/")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}/Output/")

set(MainDir ${PROJECT_SOURCE_DIR})
set(BuildDir ${PROJECT_BINARY_DIR})

add_subdirectory(Source)

# Use ALL_BUILD instead
# set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT ${PROJECT_NAME})