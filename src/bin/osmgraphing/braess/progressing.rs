use std::fmt;

use log::info;

#[derive(Debug)]
pub struct Bar<'a> {
    k: u32,
    n: u32,
    levels: Vec<(u32, &'a str)>,
}
impl<'a> Bar<'a> {
    pub fn from(max_n: u32) -> Bar<'a> {
        let progress_levels = vec![
            (max_n * 00 / 20, "Found routes: [>                   ]"),
            (max_n * 01 / 20, "Found routes: [=>                  ]"),
            (max_n * 02 / 20, "Found routes: [==>                 ]"),
            (max_n * 03 / 20, "Found routes: [===>                ]"),
            (max_n * 04 / 20, "Found routes: [====>               ]"),
            (max_n * 05 / 20, "Found routes: [=====>              ]"),
            (max_n * 06 / 20, "Found routes: [======>             ]"),
            (max_n * 07 / 20, "Found routes: [=======>            ]"),
            (max_n * 08 / 20, "Found routes: [========>           ]"),
            (max_n * 09 / 20, "Found routes: [=========>          ]"),
            (max_n * 10 / 20, "Found routes: [==========>         ]"),
            (max_n * 11 / 20, "Found routes: [===========>        ]"),
            (max_n * 12 / 20, "Found routes: [============>       ]"),
            (max_n * 13 / 20, "Found routes: [=============>      ]"),
            (max_n * 14 / 20, "Found routes: [==============>     ]"),
            (max_n * 15 / 20, "Found routes: [===============>    ]"),
            (max_n * 16 / 20, "Found routes: [================>   ]"),
            (max_n * 17 / 20, "Found routes: [=================>  ]"),
            (max_n * 18 / 20, "Found routes: [==================> ]"),
            (max_n * 19 / 20, "Found routes: [===================>]"),
            (max_n * 20 / 20, "Found routes: [====================]"),
        ];
        Bar {
            k: 0,
            n: 0,
            levels: progress_levels,
        }
    }

    pub fn log(&self) {
        for &(cap, bar) in &self.levels {
            if self.k == cap {
                info!("{} ({}/{}) valid", bar, self.k, self.n)
            }
        }
    }

    pub fn k(&self) -> u32 {
        self.k
    }

    pub fn n(&self) -> u32 {
        self.n
    }

    pub fn inc_k(&mut self) {
        self.k += 1
    }

    pub fn inc_n(&mut self) {
        self.n += 1
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
