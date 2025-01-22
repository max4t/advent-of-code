use std::iter::{self};

use thiserror::Error;

pub enum Damage {
    Physic(u64),
    Magic(u64),
}

#[derive(Error, Debug)]
pub enum CharacterError {
    #[error("i'm dead")]
    SelfOutOfHP,
    #[error("i'm out of mana")]
    SelfOutOfMana,
    #[error("he's dead")]
    EnemyOutOfHP,
    #[error("spell already in effect")]
    SpellAlreadyInEffect,
}

#[derive(Error, Debug)]
enum GameError {
    #[error("character error {0}")]
    CharacterError(#[from] CharacterError)
}

pub trait Character: Clone {
    fn hp(&mut self) -> &mut u64;
    fn armor(&self) -> u64;
    fn gets_attacked(&mut self, damage: Damage) -> Result<(), CharacterError> {
        let d = match damage {
            Damage::Physic(v) => v.saturating_sub(self.armor()),
            Damage::Magic(v) => v,
        }.max(1);
        *self.hp() = self.hp().saturating_sub(d);
        if *self.hp() == 0 {
            Err(CharacterError::SelfOutOfHP)
        } else {
            Ok(())
        }
    }
    fn run_effects(&mut self) -> Result<(), CharacterError>;
    fn poison(&mut self, poison: Poison) -> Result<(), CharacterError>;
    fn shield(&mut self, shield: Shield) -> Result<(), CharacterError>;
}

#[derive(Clone)]
struct Buffered<I: Iterator> {
    buffer: Option<I::Item>,
    iter: I,
}

impl<I: Iterator> Buffered<I> {
    fn new(mut iter: I) -> Self {
        Self {
            buffer: iter.next(),
            iter,
        }
    }
    fn current(&self) -> Option<&I::Item> {
        self.buffer.as_ref()
    }
}
impl<I: Iterator> Iterator for Buffered<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(e) => self.buffer.replace(e),
            None => self.buffer.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let bonus = self.buffer.as_ref().map_or(0, |_| 1);
        match self.iter.size_hint() {
            (lower, None) => (lower + bonus, None),
            (lower, Some(upper)) => (lower + bonus, Some(upper + bonus)),
        }
    }
}
impl<I: ExactSizeIterator> ExactSizeIterator for Buffered<I> {}

#[derive(Clone)]
pub struct Shield(Buffered<iter::RepeatN<u64>>);

impl Shield {
    fn new(shield: u64, turn: usize) -> Self {
        Self(Buffered::new(iter::repeat_n(shield, turn)))
    }

    fn current(&self) -> u64 {
        *self.0.current().unwrap_or(&0)
    }
}
impl Iterator for Shield {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl ExactSizeIterator for Shield {}

#[derive(Clone)]
pub struct Poison(iter::RepeatN<u64>);

impl Poison {
    fn new(damage: u64, turn: usize) -> Self {
        Self(iter::repeat_n(damage, turn))
    }
}

impl Iterator for Poison {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl ExactSizeIterator for Poison {}

#[derive(Clone)]
struct Recharge(iter::RepeatN<u64>);

impl Recharge {
    fn new(mana: u64, turn: usize) -> Self {
        Self(iter::repeat_n(mana, turn))
    }
}

impl Iterator for Recharge {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl ExactSizeIterator for Recharge {}

#[derive(Clone)]
pub struct Warrior {
    hp: u64,
    armor: u64,
    damage: u64,

    poison: Poison,
    shield: Shield,
}

impl Warrior {
    pub fn new(hp: u64, damage: u64) -> Self {
        Self { hp, armor: 0, damage, poison: Poison::new(0, 0), shield: Shield::new(0, 0) }
    }
}

impl Character for Warrior {
    fn hp(&mut self) -> &mut u64 {
        &mut self.hp
    }

    fn armor(&self) -> u64 {
        self.armor + self.shield.current()
    }

    fn run_effects(&mut self) -> Result<(), CharacterError> {
        if let Some(dam) = self.poison.next() {
            self.gets_attacked(Damage::Magic(dam))?;
        }
        self.shield.next();
        Ok(())
    }

    fn poison(&mut self, poison: Poison) -> Result<(), CharacterError> {
        if self.poison.len() > 0 {
            Err(CharacterError::SpellAlreadyInEffect)
        } else {
            self.poison = poison;
            Ok(())
        }
    }

    fn shield(&mut self, shield: Shield) -> Result<(), CharacterError> {
        if self.shield.len() > 0 {
            Err(CharacterError::SpellAlreadyInEffect)
        } else {
            self.shield = shield;
            Ok(())
        }
    }
}

impl Warrior {
    fn attacks(&mut self, other: &mut impl Character) -> Result<(), CharacterError> {
        other.gets_attacked(Damage::Physic(self.damage)).map_err(|e| match e {
            CharacterError::SelfOutOfHP => CharacterError::EnemyOutOfHP,
            e => e,
        })
    }
}

enum TargetSpell<C: Character, E: Character> {
    OntoSelf(for<'a> fn(&'a mut C) -> Result<(), CharacterError>),
    SingleEnemy(for<'a, 'b> fn(&'a mut C, &'b mut E) -> Result<(), CharacterError>),
}

#[derive(Clone)]
pub struct Wizard {
    hp: u64,
    armor: u64,
    mana: u64,

    poison: Poison,
    shield: Shield,
    recharge: Recharge,
}

impl Wizard {
    pub fn new(hp: u64, mana: u64) -> Self {
        Self {
            hp, armor: 0, mana, poison: Poison::new(0, 0), shield: Shield::new(0, 0), recharge: Recharge::new(0, 0),
        }
    }
}

impl Character for Wizard {
    fn hp(&mut self) -> &mut u64 {
        &mut self.hp
    }

    fn armor(&self) -> u64 {
        self.armor + self.shield.current()
    }

    fn run_effects(&mut self) -> Result<(), CharacterError> {
        if let Some(dam) = self.poison.next() {
            self.gets_attacked(Damage::Magic(dam))?;
        }
        if let Some(mana) = self.recharge.next() {
            self.mana += mana;
        }
        self.shield.next();
        Ok(())
    }

    fn poison(&mut self, poison: Poison) -> Result<(), CharacterError> {
        if self.poison.len() > 0 {
            Err(CharacterError::SpellAlreadyInEffect)
        } else {
            self.poison = poison;
            Ok(())
        }
    }

    fn shield(&mut self, shield: Shield) -> Result<(), CharacterError> {
        if self.shield.len() > 0 {
            Err(CharacterError::SpellAlreadyInEffect)
        } else {
            self.shield = shield;
            Ok(())
        }
    }
}

impl Wizard {
    fn magic_missile<C: Character>(&mut self, other: &mut C) -> Result<(), CharacterError> {
        self.mana = self.mana.checked_sub(53).ok_or(CharacterError::SelfOutOfMana)?;
        other.gets_attacked(Damage::Magic(4))
    }
    fn drain<C: Character>(&mut self, other: &mut C) -> Result<(), CharacterError> {
        self.mana = self.mana.checked_sub(73).ok_or(CharacterError::SelfOutOfMana)?;
        other.gets_attacked(Damage::Magic(2))?;
        *self.hp() += 2;
        Ok(())
    }
    fn shield(&mut self) -> Result<(), CharacterError> {
        self.mana = self.mana.checked_sub(113).ok_or(CharacterError::SelfOutOfMana)?;
        <Self as Character>::shield(self, Shield::new(7, 6))
    }
    fn poison<C: Character>(&mut self, other: &mut C) -> Result<(), CharacterError> {
        self.mana = self.mana.checked_sub(173).ok_or(CharacterError::SelfOutOfMana)?;
        other.poison(Poison::new(3, 6))
    }
    fn recharge(&mut self) -> Result<(), CharacterError> {
        self.mana = self.mana.checked_sub(229).ok_or(CharacterError::SelfOutOfMana)?;
        if self.recharge.len() > 0 {
            Err(CharacterError::SpellAlreadyInEffect)
        } else {
            self.recharge = Recharge::new(101, 5);
            Ok(())
        }
    }
}

pub struct Action<C: Character, E: Character> {
    cost: u64,
    run: TargetSpell<C, E>,
}

#[derive(Clone)]
pub struct Game<P: Character> {
    pub player: P,
    pub enemy: Warrior,
}

impl<P: Character> Game<P>
where
    P: CharacterActions<Warrior>
{
    fn play_turn(&mut self, action: Action<P, Warrior>) -> Result<&Self, CharacterError> {
        self.run_effects()?;
        match action.run {
            TargetSpell::OntoSelf(r) => r(&mut self.player),
            TargetSpell::SingleEnemy(r) => r(&mut self.player, &mut self.enemy),
        }?;
        self.run_effects()?;
        self.enemy.attacks(&mut self.player).map_err(|e| match e {
            CharacterError::EnemyOutOfHP => CharacterError::SelfOutOfHP,
            CharacterError::SelfOutOfHP => CharacterError::EnemyOutOfHP,
            e => e,
        })?;
        Ok(self)
    }
    fn run_effects(&mut self) -> Result<(), CharacterError> {
        self.player.run_effects()?;
        self.enemy.run_effects().map_err(|e| match e {
            CharacterError::EnemyOutOfHP => CharacterError::SelfOutOfHP,
            CharacterError::SelfOutOfHP => CharacterError::EnemyOutOfHP,
            e => e,
        })?;
        Ok(())
    }
    fn run(&self, cutoff: u64, cost: u64) -> u64 {
        P::actions().into_iter().fold(cutoff, |min, action| {
            let acc_cost = action.cost + cost;
            if acc_cost >= min {
                return min
            }

            match self.clone().play_turn(action) {
                Ok(game) => game.run(min, acc_cost),
                Err(CharacterError::EnemyOutOfHP) => acc_cost,
                Err(_) => min,
            }
        })
    }
    pub fn process(&self) -> u64 {
        self.run(u64::MAX, 0)
    }
}

pub trait CharacterActions<E: Character>
where
    Self: Character,
{
    fn actions() -> Vec<Action<Self, E>>;
}

impl CharacterActions<Warrior> for Wizard {
    fn actions() -> Vec<Action<Self, Warrior>> {
        vec![
            Action {
                cost: 53,
                run: TargetSpell::SingleEnemy(Self::magic_missile),
            },
            Action {
                cost: 73,
                run: TargetSpell::SingleEnemy(Self::drain),
            },
            Action {
                cost: 113,
                run: TargetSpell::OntoSelf(Self::shield),
            },
            Action {
                cost: 173,
                run: TargetSpell::SingleEnemy(Self::poison),
            },
            Action {
                cost: 229,
                run: TargetSpell::OntoSelf(Self::recharge),
            },
        ]
    }
}
