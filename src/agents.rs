#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Agent {
    Npm,
    Yarn,
    Pnpm,
    Bun,
    #[allow(dead_code)]
    YarnBerry,
    #[allow(dead_code)]
    Pnpm6,
}

pub const AGENT_MAP: &[(&str, Agent)] = &[
    ("npm", Agent::Npm),
    ("yarn", Agent::Yarn),
    ("pnpm", Agent::Pnpm),
    ("bun", Agent::Bun),
    ("yarn@berry", Agent::YarnBerry),
    ("pnpm@6", Agent::Pnpm6),
];
