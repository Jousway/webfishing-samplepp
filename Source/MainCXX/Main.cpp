#include "CompatLoad.hpp"
#include "Main.hpp"

using namespace ProjectUser::ProjectName;

Main::Main()
{
	Conf = ImodInterface::ReadConfig<Config^>();

	Logger::Information("Hello World!");
}

Main::~Main()
{
	// Delete Stuff :)
}
