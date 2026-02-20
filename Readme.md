# Morph

![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

Morph is a system inspired by **ProtoBuf** that allows you to describe and serialize data packets using `.morph` files and compile them into code for multiple languages, including Java.

---

## Features

- Simple declarative syntax for defining `.morph` packets
- Code generation for different languages
- Support for nested packets and multi-dimensional arrays

## Language Support

- Java ✅
- Rust (In future)
- Kotlin (In future)

## Usage
### 1. Initialize a Morph project

Set up the Morph environment in your project directory:
```
morph init
```

This creates:
- morph.toml — project configuration (it is pretty simple)
- main.morph — empty description file

### 2. Define packets in .morph files

Packets are defined with a clear declarative syntax.
Each packet has:
- id — unique identifier (0 - 255, so far single byte)
- fields — a list of typed fields

#### Supported field types
| Type     | Description                                   |
|----------|-----------------------------------------------|
| `bool`   | Boolean value                                 |
| `i8`     | 8-bit signed integer                          |
| `i16`    | 16-bit signed integer                         |
| `i32`    | 32-bit signed integer                         |
| `i64`    | 64-bit signed integer                         |
| `u8`     | 8-bit unsigned integer                        |
| `u16`    | 16-bit unsigned integer                       |
| `u32`    | 32-bit unsigned integer                       |
| `u64`    | 64-bit unsigned integer                       |
| `f32`    | 32-bit floating point                         |
| `f64`    | 64-bit floating point                         |
| `char`   | single UTF-8 character                        |
| `string` | UTF-8 encoded string                          |
| `array`  | Array of another type, e.g., `array i32`      |
| Custom   | Nested packet type, e.g., `Position position` |

#### Supported id types
- number 0-255 - manual id set up. 
- auto - generates packet id using sha256

#### Example .morph file
```morph
packet Position {
    id = auto
    fields {
        f32 x
        f32 y
        f32 z
        f32 yaw
        f32 pitch
        string world
    }
}

packet PlayerData {
    id = auto
    fields {
        string name
        string lang
        u64 last_online
        Position position
        bool dead
    }
}

packet ParticleOptions {
    id = auto
    fields {
        string type
        f32 speed
    }
}

packet Particle {
    id = auto
    fields {
        ParticleOptions options
        i32 count
    }
}

packet Particles {
    id = auto
    fields {
        array Particle particles
    }
}

packet Matrix2DPacket {
    id = auto
    fields {
        array array i32 data
    }
}
```

#### Compile .morph files
them into target language code:
```
morph compile <lang>
```
The compiler generates:
- Classes/structs for each packet
- Support for nested packets and arrays
- Other things depending on the target lang


### Tips
- Use nested packets to model complex structures
- Use arrays for repeated fields or multi-dimensional data
- Keep .morph files small and modular — one packet per file is recommended
- You can pass an exact directory to the commands using `-i <path>`. By default it uses current directory.

## Plans

- Optional fields
- Enums
- Maps
- Java Runtime Improvements (gradle plugin, netty integration)