#ifndef COMPATLAYER_HPP
#define COMPATLAYER_HPP

#include "config.hpp"
#include "Main.hpp"

using namespace System;
using namespace System::Reflection;
using namespace System::IO;
using namespace System::Runtime::InteropServices;

namespace ProjectUser::ProjectName {
	ref class Vars {
	public:
		static Main^ Mod = nullptr;

		static Object^ modInterface = nullptr;
		static Object^ logger = nullptr;

		static Assembly^ GDWeave = nullptr;
		static Assembly^ Serilog = nullptr;
	};
	namespace ImodInterface {
		template <typename T>
		static T ReadConfig() 
		{
			MethodInfo^ readConfigMethod = Vars::modInterface->GetType()->GetMethod("ReadConfig");
			MethodInfo^ genericMethod = readConfigMethod->MakeGenericMethod(T::typeid);
			return (T)genericMethod->Invoke(Vars::modInterface, nullptr);
		};
	};
	namespace Logger {

		static void Information(String^ Str)
		{
			MethodInfo^ logMethod = Vars::logger->GetType()->GetMethod("Information", gcnew array<Type^> { String::typeid });
			logMethod->Invoke(Vars::logger, gcnew array<Object^>{ Str });
		}
	}
	namespace Logger {

		static void Fatal(String^ Str)
		{
			MethodInfo^ logMethod = Vars::logger->GetType()->GetMethod("Fatal", gcnew array<Type^> { String::typeid });
			logMethod->Invoke(Vars::logger, gcnew array<Object^>{ Str });
		}
	}
}

using namespace ProjectUser::ProjectName;

extern "C" {
	__declspec(dllexport) void RunMod(Int64 ptr)
	{
		GCHandle handle = GCHandle::FromIntPtr(IntPtr(ptr));
		Vars::modInterface = (Object^)handle.Target;
		PropertyInfo^ loggerProperty = Vars::modInterface->GetType()->GetProperty("Logger", BindingFlags::Public | BindingFlags::Instance);
		Vars::logger = loggerProperty->GetValue(Vars::modInterface);

		// Call our main code.
		Vars::Mod = gcnew Main;
	}

	__declspec(dllexport) void StopMod()
	{
		if (Vars::Mod)
			delete Vars::Mod;
	}
}

#endif