mod bounding_box;

use clap::Parser;
use core::str::FromStr;
use serde::{Deserialize};

use crate::bounding_box::{IVec2, BoundingBox,
                          Vec2,
                          TopLeftBoundingBox,
                          CenterBoundingBox,
                          NormalizedCenterBoundingBox,
};

#[derive(Debug, Clone)]
enum BoundingBoxen {
    BoundingBox,
    TopLeftBoundingBox,
    CenterBoundingBox,
    NormalizedCenterBoundingBox,
}

impl FromStr for BoundingBoxen {
    type Err = core::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bb" => Ok(BoundingBoxen::BoundingBox),
            "bbox" => Ok(BoundingBoxen::BoundingBox),
            "xyxy" => Ok(BoundingBoxen::BoundingBox),

            "tlbb" => Ok(BoundingBoxen::TopLeftBoundingBox),
            "tlbbox" => Ok(BoundingBoxen::TopLeftBoundingBox),
            "top-left-bounding-box" => Ok(BoundingBoxen::TopLeftBoundingBox),

            "cbb" => Ok(BoundingBoxen::CenterBoundingBox),
            "cbbox" => Ok(BoundingBoxen::CenterBoundingBox),
            "center-bounding-box" => Ok(BoundingBoxen::CenterBoundingBox),

            "ncbb" => Ok(BoundingBoxen::NormalizedCenterBoundingBox),
            "ncbbox" => Ok(BoundingBoxen::NormalizedCenterBoundingBox),
            "normalized-center-bounding-box" => Ok(BoundingBoxen::NormalizedCenterBoundingBox),
            "yolo" => Ok(BoundingBoxen::NormalizedCenterBoundingBox),

            _ => Err(core::fmt::Error)
        }
    }
}

/// Utility to convert bounding box formats between each other.
#[derive(Parser, Debug)]
#[clap(name = "bboxconvert")]
#[clap(bin_name = "bboxconvert")]
struct BBoxConvert {
    #[clap(short, long, required=true, value_parser)]
    input: BoundingBoxen,

    #[clap(short, long, required=true, value_parser)]
    output: BoundingBoxen,

    #[clap(short, long, value_parser)]
    width: Option<i32>,

    #[clap(short, long, value_parser)]
    height: Option<i32>,
}

#[derive(Debug)]
struct Error(String);

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Self(format!("{}", e))
    }
}

#[derive(Deserialize, Debug)]
struct Coordinate {
    i: f32,
    j: f32,
    k: f32,
    l: f32,
}

fn main() -> Result<(), Error> {
    let args = BBoxConvert::parse();
    //println!("args: {:?}", args);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(std::io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Coordinate = result?.deserialize(None)?;
        //println!("- record: {:?}", record);
        //assert_eq!(record.len(), 4);
        match args.input {
            BoundingBoxen::BoundingBox => {
                let input = BoundingBox {
                    min: IVec2(record.i as i32, record.j as i32),
                    max: IVec2(record.k as i32, record.l as i32)
                };
                match args.output {
                    BoundingBoxen::BoundingBox => {
                        println!("{}", input);
                    },
                    BoundingBoxen::TopLeftBoundingBox => {
                        let output = TopLeftBoundingBox::from(input);
                        println!("{}", output);
                    },
                    BoundingBoxen::CenterBoundingBox => {
                        let output = CenterBoundingBox::from(input);
                        println!("{}", output);
                    },
                    BoundingBoxen::NormalizedCenterBoundingBox => {
                        let output = NormalizedCenterBoundingBox::from(
                            (
                                CenterBoundingBox::from(input),
                                Vec2(
                                    args.width.expect("NormalizedCenterBoundingBox requires width argument") as f32,
                                    args.height.expect("NormalizedCenterBoundingBox requires height argument") as f32
                                )

                            )
                        );
                        println!("{}", output);
                    },
                }
            },
            BoundingBoxen::TopLeftBoundingBox => {
                let input = TopLeftBoundingBox {
                    top_left: IVec2(record.i as i32, record.j as i32),
                    size: IVec2(record.k as i32, record.l as i32),
                };
                match args.output {
                    BoundingBoxen::BoundingBox => {
                        let output = BoundingBox::from(input);
                        println!("{}", output);
                    },
                    BoundingBoxen::TopLeftBoundingBox => {
                        println!("{}", input);
                    },
                    BoundingBoxen::CenterBoundingBox => {
                        let bbox = BoundingBox::from(input);
                        let output = CenterBoundingBox::from(bbox);
                        println!("{}", output);
                    },
                    BoundingBoxen::NormalizedCenterBoundingBox => {
                        let bbox = BoundingBox::from(input);
                        let cbbox = CenterBoundingBox::from(bbox);
                        let output = NormalizedCenterBoundingBox::from((
                            cbbox,
                            Vec2(
                                args.width.expect("NormalizedCenterBoundingBox requires width argument") as f32,
                                args.height.expect("NormalizedCenterBoundingBox requires height argument") as f32
                            )
                        ));
                        println!("{}", output);
                    },
                }
            },
            BoundingBoxen::CenterBoundingBox => {
                let input = CenterBoundingBox {
                    center: IVec2(record.i as i32, record.j as i32),
                    size: IVec2(record.k as i32, record.l as i32),
                };
                match args.output {
                    BoundingBoxen::BoundingBox => {
                        let output = BoundingBox::from(input);
                        println!("{}", output);
                    },
                    BoundingBoxen::TopLeftBoundingBox => {
                        let bbox = BoundingBox::from(input);
                        let output = TopLeftBoundingBox::from(bbox);
                        println!("{}", output);
                    },
                    BoundingBoxen::CenterBoundingBox => {
                        println!("{}", input);
                    },
                    BoundingBoxen::NormalizedCenterBoundingBox => {
                        let output = NormalizedCenterBoundingBox::from((
                            input,
                            Vec2(
                                args.width.expect("NormalizedCenterBoundingBox requires width argument") as f32,
                                args.height.expect("NormalizedCenterBoundingBox requires height argument") as f32
                            )
                        ));
                        println!("{}", output);
                    },
                }
            },
            BoundingBoxen::NormalizedCenterBoundingBox => {
                let input = NormalizedCenterBoundingBox {
                    center: Vec2(record.i, record.j),
                    size: Vec2(record.k, record.l),
                };
                match args.output {
                    BoundingBoxen::BoundingBox => {
                        let output = BoundingBox::from((
                            input,
                            Vec2(
                                args.width.expect("NormalizedCenterBoundingBox requires width argument") as f32,
                                args.height.expect("NormalizedCenterBoundingBox requires height argument") as f32
                            )
                        ));
                        println!("{}", output);
                    },
                    BoundingBoxen::TopLeftBoundingBox => {
                        let bbox = BoundingBox::from((
                            input,
                            Vec2(
                                args.width.expect("NormalizedCenterBoundingBox requires width argument") as f32,
                                args.height.expect("NormalizedCenterBoundingBox requires height argument") as f32
                            )
                        ));
                        let output = TopLeftBoundingBox::from(bbox);
                        println!("{}", output);
                    },
                    BoundingBoxen::CenterBoundingBox => {
                        let output = CenterBoundingBox::from((
                            input,
                            Vec2(
                                args.width.expect("NormalizedCenterBoundingBox requires width argument") as f32,
                                args.height.expect("NormalizedCenterBoundingBox requires height argument") as f32
                            )
                        ));
                        println!("{}", output);
                    },
                    BoundingBoxen::NormalizedCenterBoundingBox => {
                        println!("{}", input);
                    },
                }
            },
        }
    }

    Ok(())
}
