use std::fmt;

use log::info;

#[derive(Debug)]
pub struct Bar<'a> {
    k: u32,
    n: u32,
    max_k: u32,
    levels: Vec<(u32, &'a str)>,
}
impl<'a> Bar<'a> {
    pub fn from(max_k: u32) -> Bar<'a> {
        let progress_levels = vec![
            (max_k * 00 / 20, "Found routes: [>                   ]"),
            (max_k * 01 / 20, "Found routes: [=>                  ]"),
            (max_k * 02 / 20, "Found routes: [==>                 ]"),
            (max_k * 03 / 20, "Found routes: [===>                ]"),
            (max_k * 04 / 20, "Found routes: [====>               ]"),
            (max_k * 05 / 20, "Found routes: [=====>              ]"),
            (max_k * 06 / 20, "Found routes: [======>             ]"),
            (max_k * 07 / 20, "Found routes: [=======>            ]"),
            (max_k * 08 / 20, "Found routes: [========>           ]"),
            (max_k * 09 / 20, "Found routes: [=========>          ]"),
            (max_k * 10 / 20, "Found routes: [==========>         ]"),
            (max_k * 11 / 20, "Found routes: [===========>        ]"),
            (max_k * 12 / 20, "Found routes: [============>       ]"),
            (max_k * 13 / 20, "Found routes: [=============>      ]"),
            (max_k * 14 / 20, "Found routes: [==============>     ]"),
            (max_k * 15 / 20, "Found routes: [===============>    ]"),
            (max_k * 16 / 20, "Found routes: [================>   ]"),
            (max_k * 17 / 20, "Found routes: [=================>  ]"),
            (max_k * 18 / 20, "Found routes: [==================> ]"),
            (max_k * 19 / 20, "Found routes: [===================>]"),
            (max_k * 20 / 20, "Found routes: [====================]"),
        ];
        Bar {
            k: 0,
            max_k,
            n: 0,
            levels: progress_levels,
        }
    }

    pub fn log(&self) -> &Self {
        for &(cap, bar) in &self.levels {
            if self.k == cap {
                if cap == 0 {
                    info!("{} ({} will be valid)", bar, self.max_k)
                } else {
                    info!("{} ({} of {} valid)", bar, self.k, self.n)
                }
            }
        }
        self
    }

    pub fn k(&self) -> u32 {
        self.k
    }

    pub fn n(&self) -> u32 {
        self.n
    }

    pub fn inc_k(&mut self) -> &Self {
        self.k += 1;
        self
    }

    pub fn inc_n(&mut self) -> &Self {
        self.n += 1;
        self
    }
}
impl<'a> fmt::Display for Bar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tried {}-times for {} valid src-dst-pairs.",
            self.n(),
            self.k(),
        )
    }
}
