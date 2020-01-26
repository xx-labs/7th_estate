//! # Plane Filter Entries

use super::*;

#[derive(Debug, Clone)]
pub struct PlaneFilterEntry {
    pub key: AEADKey,
    pub decrypt: bool
}

#[derive(Debug, Clone)]
pub struct PlaneFilterRecord {
    pub col1: PlaneFilterEntry,
    pub col3: PlaneFilterEntry
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaneFilterFileRecord {
    pub col1: String,
    pub col2: String,
    pub col3: String
}

pub struct PlaneFilter {
    pub rows: Vec<PlaneFilterRecord>
}

pub struct PermutedPlaneFilter {
    pub rows: Vec<PlaneFilterRecord>
}

impl PlaneFilter {
    pub fn from(col1_keys: &Vec<AEADKey>, col3_keys: &Vec<AEADKey>) -> Self {
        assert!(col1_keys.len() == col3_keys.len(),
            "Number of rows do not agree between the first and third columns.");
        PlaneFilter {
            rows: col1_keys.iter().zip(col3_keys.iter())
                .map(|(k1, k3)| {
                    PlaneFilterRecord {
                        col1: PlaneFilterEntry { key: k1.clone(), decrypt: false },
                        col3: PlaneFilterEntry { key: k3.clone(), decrypt: false },
                    }
                }).collect()
        }
    }

    pub fn decrypt_serials(self: &Self, serials: &Vec<BallotSerial>) -> Self {
        Self {
            rows: self.rows.iter().enumerate()
                .map(|(n, row)| {
                    let serial = n/2;
                    if serials.contains(&serial) {
                        PlaneFilterRecord {
                            col1: PlaneFilterEntry { key: row.col1.key.clone(), decrypt: true },
                            col3: PlaneFilterEntry { key: row.col3.key.clone(), decrypt: true }
                        }
                    } else {
                        row.clone()
                    }
                }).collect()
        }
    }

    pub fn decrypt_column(self: &Self, column_number: usize) -> Self {
        match column_number {
            1 => {
                Self {
                    rows: self.rows.iter()
                        .map(|row| {
                            PlaneFilterRecord {
                                col1: PlaneFilterEntry { key: row.col1.key.clone(), decrypt: true },
                                col3: row.col3.clone()
                            }
                        }).collect()
                }
            },
            3 => {
                Self {
                    rows: self.rows.iter()
                        .map(|row| {
                            PlaneFilterRecord {
                                col1: row.col1.clone(),
                                col3: PlaneFilterEntry { key: row.col3.key.clone(), decrypt: true }
                            }
                        }).collect()
                }
            },
            _ => {
                panic!("Filter construction failed.");
            }
        }
    }
    
    pub fn len(self: &Self) -> usize { self.rows.len() }

    pub fn permute(self: &Self, permutation: &Vec<usize>) -> PermutedPlaneFilter {
        PermutedPlaneFilter {
            rows: permutation.iter().map(|&n| self.rows[n].clone()).collect()
        }
    }
}

impl PermutedPlaneFilter {
    pub fn serializable(self: &Self) -> Vec<PlaneFilterFileRecord> {
        self.rows.iter()
            .map(|row| {
                PlaneFilterFileRecord {
                    col1: if row.col1.decrypt { base64::encode(&row.col1.key.0) } else { "".to_owned() },
                    col2: "".to_owned(),
                    col3: if row.col3.decrypt { base64::encode(&row.col3.key.0) } else { "".to_owned() }
                }
            }).collect()
    }
}

