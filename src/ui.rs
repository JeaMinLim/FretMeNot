use eframe::egui::{
    self, CentralPanel, Context, FontData, FontDefinitions, RichText, Ui, Color32, Label, FontId,
};
use eframe::{App, CreationContext};

#[derive(PartialEq)]
enum Tab {
    ShowFrets,
    Settings,
    Info,
}

pub struct FretMeNotApp {
    current_tab: Tab,
}

impl Default for FretMeNotApp {
    fn default() -> Self {
        Self {
            current_tab: Tab::ShowFrets,
        }
    }
}

impl App for FretMeNotApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // 제목
            ui.heading("🎸 FretMeNot 🎸");
            ui.label("기타자판을 외워봅시다!!");

            // 탭
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.current_tab == Tab::ShowFrets, "🎵 지판 확인")
                    .clicked()
                {
                    self.current_tab = Tab::ShowFrets;
                }

                if ui
                    .selectable_label(self.current_tab == Tab::Settings, "⚙ 설정")
                    .clicked()
                {
                    self.current_tab = Tab::Settings;
                }

                if ui
                    .selectable_label(self.current_tab == Tab::Info, "👍 정보")
                    .clicked()
                {
                    self.current_tab = Tab::Info;
                }
            });

            ui.separator();

            // 선택된 탭에 따라 내용 표시
            match self.current_tab {
                Tab::ShowFrets => tab_show_frets(ui),
                Tab::Settings => tab_settings(ui),
                Tab::Info => tab_info(ui),
            }
        });
    }
}

fn tab_show_frets(ui: &mut Ui) {
    use egui::ScrollArea;
    static mut START_FRET: i32 = 0;
    static mut END_FRET: i32 = 24;

    let string_name = ["1번줄", "2번줄", "3번줄", "4번줄", "5번줄", "6번줄"];
    let first_fret_notes = ["E", "B", "G", "D", "A", "E"];
    let max_fret = 24;

    unsafe {
        ui.horizontal(|ui| {
            ui.label("카포:");
            ui.add(egui::Slider::new(&mut START_FRET, 0..=END_FRET.min(max_fret)).text(""));
            ui.label("끝 프렛:");
            ui.add(egui::Slider::new(&mut END_FRET, START_FRET..=max_fret).text(""));
        });

        ui.separator();

        ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new("fretboard")
                .striped(true)
                .spacing([8.0, 8.0])
                .min_col_width(20.0)
                .min_row_height(20.0)
                .show(ui, |ui| {
                    // 헤더 (프렛 번호)
                    ui.label(" ");
                    for fret in 0..=max_fret {
                        ui.add(Label::new(
                            RichText::new(format!("{}", fret))
                                .font(FontId::proportional(18.0))
                                .color(Color32::from_rgb(255, 255, 255))
                                .strong(),
                        ));
                    }
                    ui.end_row();

                    // 줄별 프렛
                    for (i, string_number) in string_name.iter().enumerate() {
                        ui.add(Label::new(
                            RichText::new(*string_number)
                                .font(FontId::proportional(18.0))
                                .color(Color32::from_rgb(255, 255, 255))
                                .strong(),
                        ));

                        for fret in 0..=max_fret {
                            if fret < START_FRET || fret > END_FRET {
                                ui.label(""); // 칸은 표시하되 음은 없음
                            } else if fret == 0 {
                                ui.label(first_fret_notes[i]);
                            } else {
                                let fret_offset = fret - 1;
                                ui.label(calculate_note(first_fret_notes[i], fret_offset as usize));
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

fn calculate_note(start_note: &str, fret_offset: usize) -> String {
    let notes = ["E", "F", "F#", "G", "G#", "A", "A#", "B", "C", "C#", "D", "D#"];

    // 시작 음의 인덱스를 찾음
    let start_index = notes.iter().position(|&n| n == start_note).unwrap() + 1 ; 

    // 프렛 이동 후 음계 계산
    let note_index = (start_index + fret_offset) % notes.len();
    notes[note_index].to_string()
}

fn tab_settings(ui: &mut Ui) {
    ui.label("설정 탭입니다.");
    ui.label("Setting Tab.");
}

fn tab_info(ui: &mut Ui) {
    ui.label(RichText::new("👤 제작자: 잼니크"));
    ui.label(RichText::new("📧 이메일: gam1532@gmail.com"));
    ui.hyperlink_to("🔗 GitHub: https://github.com/JeaMinLim", "https://github.com/JeaMinLim");
}

// 폰트 설정 함수
pub fn setup_fonts(cc: &CreationContext) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "Regular".to_owned(),
       //FontData::from_owned(std::fs::read("fonts/NanumGothic-Regular.ttf").unwrap()),
       FontData::from_static(include_bytes!("../fonts/NanumGothic-Regular.ttf")),
    );
    fonts.font_data.insert(
        "Bold".to_owned(),
        //FontData::from_owned(std::fs::read("fonts/NanumGothic-Bold.ttf").unwrap()),
        FontData::from_static(include_bytes!("../fonts/NanumGothic-Bold.ttf")),
    );
    fonts.font_data.insert(
        "ExtraBold".to_owned(),
        //FontData::from_owned(std::fs::read("fonts/NanumGothic-ExtraBold.ttf").unwrap()),
        FontData::from_static(include_bytes!("../fonts/NanumGothic-ExtraBold.ttf")),
    );
    fonts.font_data.insert(
        "Emoji".to_owned(),
        //FontData::from_owned(std::fs::read("fonts/NotoEmoji-VariableFont_wght.ttf").unwrap()),
        FontData::from_static(include_bytes!("../fonts/NotoEmoji-VariableFont_wght.ttf")),
    );

    // Proportional에 Emoji -> Regular 순서로 등록
    //if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        //family.clear();
        //family.push("Emoji".to_owned());
        //family.push("Regular".to_owned());
    //}

    // 사용자 정의 굵기 등록
    //fonts
        //.families
        //.insert(FontFamily::Name("Bold".into()), vec!["Bold".to_owned()]);
    //fonts
        //.families
        //.insert(FontFamily::Name("ExtraBold".into()), vec!["ExtraBold".to_owned()]);

    fonts  
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Regular".to_owned());

    cc.egui_ctx.set_fonts(fonts);
    
}
