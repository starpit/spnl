// Sigh, not sure how to get this automatically from Rust, yet. Vite seems to be getting in the way.
export type User = { user: [string] }
export type System = { system: [string] }
export type Print = { print: [string] }
export type Plus = { plus: Unit[] }
export type Cross = { cross: Unit[] }
export type Repeat = { repeat: [number, Unit] }
export type Generate = { g: [string, Unit, number, number] }
export type Unit = Print | User | System | Plus | Cross | Generate
