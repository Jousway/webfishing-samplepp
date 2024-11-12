# Main Rust Project.

# ("${Username}.${ProjectName}.data")

configure_file("Cargo.in.toml" "Cargo.toml")
add_custom_command(
    POST_BUILD
    COMMAND cargo build --manifest-path "${PROJECT_BINARY_DIR}/Source/MainRust/Cargo.toml" --target-dir "${PROJECT_BINARY_DIR}/Source/MainRust/"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
	OUTPUT  "Out.out"
	COMMENT "Building MainRust Project."
	COMMAND ${CMAKE_COMMAND} -E copy_if_different "${PROJECT_BINARY_DIR}/Source/MainRust/$<$<CONFIG:Debug>:debug>$<$<CONFIG:Release>:release>/${Username}_${ProjectName}.dll" "${MainDir}/Output/$<$<CONFIG:Debug>:debug>$<$<CONFIG:Release>:release>/${Username}.${ProjectName}.data.dll"
)

add_custom_target(
    "${Username}.${ProjectName}.data" ALL
	DEPENDS Rust
    "Out.out"
)