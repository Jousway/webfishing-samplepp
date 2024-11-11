# CSharp Wrapper
project("${Username}.${ProjectName}" VERSION ${Version} LANGUAGES CSharp)

# CS Wrapper
configure_file("Wrapper.in.cs" "${PROJECT_BINARY_DIR}/Wrapper.cs")
add_library("${Username}.${ProjectName}" SHARED "${PROJECT_BINARY_DIR}/Wrapper.cs")
set_target_properties("${Username}.${ProjectName}" PROPERTIES
DOTNET_SDK "Microsoft.NET.Sdk"
DOTNET_TARGET_FRAMEWORK "net8.0"
VS_PACKAGE_REFERENCES "System;System.Runtime"
VS_DOTNET_REFERENCES_COPY_LOCAL OFF
VS_DOTNET_REFERENCE_GDWeave "${MainDir}/core/GDWeave.dll"
VS_DOTNET_REFERENCE_Serilog "${MainDir}/core/Serilog.dll"
)
target_include_directories("${Username}.${ProjectName}" PRIVATE ${PROJECT_BINARY_DIR})
