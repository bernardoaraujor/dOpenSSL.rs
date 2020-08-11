extern crate cmake;
use cmake::Config;

fn main()
{
    let dst = Config::new("cmake")
                     .cflag("-DCMAKE_INSTALL_PREFIX=$PWD/ext_install")
                     .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
}
