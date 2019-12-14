//! # Day 14: Space Stoichiometry
//!
//! As you approach the rings of Saturn, your ship's low fuel indicator turns
//! on. There isn't any fuel here, but the rings have plenty of raw material.
//! Perhaps your ship's Inter-Stellar Refinery Union brand nanofactory can turn
//! these raw materials into fuel.
//!
//! You ask the nanofactory to produce a list of the reactions it can perform
//! that are relevant to this process (your puzzle input). Every reaction turns
//! some quantities of specific input chemicals into some quantity of an output
//! chemical. Almost every chemical is produced by exactly one reaction; the
//! only exception, ORE, is the raw material input to the entire process and is
//! not produced by a reaction.
//!
//! You just need to know how much ORE you'll need to collect before you can
//! produce one unit of FUEL.
//!
//! Each reaction gives specific quantities for its inputs and output; reactions
//! cannot be partially run, so only whole integer multiples of these quantities
//! can be used. (It's okay to have leftover chemicals when you're done,
//! though.) For example, the reaction `1 A, 2 B, 3 C => 2 D` means that exactly
//! 2 units of chemical D can be produced by consuming exactly 1 A, 2 B and 3 C.
//! You can run the full reaction as many times as necessary; for example, you
//! could produce 10 D by consuming 5 A, 10 B, and 15 C.
//!
//! Suppose your nanofactory produces the following list of reactions:
//!
//! ```text
//! 10 ORE => 10 A
//! 1 ORE => 1 B
//! 7 A, 1 B => 1 C
//! 7 A, 1 C => 1 D
//! 7 A, 1 D => 1 E
//! 7 A, 1 E => 1 FUEL
//! ```
//!
//! The first two reactions use only ORE as inputs; they indicate that you can
//! produce as much of chemical A as you want (in increments of 10 units, each
//! 10 costing 10 ORE) and as much of chemical B as you want (each costing 1
//! ORE). To produce 1 FUEL, a total of 31 ORE is required: 1 ORE to produce 1
//! B, then 30 more ORE to produce the 7 + 7 + 7 + 7 = 28 A (with 2 extra A
//! wasted) required in the reactions to convert the B into C, C into D, D into
//! E, and finally E into FUEL. (30 A is produced because its reaction requires
//! that it is created in increments of 10.)
//!
//! Or, suppose you have the following list of reactions:
//!
//! ```text
//! 9 ORE => 2 A
//! 8 ORE => 3 B
//! 7 ORE => 5 C
//! 3 A, 4 B => 1 AB
//! 5 B, 7 C => 1 BC
//! 4 C, 1 A => 1 CA
//! 2 AB, 3 BC, 4 CA => 1 FUEL
//! ```
//!
//! The above list of reactions requires 165 ORE to produce 1 FUEL:
//!
//! * Consume 45 ORE to produce 10 A.
//! * Consume 64 ORE to produce 24 B.
//! * Consume 56 ORE to produce 40 C.
//! * Consume 6 A, 8 B to produce 2 AB.
//! * Consume 15 B, 21 C to produce 3 BC.
//! * Consume 16 C, 4 A to produce 4 CA.
//! * Consume 2 AB, 3 BC, 4 CA to produce 1 FUEL.
//!
//! Here are some larger examples:
//!
//! * 13312 ORE for 1 FUEL:
//!
//! ```text
//! 157 ORE => 5 NZVS
//! 165 ORE => 6 DCFZ
//! 44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
//! 12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
//! 179 ORE => 7 PSHF
//! 177 ORE => 5 HKGWZ
//! 7 DCFZ, 7 PSHF => 2 XJWVT
//! 165 ORE => 2 GPVTF
//! 3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
//! ```
//!
//! * 180697 ORE for 1 FUEL:
//!
//! ```text
//! 2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
//! 17 NVRVD, 3 JNWZP => 8 VPVL
//! 53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
//! 22 VJHF, 37 MNCFX => 5 FWMGM
//! 139 ORE => 4 NVRVD
//! 144 ORE => 7 JNWZP
//! 5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
//! 5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
//! 145 ORE => 6 MNCFX
//! 1 NVRVD => 8 CXFTF
//! 1 VJHF, 6 MNCFX => 4 RFSQX
//! 176 ORE => 6 VJHF
//! ```
//!
//! * 2210736 ORE for 1 FUEL:
//!
//! ```text
//! 171 ORE => 8 CNZTR
//! 7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
//! 114 ORE => 4 BHXH
//! 14 VRPVC => 6 BMBT
//! 6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
//! 6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
//! 15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
//! 13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
//! 5 BMBT => 4 WPTQ
//! 189 ORE => 9 KTJDG
//! 1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
//! 12 VRPVC, 27 CNZTR => 2 XDBXC
//! 15 KTJDG, 12 BHXH => 5 XCVML
//! 3 BHXH, 2 VRPVC => 7 MZWV
//! 121 ORE => 7 VRPVC
//! 7 XCVML => 6 RJRHP
//! 5 BHXH, 4 VRPVC => 5 LTCX
//! ```
//!
//! Given the list of reactions in your puzzle input, what is the minimum amount
//! of ORE required to produce exactly 1 FUEL?

use anyhow::{Error, Result};
use std::str::FromStr;

const PUZZLE_INPUT: &str = include_str!("../inputs/input-14");

#[derive(Clone, Debug, PartialEq, Eq)]
struct Reaction {
    inputs: Vec<Reactant>,
    output: Reactant,
}

impl FromStr for Reaction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().splitn(2, "=>");
        let input_str = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing inputs"))?;
        let inputs: Vec<Reactant> = input_str
            .split(',')
            .map(|r| r.parse())
            .collect::<Result<_, _>>()?;
        let output_str = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing name"))?;
        let output = output_str.parse()?;
        Ok(Self { inputs, output })
    }
}

#[derive(Clone, Debug)]
struct Reactant {
    name: ReactantName,
    amount: usize,
}

impl std::hash::Hash for Reactant {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.name.hash(h)
    }
}

impl PartialEq for Reactant {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Reactant {}

impl std::borrow::Borrow<ReactantName> for Reactant {
    fn borrow(&self) -> &ReactantName {
        &self.name
    }
}

impl std::borrow::Borrow<str> for Reactant {
    fn borrow(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct ReactantName(String);

impl AsRef<str> for ReactantName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Reactant {
    fn fuel() -> Self {
        Self {
            name: ReactantName(String::from("FUEL")),
            amount: 1,
        }
    }
}

impl FromStr for Reactant {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().splitn(2, ' ');
        let qty = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing quantity"))?;
        log::debug!("Quantity: {}", qty);
        let qty = qty.parse().map_err(|_| anyhow::anyhow!("bad quantity"))?;
        let name = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing name"))?;
        log::debug!("Name: {}", name);
        Ok(Self {
            name: ReactantName(String::from(name)),
            amount: qty,
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Reaction>> {
    use std::io::{BufRead, Cursor};

    let reactions: Vec<Reaction> = Cursor::new(input)
        .lines()
        .filter_map(|line| {
            if let Ok(l) = line {
                let trimmed = l.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.parse())
                }
            } else {
                None
            }
        })
        .collect::<Result<_>>()?;

    Ok(reactions)
}

fn find_ore_requirement(reactions: Vec<Reaction>, fuel_needed: usize) -> usize {
    use std::collections::HashMap;
    let reaction_map: HashMap<_, _> = reactions
        .into_iter()
        .map(|react| (react.output.name, (react.inputs, react.output.amount)))
        .collect();

    let mut in_store = HashMap::<ReactantName, isize>::new();

    in_store.insert(ReactantName(String::from("FUEL")), -(fuel_needed as isize));

    loop {
        let (inputs, mul) = {
            if let Some((need, v)) = in_store
                .iter_mut()
                .filter(|(k, &mut x)| x < 0 && k.as_ref() != "ORE")
                .next()
            {
                log::debug!("Processing {:?} for {}", need, v);
                let (ins, amt) = &reaction_map[need];
                *v += *amt as isize;
                (ins, *amt as isize)
            } else {
                break;
            }
        };
        log::debug!("Inputs {:?}, multiplier {}", inputs, 1);
        for input in inputs {
            let stock = in_store.entry(input.name.clone()).or_default();
            *stock -= input.amount as isize;
        }
        log::debug!("State {:?}", in_store);
    }

    in_store[&ReactantName(String::from("ORE"))].abs() as usize
}

pub fn run() -> Result<()> {
    let reactions = parse_input(PUZZLE_INPUT)?;

    let actual = find_ore_requirement(reactions, 1);

    println!("Ore requirements for 1 fuel: {}", actual);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{find_ore_requirement, parse_input};
    use anyhow::Result;

    const EXAMPLE_1: &str = "
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL";
    const EXAMPLE_1_ORE: usize = 31;

    #[test]
    fn find_ore_requirement_example_1() -> Result<()> {
        crate::init_logging();
        let reactions = parse_input(EXAMPLE_1)?;

        let actual = find_ore_requirement(reactions, 1);

        assert_eq!(actual, EXAMPLE_1_ORE);

        Ok(())
    }

    const EXAMPLE_2: &str = "
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";
    const EXAMPLE_2_ORE: usize = 165;

    #[test]
    fn find_ore_requirement_example_2() -> Result<()> {
        crate::init_logging();
        let reactions = parse_input(EXAMPLE_2)?;

        let actual = find_ore_requirement(reactions, 1);

        assert_eq!(actual, EXAMPLE_2_ORE);

        Ok(())
    }

    const EXAMPLE_3: &str = "
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    const EXAMPLE_3_ORE: usize = 13312;

    #[test]
    fn find_ore_requirement_example_3() -> Result<()> {
        crate::init_logging();
        let reactions = parse_input(EXAMPLE_3)?;

        let actual = find_ore_requirement(reactions, 1);

        assert_eq!(actual, EXAMPLE_3_ORE);

        Ok(())
    }

    const EXAMPLE_4: &str = "
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF";
    const EXAMPLE_4_ORE: usize = 180_697;

    #[test]
    fn find_ore_requirement_example_4() -> Result<()> {
        crate::init_logging();
        let reactions = parse_input(EXAMPLE_4)?;

        let actual = find_ore_requirement(reactions, 1);

        assert_eq!(actual, EXAMPLE_4_ORE);

        Ok(())
    }

    const EXAMPLE_5: &str = "
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX";
    const EXAMPLE_5_ORE: usize = 2_210_736;

    #[test]
    fn find_ore_requirement_example_5() -> Result<()> {
        crate::init_logging();
        let reactions = parse_input(EXAMPLE_5)?;

        let actual = find_ore_requirement(reactions, 1);

        assert_eq!(actual, EXAMPLE_5_ORE);

        Ok(())
    }
}
