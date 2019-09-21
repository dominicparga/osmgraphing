use std::cmp::min;
use std::fmt;

use log::info;

#[derive(Debug)]
pub struct Bar<'a> {
    k: u32,
    max_k: u32,
    n: u32,
    levels: Vec<(&'a str, bool)>,
}
impl<'a> Bar<'a> {
    pub fn from(max_k: u32) -> Bar<'a> {
        let progress_levels = vec![
            ("Found routes: [>                   ]", false),
            ("Found routes: [=>                  ]", false),
            ("Found routes: [==>                 ]", false),
            ("Found routes: [===>                ]", false),
            ("Found routes: [====>               ]", false),
            ("Found routes: [=====>              ]", false),
            ("Found routes: [======>             ]", false),
            ("Found routes: [=======>            ]", false),
            ("Found routes: [========>           ]", false),
            ("Found routes: [=========>          ]", false),
            ("Found routes: [==========>         ]", false),
            ("Found routes: [===========>        ]", false),
            ("Found routes: [============>       ]", false),
            ("Found routes: [=============>      ]", false),
            ("Found routes: [==============>     ]", false),
            ("Found routes: [===============>    ]", false),
            ("Found routes: [================>   ]", false),
            ("Found routes: [=================>  ]", false),
            ("Found routes: [==================> ]", false),
            ("Found routes: [===================>]", false),
            ("Found routes: [====================]", false),
        ];
        Bar {
            k: 0,
            max_k,
            n: 0,
            levels: progress_levels,
        }
    }

    fn log_conditionally(&mut self, always: bool) -> &mut Self {
        let idx = {
            // (len - 1) because 0 % (or respectively 100 %)
            let len = (self.levels.len() - 1) as f32;
            let k = self.k as f32;
            let max_k = self.max_k as f32;

            min(len as usize, (k / max_k * len) as usize)
        };
        let (bar, already_logged) = self.levels[idx];

        if always || !already_logged {
            if self.k == 0 {
                info!("{} ({} will be valid)", bar, self.max_k)
            } else {
                info!("{} ({} of {} valid)", bar, self.k, self.n)
            }
            self.levels[idx].1 = true;
        }

        self
    }
    /// Doesn't log progress-level if already logged once
    pub fn try_log(&mut self) -> &mut Self {
        self.log_conditionally(false)
    }
    /// Does log progress-level, even if already logged once
    pub fn log(&mut self) -> &mut Self {
        self.log_conditionally(true)
    }

    pub fn k(&self) -> u32 {
        self.k
    }
    pub fn n(&self) -> u32 {
        self.n
    }

    pub fn inc_k(&mut self) -> &mut Self {
        self.k += 1;
        self
    }
    pub fn inc_n(&mut self) -> &mut Self {
        self.n += 1;
        self
    }
    pub fn update_k(&mut self, delta_k: u32) -> &mut Self {
        self.k += delta_k;
        self
    }
    pub fn update_n(&mut self, delta_n: u32) -> &mut Self {
        self.n += delta_n;
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
