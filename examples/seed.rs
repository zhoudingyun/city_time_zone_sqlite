#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use serde::Deserialize;
// use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::panic;

use city_time_zone_sqlite::{
    AppError, ErrorType, Repo, TraitRepoD01, TraitRepoD02, TraitRepoD03,
    TraitRepoD04, TraitRepoD05,
};

const PATH: &str = "assets/citys.json";
const PATH_TZ: &str = "assets/tz_utc.json";

#[derive(Debug, Clone)]
pub struct Citys {
    pub city: Vec<City>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct City {
    pub country: String,
    pub name: String,
    pub lat: f32,
    pub lng: f32,
    pub time_zone_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TimeZones {
    pub time_zone: Vec<TimeZone>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeZone {
    pub text: String,
    pub offset: f32,
    pub utc: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct TempD04D02 {
    pub id: String,
    pub name: String,
    pub d03: Vec<TempD04D03>,
}

#[derive(Debug, Clone)]
pub struct TempD04D03 {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct TempD05D01 {
    pub id: String,
    pub name: String,
    pub d02: Vec<TempD05D02>,
}

#[derive(Debug, Clone)]
pub struct TempD05D02 {
    pub id: String,
    pub name: String,
}

impl Citys {
    fn new(path: &str) -> Citys {
        let mut s = String::new();
        let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
        file_path.push(std::env::current_dir().unwrap().as_path());
        file_path.push(path);
        File::open(file_path.as_path())
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        Citys {
            city: serde_json::from_str(&s).unwrap(),
        }
    }
}

impl TimeZones {
    fn new(path: &str) -> TimeZones {
        let mut s = String::new();
        let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
        file_path.push(std::env::current_dir().unwrap().as_path());
        file_path.push(path);
        File::open(file_path.as_path())
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        TimeZones {
            time_zone: serde_json::from_str(&s).unwrap(),
        }
    }
}

fn main() {
    println!("Seed database");
    // If this project is bigger, i need to put this code in one controller
    // for better reading of the code
    let mut i: u32 = 0;
    let citys = Citys::new(PATH);
    let time_zones = TimeZones::new(PATH_TZ);
    let repo = Repo::new();
    // d01
    let mut temp_d05: Vec<TempD05D01> = Vec::new();
    for c in &citys.city {
        let res =
            repo.d01_insert(c.country.as_ref(), c.name.as_ref(), c.lat, c.lng);
        match res {
            Ok(id) => {
                temp_d05.push(TempD05D01 {
                    id: id,
                    name: c.name.clone(),
                    d02: Vec::new(),
                });
                i += 1;
            }
            Err(AppError { err_type, message }) => match err_type {
                _ => {
                    panic!("{:?} {:?}", err_type, message);
                }
            },
        }
    }
    println!("d01 -> {} record(s) insert", i);
    // d02
    i = 0;
    let mut temp_d04: Vec<TempD04D02> = Vec::new();
    for city in citys.city {
        for t in city.time_zone_name {
            let res = repo.d02_insert(t.clone().as_ref());
            match res {
                Ok(id) => {
                    temp_d04.push(TempD04D02 {
                        id: id.clone(),
                        name: t.clone(),
                        d03: Vec::new(),
                    });
                    let clone_d05: Vec<TempD05D01> = temp_d05.clone();
                    temp_d05 = Vec::new();
                    i += 1;
                    for c in clone_d05 {
                        if c.name == city.name {
                            let mut temp_d02 = Vec::new();
                            for c_d02 in c.d02 {
                                temp_d02.push(TempD05D02 {
                                    id: c_d02.id,
                                    name: c_d02.name,
                                });
                            }
                            // Add
                            temp_d02.push(TempD05D02 {
                                id: id.clone(),
                                name: t.clone(),
                            });
                            temp_d05.push(TempD05D01 {
                                id: c.id,
                                name: c.name,
                                d02: temp_d02,
                            });
                        } else {
                            let mut temp_d02 = Vec::new();
                            for c_d02 in c.d02 {
                                temp_d02.push(TempD05D02 {
                                    id: c_d02.id,
                                    name: c_d02.name,
                                });
                            }
                            temp_d05.push(TempD05D01 {
                                id: c.id,
                                name: c.name,
                                d02: temp_d02,
                            });
                        }
                    }
                }
                Err(AppError { err_type, message }) => match err_type {
                    ErrorType::UniqueViolation => {}
                    _ => {
                        panic!("{:?} {:?}", err_type, message);
                    }
                },
            }
        }
    }
    println!("d02 -> {} record(s) insert", i);
    /*for t in &temp_d05 {
        println!("{}", t.d02.len());
    }*/
    //println!("{:?}", temp_d05);
    // d03
    i = 0;
    for t in time_zones.time_zone {
        let res = repo.d03_insert(t.offset, t.text.as_ref());
        match res {
            Ok(id) => {
                for utc in t.utc {
                    let clone_d04: Vec<TempD04D02> = temp_d04.clone();
                    temp_d04 = Vec::new();
                    for c in clone_d04 {
                        if utc == c.name {
                            let mut temp_d03 = Vec::new();
                            for c_d03 in c.d03 {
                                temp_d03.push(TempD04D03 {
                                    id: c_d03.id,
                                    text: c_d03.text,
                                });
                            }
                            // Add
                            temp_d03.push(TempD04D03 {
                                id: id.clone(),
                                text: t.text.clone(),
                            });
                            temp_d04.push(TempD04D02 {
                                id: c.id,
                                name: c.name,
                                d03: temp_d03,
                            });
                        } else {
                            let mut temp_d03 = Vec::new();
                            for c_d03 in c.d03 {
                                temp_d03.push(TempD04D03 {
                                    id: c_d03.id,
                                    text: c_d03.text,
                                });
                            }
                            temp_d04.push(TempD04D02 {
                                id: c.id,
                                name: c.name,
                                d03: temp_d03,
                            });
                        }
                    }
                }
                i += 1;
            }
            Err(AppError { err_type, message }) => {
                println!("{:?}: {}", err_type, message);
                panic!(t.text)
            }
        }
    }
    println!("d03 -> {} record(s) insert", i);
    // println!("{:?}", temp_d04);
    // d04
    i = 0;
    for t_d04 in temp_d04 {
        //t_d04 = d02
        for t_d03 in t_d04.d03 {
            let res = repo.d04_insert(t_d04.id.as_ref(), t_d03.id.as_ref());
            match res {
                Ok(()) => {
                    i += 1;
                }
                Err(AppError { err_type, message }) => match err_type {
                    _ => {
                        panic!("{:?} {:?}", err_type, message);
                    }
                },
            }
        }
    }
    println!("d04 -> {} record(s) insert", i);
    // d05
    i = 0;
    for t_d05 in temp_d05 {
        //t_d05 = d01
        for t_d02 in t_d05.d02 {
            let res = repo.d05_insert(t_d05.id.as_ref(), t_d02.id.as_ref());
            match res {
                Ok(()) => {
                    println!("{}", t_d05.name);
                    i += 1;
                }
                Err(AppError { err_type, message }) => match err_type {
                    _ => {
                        panic!("{:?} {:?}", err_type, message);
                    }
                },
            }
        }
    }
    println!("d05 -> {} record(s) insert", i);
}
/*
 * pub enum ErrorType {
    Internal,
    NotFound,
    UniqueViolation,
}
pub struct TempD02TimeZoneUtc {
    pub id: String,
    pub name: String,
    pub d03: Vec<TempD03TimeZoneInfo>,
}

pub struct TempD03TimeZoneInfo {
    pub id: String,
    pub text: String,
}

*/
