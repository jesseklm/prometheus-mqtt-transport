pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn show_version() {
    print!("{} version {}
Copyright (C) 2022 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.

{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)

", PACKAGE_NAME, VERSION, PACKAGE_NAME);
}

