//made by adf  29/12/2022
//#![windows_subsystem = "windows"]

use eframe::egui;
use hound::{SampleFormat, WavSpec, WavWriter};
use rodio::{Decoder, Source};
use std::fs;
use std::io::Cursor;

use crate::clicksgen::Generator;
use crate::hwid::hwid_check;
use crate::macro_parser::{parse_txt, parse_zbf};
use eframe::egui::Vec2;
use rodio::source::Buffered;
use std::path::Path;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct clicksbotgui {
    file_export_name: String,
    string_fps: String,
    framemacro_checked: bool,
    framemacro_name: String,
    softclicks_checked: bool,
    hardclicks_checked: bool,
    softclicks_number: f32,
    hardclicks_number: f32,
    selected: Enum,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
enum Enum {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
    Ninth = 8,
    Tenth = 9,
    Eleventh = 10,
    Twelfth = 11,
    Thirteenth = 12,
    Fourteenth = 13,
    Fifteenth = 14,
    Sixteenth = 15,
    Seventeenth = 16,
    Eighteenth = 17,
    Nineteenth = 18,
    Twentieth = 19,
    TwentyFirst = 20,
    TwentySecond = 21,
    TwentyThird = 22,
    TwentyFourth = 23,
    TwentyFifth = 24,
    TwentySixth = 26,
    TwentySeventh = 27,
    TwentyEighth = 28,
    TwentyNinth = 29,
    Thirtieth = 30,
}

impl Default for clicksbotgui {
    fn default() -> Self {
        Self {
            file_export_name: "Clicks".to_owned(),
            string_fps: "360".to_owned(),
            framemacro_checked: true,
            framemacro_name: "".to_owned(),
            softclicks_checked: true,
            hardclicks_checked: false,
            softclicks_number: 0.35,
            hardclicks_number: 5.0,
            selected: Enum::First,
        }
    }
}

impl eframe::App for clicksbotgui {
    #[link_section = ".code"]
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let clicksbotgui {
            file_export_name,
            string_fps,
            framemacro_checked,
            framemacro_name,
            softclicks_checked,
            hardclicks_checked,
            softclicks_number,
            hardclicks_number,
            selected,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_min_size(Vec2::new(350.0, 550.0));  // Set minimum size
                ui.set_max_size(Vec2::new(350.0, 550.0));  // Set maximum size

                ui.heading("adf's Clickbot");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("zBot Frame Replay: ");
                ui.add(egui::TextEdit::singleline(framemacro_name).desired_width(100.0));

                ui.label("TPS: ");
                ui.add(egui::TextEdit::singleline(string_fps).desired_width(100.0));
                
                ui.label("Export File: ");
                ui.add(egui::TextEdit::singleline(file_export_name).desired_width(100.0));
                ui.label(".wav");
            });

            let paths = fs::read_dir("./click_types").unwrap();
            let mut names: Vec<String> = vec![];
            for path in paths {
                let name = path.unwrap().file_name().into_string().unwrap();
                names.push(name);
            }

            let clicks_type = names.get(*selected as usize).unwrap();
            
            ui.horizontal(|ui| {
            ui.label("Clickpack: ");
            egui::ComboBox::from_label("")
                .selected_text(format!("{}", names.get(*selected as usize).unwrap()))
                .width(100.0)
                .show_ui(ui, |ui| {
                    for i in 0..names.len() {
                        match i {
                            0 => {
                                ui.selectable_value(selected, Enum::First, names.get(i).unwrap());
                            }
                            1 => {
                                ui.selectable_value(selected, Enum::Second, names.get(i).unwrap());
                            }
                            2 => {
                                ui.selectable_value(selected, Enum::Third, names.get(i).unwrap());
                            }
                            3 => {
                                ui.selectable_value(selected, Enum::Fourth, names.get(i).unwrap());
                            }
                            4 => {
                                ui.selectable_value(selected, Enum::Fifth, names.get(i).unwrap());
                            }
                            5 => {
                                ui.selectable_value(selected, Enum::Sixth, names.get(i).unwrap());
                            }
                            6 => {
                                ui.selectable_value(selected, Enum::Seventh, names.get(i).unwrap());
                            }
                            7 => {
                                ui.selectable_value(selected, Enum::Eighth, names.get(i).unwrap());
                            }
                            8 => {
                                ui.selectable_value(selected, Enum::Ninth, names.get(i).unwrap());
                            }
                            9 => {
                                ui.selectable_value(selected, Enum::Tenth, names.get(i).unwrap());
                            }
                            10 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Eleventh,
                                    names.get(i).unwrap(),
                                );
                            }
                            11 => {
                                ui.selectable_value(selected, Enum::Twelfth, names.get(i).unwrap());
                            }
                            12 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Thirteenth,
                                    names.get(i).unwrap(),
                                );
                            }
                            13 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Fourteenth,
                                    names.get(i).unwrap(),
                                );
                            }
                            14 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Fifteenth,
                                    names.get(i).unwrap(),
                                );
                            }
                            15 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Sixteenth,
                                    names.get(i).unwrap(),
                                );
                            }
                            16 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Seventeenth,
                                    names.get(i).unwrap(),
                                );
                            }
                            17 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Eighteenth,
                                    names.get(i).unwrap(),
                                );
                            }
                            18 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Nineteenth,
                                    names.get(i).unwrap(),
                                );
                            }
                            19 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Twentieth,
                                    names.get(i).unwrap(),
                                );
                            }
                            20 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentyFirst,
                                    names.get(i).unwrap(),
                                );
                            }
                            21 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentySecond,
                                    names.get(i).unwrap(),
                                );
                            }
                            22 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentyThird,
                                    names.get(i).unwrap(),
                                );
                            }
                            23 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentyFourth,
                                    names.get(i).unwrap(),
                                );
                            }
                            24 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentyFifth,
                                    names.get(i).unwrap(),
                                );
                            }
                            25 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentySixth,
                                    names.get(i).unwrap(),
                                );
                            }
                            26 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentySeventh,
                                    names.get(i).unwrap(),
                                );
                            }
                            27 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentyEighth,
                                    names.get(i).unwrap(),
                                );
                            }
                            28 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentyNinth,
                                    names.get(i).unwrap(),
                                );
                            }
                            29 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::Thirtieth,
                                    names.get(i).unwrap(),
                                );
                            }
                            30 => {
                                ui.selectable_value(
                                    selected,
                                    Enum::TwentySixth,
                                    names.get(i).unwrap(),
                                );
                            }
                            _ => todo!(),
                        }
                    }
                 });
             });

            //println!("{}", *selected as usize);
            ui.checkbox(softclicks_checked, "softClicks");
            ui.checkbox(hardclicks_checked, "hardClicks");
            ui.checkbox(framemacro_checked, "zBot Replay");

               ui.horizontal(|ui| {
                ui.label("softClicks");
                ui.add(egui::DragValue::new(softclicks_number).speed(0.1));
                ui.label("Time")
                
               });
                
               ui.horizontal(|ui| {
                ui.label("hardClicks");
                ui.add(egui::DragValue::new(hardclicks_number).speed(0.1));
                ui.label("Time")

               });

            if ui.button("Render").clicked() {
                hwid_check();

                let mut fps: u32 = string_fps.trim().parse().expect("Invalid FPS!");

                let mut clicks: Vec<u64> = Vec::new();
                let mut releases: Vec<u64> = Vec::new();

                if *framemacro_checked {
                    if Path::new(&(framemacro_name.to_owned() + ".zbf")).exists() {
                        let mut fps_parsable: f32 = 0.0;

                        parse_zbf(
                            &mut clicks,
                            &mut releases,
                            &mut fps_parsable,
                            framemacro_name.to_owned() + ".zbf",
                        );

                        fps = fps_parsable.round() as u32;
                        fps = fps / 2;

                        println!("fps: {}", fps);
                    }
                } else {
                    let clicks_path = "clicks.txt";
                    let releases_path = "releases.txt";

                    parse_txt(&mut clicks, &mut releases, clicks_path, releases_path);

                    fps = fps / 2;
                }

                /*let lister;
                if clicks.len() < releases.len() { lister = clicks.len() } else { lister = releases.len() }

                for i in 0..lister {
                    if releases.get(i) < clicks.get(i) {
                        releases.remove(i);
                    }
                }*/

                let silence_buffered = Decoder::new(Cursor::new(fs::read("silent.wav").unwrap()))
                    .unwrap()
                    .buffered();

                let click_buffered: Vec<_> =
                    fs::read_dir("click_types/".to_owned() + &clicks_type + "/clicks")
                        .unwrap()
                        .map(|x| {
                            Decoder::new(Cursor::new(fs::read(x.unwrap().path()).unwrap()))
                                .unwrap()
                                .buffered()
                        })
                        .collect();

                let release_buffered: Vec<_> =
                    fs::read_dir("click_types/".to_owned() + &clicks_type + "/releases")
                        .unwrap()
                        .map(|x| {
                            Decoder::new(Cursor::new(fs::read(x.unwrap().path()).unwrap()))
                                .unwrap()
                                .buffered()
                        })
                        .collect();

                let mut soft_click_buffered: Vec<Buffered<Decoder<Cursor<Vec<u8>>>>> = vec![];
                let mut soft_release_buffered: Vec<Buffered<Decoder<Cursor<Vec<u8>>>>> = vec![];
                let mut hard_click_buffered: Vec<Buffered<Decoder<Cursor<Vec<u8>>>>> = vec![];
                let mut hard_release_buffered: Vec<Buffered<Decoder<Cursor<Vec<u8>>>>> = vec![];

                if *softclicks_checked == true {
                    soft_click_buffered =
                        fs::read_dir("click_types/".to_owned() + &clicks_type + "/softClicks")
                            .unwrap()
                            .map(|x| {
                                Decoder::new(Cursor::new(fs::read(x.unwrap().path()).unwrap()))
                                    .unwrap()
                                    .buffered()
                            })
                            .collect();

                    soft_release_buffered =
                        fs::read_dir("click_types/".to_owned() + &clicks_type + "/softReleases")
                            .unwrap()
                            .map(|x| {
                                Decoder::new(Cursor::new(fs::read(x.unwrap().path()).unwrap()))
                                    .unwrap()
                                    .buffered()
                            })
                            .collect();
                }

                if *hardclicks_checked == true {
                    hard_click_buffered =
                        fs::read_dir("click_types/".to_owned() + &clicks_type + "/hardClicks")
                            .unwrap()
                            .map(|x| {
                                Decoder::new(Cursor::new(fs::read(x.unwrap().path()).unwrap()))
                                    .unwrap()
                                    .buffered()
                            })
                            .collect();

                    hard_release_buffered =
                        fs::read_dir("click_types/".to_owned() + &clicks_type + "/hardReleases")
                            .unwrap()
                            .map(|x| {
                                Decoder::new(Cursor::new(fs::read(x.unwrap().path()).unwrap()))
                                    .unwrap()
                                    .buffered()
                            })
                            .collect();
                }

                if *softclicks_checked == false {
                    soft_click_buffered = click_buffered.clone();
                    soft_release_buffered = release_buffered.clone();
                }

                if *hardclicks_checked == false {
                    hard_click_buffered = click_buffered.clone();
                    hard_release_buffered = release_buffered.clone();
                }

                let g = Generator::from_clicks_and_releases(
                    silence_buffered,
                    click_buffered,
                    release_buffered,
                    soft_click_buffered,
                    soft_release_buffered,
                    hard_click_buffered,
                    hard_release_buffered,
                    *softclicks_number,
                    *hardclicks_number,
                    &clicks,
                    &releases,
                    fps,
                );

                let mut writer = WavWriter::create(
                    file_export_name.to_owned() + ".wav",
                    WavSpec {
                        channels: g.channels(),
                        sample_rate: g.sample_rate(),
                        bits_per_sample: 16,
                        sample_format: SampleFormat::Int,
                    },
                )
                .unwrap();

                for sample in g {
                    writer.write_sample(sample).unwrap();
                }
            }

            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Made by adf, maintained by alexander.");
                    ui.hyperlink_to("Discord", "https://discord.gg/9RvQCjgEUf");
                    ui.hyperlink_to("GitHub", "https://github.com/adeferion/acb");
                });

            });

        });
    }
}