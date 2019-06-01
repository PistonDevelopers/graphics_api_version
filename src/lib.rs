#![deny(missing_docs)]

//! A library for storing graphics API versions.

/// A graphics API developed by Khronos Group.
/// See https://en.wikipedia.org/wiki/OpenGL for more information.
pub const OPENGL: &'static str = "OpenGL";
/// A graphics API developed by Khronos Group.
/// See https://en.wikipedia.org/wiki/Vulkan_(API) for more information.
pub const VULKAN: &'static str = "Vulkan";
/// A graphics API developed by Microsoft.
/// See https://en.wikipedia.org/wiki/DirectX for more information.
pub const DIRECTX: &'static str = "DirectX";
/// A graphics API developed by Apple.
/// See https://en.wikipedia.org/wiki/Metal_%28API%29 for more information.
pub const METAL: &'static str = "Metal";

use std::borrow::Cow;

/// Stores graphics API version.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Version {
    /// A string identifying the API.
    pub api: Cow<'static, str>,
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
}

impl Version {
    /// Creates a new OpenGL version.
    pub fn opengl(major: u32, minor: u32) -> Version {
        Version {
            api: OPENGL.into(),
            major,
            minor,
        }
    }

    /// Creates a new Vulkan version.
    pub fn vulkan(major: u32, minor: u32) -> Version {
        Version {
            api: VULKAN.into(),
            major,
            minor,
        }
    }

    /// Creates a new DirectX version.
    pub fn directx(major: u32, minor: u32) -> Version {
        Version {
            api: DIRECTX.into(),
            major,
            minor,
        }
    }

    /// Creates a new Metal version.
    pub fn metal(major: u32, minor: u32) -> Version {
        Version {
            api: METAL.into(),
            major,
            minor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let a = Version::opengl(3, 2);
        let b = Version::opengl(4, 0);
        assert!(b > a);
    }
}