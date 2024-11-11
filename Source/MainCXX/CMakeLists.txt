# Main CPP Project.
project("${Username}.${ProjectName}.data" VERSION ${Version} LANGUAGES CXX)

set(Main_Files 
	"CompatLoad.hpp"
	"Main.hpp"
	"Main.cpp"
)

source_group("" FILES ${Main_Files})

list(APPEND Included_Files
	${Main_Files}
)

add_library("${Username}.${ProjectName}.data" SHARED ${Included_Files})
set_target_properties("${Username}.${ProjectName}.data" PROPERTIES
CXX_EXTENSIONS ON
CXX_STANDARD 17
COMMON_LANGUAGE_RUNTIME "netcore"
DOTNET_TARGET_FRAMEWORK "net8.0"
DOTNET_TARGET_FRAMEWORK_VERSION "8.0"
VS_GLOBAL_CLRSupport "true"
VS_GLOBAL_Nullable "enable"
VS_DOTNET_REFERENCE ""
VS_DOTNET_REFERENCES_COPY_LOCAL OFF
VS_PLATFORM_TOOLSET "v143"
)

configure_file("Config.in.hpp" "${PROJECT_BINARY_DIR}/config.hpp")
target_include_directories("${Username}.${ProjectName}.data" PRIVATE ${PROJECT_BINARY_DIR})