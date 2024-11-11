using System.Runtime.InteropServices;
using GDWeave;

namespace ${PROJECT_NAME};

public class Mod : IMod {
	[DllImport("${PROJECT_NAME}.Data.dll", SetLastError = true)]
	public static extern void RunMod(long ptr);
	
	[DllImport("${PROJECT_NAME}.Data.dll", SetLastError = true)]
	public static extern void StopMod();

    public Mod(IModInterface modInterface) {
        RunMod(GCHandle.ToIntPtr(GCHandle.Alloc(modInterface, GCHandleType.Normal)).ToInt64());
    }

    public void Dispose() {
        StopMod();
    }
}