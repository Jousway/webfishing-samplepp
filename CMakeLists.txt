cmake_minimum_required(VERSION 3.17)

#Change These
set(Username "Name")
set(ProjectName "Sample")
set(Version 1.0.0)

# let me handle the rest :)
project ("${Username}.${ProjectName}.Main" VERSION ${Version} LANGUAGES CXX)

set(MainDir ${PROJECT_SOURCE_DIR})
set(BuildDir ${PROJECT_BINARY_DIR})

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG "${MainDir}/Output/Debug/")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE "${MainDir}/Output/")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${MainDir}/Output/Other/")

add_subdirectory(Source)
