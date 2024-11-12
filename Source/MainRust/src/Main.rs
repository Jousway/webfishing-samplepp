mod CompatLoad;

impl CompatLoad::Config {
    const test: bool = false; 
}

unsafe fn main()
{
    let conf = &mut CompatLoad::Config::default();

    CompatLoad::ReadConfig(conf);
}

fn stop()
{

}