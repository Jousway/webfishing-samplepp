# Main Rust Project.

# ("${Username}.${ProjectName}.data")

configure_file("Cargo.in.toml" "Cargo.toml")
add_custom_command(
    POST_BUILD
    COMMAND cargo build --manifest-path "${PROJECT_BINARY_DIR}/Source/MainRust/Cargo.toml" --target-dir "${MainDir}/Output/"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
	OUTPUT  "Out.out"
	COMMENT "Building MainRust Project."
)

add_custom_target(
    "${Username}.${ProjectName}.data" ALL
	DEPENDS Rust
    "Out.out"
)