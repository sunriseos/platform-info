// This file is part of the uutils coreutils package.
//
// (c) Alex Lyon <arcterus@mail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

use super::Uname;
use std::borrow::Cow;
use std::io;

pub struct PlatformInfo;

impl PlatformInfo {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }
}

impl Uname for PlatformInfo {
    fn sysname(&self) -> Cow<str> {
        Cow::from("Sunrise")
    }

    fn nodename(&self) -> Cow<str> {
        Cow::from("ocean")
    }

    fn release(&self) -> Cow<str> {
        Cow::from("1.0.0")
    }

    fn version(&self) -> Cow<str> {
        Cow::from("#1 Horizon 5.0.0 compatible")
    }

    fn machine(&self) -> Cow<str> {
        Cow::from("x86")
    }
}