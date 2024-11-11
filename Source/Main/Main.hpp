#ifndef MAIN_HPP
#define MAIN_HPP

using namespace System::Text::Json::Serialization;

ref class Main {
public:
	ref class Config {
	public:
		[JsonIncludeAttribute] bool Testbool = true;
		[JsonIncludeAttribute] int Testint = 1;
		[JsonIncludeAttribute] float Testfloat = 1.5;
	};

	Config^ Conf;

	Main();
	~Main();
};

#endif