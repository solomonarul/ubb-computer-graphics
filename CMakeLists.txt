cmake_minimum_required(VERSION 3.20)

add_subdirectory(lib)

project(app)

file(
    GLOB_RECURSE
    ${PROJECT_NAME}_SRC
        "src/*.cpp"
)

add_executable(
    ${PROJECT_NAME}
    ${${PROJECT_NAME}_SRC}
)

if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4 /WX)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()

set_property(
    TARGET ${PROJECT_NAME}
    PROPERTY CXX_STANDARD 23
)

target_include_directories(
    ${PROJECT_NAME}
    PUBLIC
        "inc"
)

target_compile_definitions(
    ${PROJECT_NAME}
    PRIVATE
        BUILD_TYPE_${CMAKE_BUILD_PLATFORM}
)

find_package(OpenGL REQUIRED COMPONENTS OpenGL)

target_link_libraries(
    ${PROJECT_NAME}
    PUBLIC
        glfw
        OpenGL::OpenGL
)

install(TARGETS ${PROJECT_NAME})