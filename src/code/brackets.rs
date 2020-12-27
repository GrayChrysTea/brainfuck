//! [`brainfucklib::code::brackets`]
//! 
//! This module helps to identify pairs of brackets and their locations.

use std::{
    collections::HashMap,
    cmp::Ordering
};

use crate::{
    debug::{BfErrorKind},
    someorreturn
};

/// This categorizes a bracket into [`Left`] or [`Right`] hand size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BType {
    Left,
    Right,
}

impl BType {
    /// Checks whether the bracket is on the left.
    pub fn is_left(&self) -> bool {
        return match self {
            BType::Left => true,
            BType::Right => false,
        };
    }

    /// Checks whether the bracket is on the right.
    pub fn is_right(&self) -> bool {
        return !self.is_left();
    }
}

/// A representation of a [`Bracket`], identified by side, kind and the
/// location of its counterpart.
/// 
/// For brackets of the same family (i.e. '(' with ')' or '[' with ']'), their
/// kind should have the same number. This means that:
/// - '(' and ')' should be of kind `n`, and
/// - '[' and ']' should be of kind `m`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bracket {
    btype: BType,
    kind: u8,
    counterpart: Option<usize>,
}

impl Bracket {
    /// Create a new [`Bracket`] instance.
    pub fn new(btype: BType, kind: u8) -> Self {
        return Self {btype, kind, counterpart: None};
    }

    /// Set the counterpart of this [`Bracket`].
    pub fn set_counterpart(&mut self, counterpart: usize) -> &mut Self {
        self.counterpart = Some(counterpart);
        return self;
    }

    /// Get the counterpart to this [`Bracket`].
    pub fn counterpart(&self) -> Option<usize> {
        return self.counterpart;
    }

    /// Get the [`BType`] of this [`Bracket`].
    pub fn btype(&self) -> BType {
        return self.btype;
    }

    /// Get the family of this [`Bracket`].
    pub fn kind(&self) -> u8 {
        return self.kind;
    }
}

type BMap = HashMap<usize, Bracket>;

#[derive(Clone, Debug)]
struct _Bracket {
    pub location: usize,
    pub bracket: Bracket,
}

impl Eq for _Bracket {}

impl PartialEq for _Bracket {
    fn eq(&self, other: &Self) -> bool {
        return self.location == other.location;
    }
}

impl PartialOrd for _Bracket {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        return self.location.partial_cmp(&rhs.location);
    }
}

impl Ord for _Bracket {
    fn cmp(&self, rhs: &Self) -> Ordering {
        return self.location.cmp(&rhs.location);
    }
}

type BMapResult<T> = Result<T, (BfErrorKind, usize)>;

/// A mapping of [`Bracket`]s and their counterparts.
#[derive(Clone, Debug)]
pub struct BracketMap {
    map: BMap,
}

impl BracketMap {
    /// Create a new [`BracketMap`].
    pub fn new() -> Self {
        let map: BMap = HashMap::new();
        return Self {map};
    }

    /// Insert a [`Bracket`] with its location (in the program file) into
    /// the map.
    pub fn insert(&mut self, bracket: Bracket, location: usize) -> bool {
        if self.map.contains_key(&location) {
            return false;
        }
        self.map.insert(location, bracket);
        return true;
    }

    /// Sort the brackets by location.
    fn get_sorted_brackets(&self) -> Vec<_Bracket> {
        let mut brackets: Vec<_Bracket> = Vec::new();
        for (location, bracket) in self.map.iter() {
            let location = location.clone();
            let bracket = bracket.clone();
            let bracket = _Bracket {location, bracket};
            brackets.push(bracket);
        }
        brackets.sort();
        return brackets;
    }

    /// Check if the brackets all have counterparts and are balanced.
    pub fn is_balanced(&self) -> BMapResult<()> {
        let brackets = self.get_sorted_brackets();
        let mut stack: Vec<_Bracket> = Vec::new();
        for _bracket in brackets.iter() {
            let _bracket = _bracket.clone();
            let location = _bracket.location;
            let bracket = _bracket.bracket.clone();
            if bracket.btype().is_left() {
                stack.push(_bracket.clone());
            } else {
                let top = someorreturn!(
                    stack.last(),
                    Err((
                        BfErrorKind::UnmatchedRightBracket,
                        location
                    ))
                ).clone();
                if top.bracket.kind() == bracket.kind() {
                    stack.pop();
                } else {
                    return Err((
                        BfErrorKind::UnmatchedRightBracket,
                        location
                    ))
                }
            }
        }
        return match stack.is_empty() {
            true => Ok(()),
            false => Err((
                BfErrorKind::UnmatchedLeftBracket,
                stack.last().unwrap().location
            ))
        };
    }

    /// Pair up the [`Bracket`]s together.
    pub fn pair_up(&mut self) -> BMapResult<()> {
        // Get brackets in correct order because hash maps do not order
        // values by key.
        let brackets = self.get_sorted_brackets();
        // Get a copy of the bracket map. Do not directly edit the map in case
        // there is an error.
        let mut map = self.map.clone();
        // Using Vector because rust doesn't have a stack.
        let mut stack: Vec<_Bracket> = Vec::new();
        for _bracket in &brackets {
            let _bracket = _bracket.clone();
            let location = _bracket.location;
            let bracket = _bracket.bracket.clone();
            // If the bracket is left (e.g. '(', '[' or '{'), push to the
            // stack.
            if bracket.btype().is_left() {
                stack.push(_bracket.clone());
            } else {
                // Otherwise
                let top = someorreturn!(
                    stack.last(),
                    Err((
                        BfErrorKind::UnmatchedRightBracket,
                        location
                    ))
                ).clone();
                // Check if the bracket types match
                // E.g. '(' with ')' or '[' with ']'
                // If check fails, return false.
                if top.bracket.kind() == bracket.kind() {
                    stack.pop();
                    // Pair the 2 brackets up.
                    let left = someorreturn!(
                        map.get_mut(&top.location),
                        Err((
                            BfErrorKind::UnmatchedLeftBracket,
                            top.location
                        ))
                    );
                    left.set_counterpart(location);
                    let right = someorreturn!(
                        map.get_mut(&location),
                        Err((
                            BfErrorKind::UnmatchedRightBracket,
                            location
                        ))
                    );
                    right.set_counterpart(top.location);
                } else {
                    return Err((
                        BfErrorKind::UnmatchedRightBracket,
                        location
                    ));
                }
            }
        }

        // Set the new map as the true map.
        self.map = map;
        return Ok(());
    }

    pub fn get_counterpart(&self, index: usize) -> Option<usize> {
        return self.map.get(&index)?.counterpart();
    }
}

impl AsRef<BMap> for BracketMap {
    fn as_ref(&self) -> &BMap {
        return &self.map;
    }
}

impl AsMut<BMap> for BracketMap {
    fn as_mut(&mut self) -> &mut BMap {
        return &mut self.map;
    }
}